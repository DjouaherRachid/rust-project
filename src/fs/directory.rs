//! Parsing des entrées de répertoire FAT32 (8.3 uniquement)

use crate::fs::clusters::{ClusterError, ClusterReader};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectoryError {
    Cluster(ClusterError),
    InvalidEntry,
}

impl EntryType {
    pub fn is_dir(&self) -> bool {
        matches!(self, EntryType::Directory)
    }
}

impl From<ClusterError> for DirectoryError {
    fn from(e: ClusterError) -> Self {
        DirectoryError::Cluster(e)
    }
}

/// Type d’entrée de répertoire
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryType {
    File,
    Directory,
}

/// Entrée de répertoire FAT32 (simplifiée)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryEntry {
    pub name: alloc::string::String,
    pub entry_type: EntryType,
    pub start_cluster: u32,
    pub size: u32,
}

/// Lecteur de répertoire
pub struct DirectoryReader<'a, D: crate::device::block_device::BlockDevice> {
    cluster_reader: &'a ClusterReader<'a, D>,
}

impl<'a, D: crate::device::block_device::BlockDevice> DirectoryReader<'a, D> {
    pub fn new(cluster_reader: &'a ClusterReader<'a, D>) -> Self {
        Self { cluster_reader }
    }

    /// Lit toutes les entrées d’un répertoire à partir du cluster donné.
    pub fn read_dir(
        &self,
        start_cluster: u32,
    ) -> Result<alloc::vec::Vec<DirectoryEntry>, DirectoryError> {
        let mut data = alloc::vec::Vec::new();
        self.cluster_reader
            .read_cluster_chain(start_cluster, &mut data)?;

        let mut entries = alloc::vec::Vec::new();

        for chunk in data.chunks_exact(32) {
            let first = chunk[0];

            if first == 0x00 {
                break; // fin du répertoire
            }

            if first == 0xE5 {
                continue; // supprimé
            }

            let attr = chunk[11];
            if attr == 0x0F {
                continue; // LFN ignoré
            }

            let name = parse_short_name(&chunk[0..11])?;

            let is_dir = attr & 0x10 != 0;

            let high =
                u16::from_le_bytes([chunk[20], chunk[21]]) as u32;
            let low =
                u16::from_le_bytes([chunk[26], chunk[27]]) as u32;
            let start_cluster = (high << 16) | low;

            let size =
                u32::from_le_bytes([chunk[28], chunk[29], chunk[30], chunk[31]]);

            entries.push(DirectoryEntry {
                name,
                entry_type: if is_dir {
                    EntryType::Directory
                } else {
                    EntryType::File
                },
                start_cluster,
                size,
            });
        }

        Ok(entries)
    }
}

/// Parse un nom 8.3
fn parse_short_name(raw: &[u8]) -> Result<alloc::string::String, DirectoryError> {
    let name = core::str::from_utf8(&raw[0..8]).map_err(|_| DirectoryError::InvalidEntry)?;
    let ext = core::str::from_utf8(&raw[8..11]).map_err(|_| DirectoryError::InvalidEntry)?;

    let name = name.trim_end();
    let ext = ext.trim_end();

    let full = if ext.is_empty() {
        alloc::format!("{}", name)
    } else {
        alloc::format!("{}.{}", name, ext)
    };

    Ok(full)
}
