//! Lecture et interprétation de la FAT32 (File Allocation Table)

use crate::device::block_device::{BlockDevice, BlockDeviceError};
use crate::fs::boot_sector::BootSector;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FatError {
    Io(BlockDeviceError),
    InvalidCluster,
}

impl From<BlockDeviceError> for FatError {
    fn from(e: BlockDeviceError) -> Self {
        FatError::Io(e)
    }
}

/// Représente une FAT32 lisible
pub struct Fat<'a, D: BlockDevice> {
    device: &'a D,
    boot: &'a BootSector,
}

impl<'a, D: BlockDevice> Fat<'a, D> {
    pub fn new(device: &'a D, boot: &'a BootSector) -> Self {
        Self { device, boot }
    }

    /// Retourne le cluster suivant dans la chaîne.
    ///
    /// - `None` → fin de chaîne (EOC)
    /// - `Some(cluster)` → cluster suivant
    pub fn next_cluster(&self, cluster: u32) -> Result<Option<u32>, FatError> {
        if cluster < 2 {
            return Err(FatError::InvalidCluster);
        }

        let fat_offset = cluster as u64 * 4;
        let fat_start = self.boot.reserved_sectors as u64
            * self.boot.bytes_per_sector as u64;

        let offset = fat_start + fat_offset;

        let mut entry = [0u8; 4];
        self.device.read_at(offset, &mut entry)?;

        let value = u32::from_le_bytes(entry) & 0x0FFF_FFFF;

        match value {
            0x0000_0000 => Err(FatError::InvalidCluster),
            0x0FFF_FFF8..=0x0FFF_FFFF => Ok(None), // End Of Chain
            next => Ok(Some(next)),
        }
    }
}
