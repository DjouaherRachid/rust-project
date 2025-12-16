use alloc::vec::Vec;

use crate::device::block_device::BlockDevice;
use crate::fs::clusters::ClusterReader;
use crate::fs::path::{PathResolver, PathError};
use crate::fs::directory::EntryType;

#[derive(Debug, PartialEq, Eq)]
pub enum CatError {
    NotFound,
    NotAFile,
    Io,
}

pub struct Cat<'a, D: BlockDevice> {
    resolver: &'a PathResolver<'a, D>,
    cluster_reader: &'a ClusterReader<'a, D>,
}

impl<'a, D: BlockDevice> Cat<'a, D> {
    pub fn new(
        resolver: &'a PathResolver<'a, D>,
        cluster_reader: &'a ClusterReader<'a, D>,
    ) -> Self {
        Self {
            resolver,
            cluster_reader,
        }
    }

    pub fn cat(&self, path: &str, cwd: u32) -> Result<Vec<u8>, CatError> {
        let (_parent, entry_opt) =
            self.resolver.resolve(path, cwd).map_err(|e| match e {
                PathError::NotFound => CatError::NotFound,
                _ => CatError::Io,
            })?;

        let entry = entry_opt.ok_or(CatError::NotFound)?;

        if entry.entry_type != EntryType::File {
            return Err(CatError::NotAFile);
        }

        let mut data = Vec::new();

        self.cluster_reader
            .read_cluster_chain(entry.start_cluster, &mut data)
            .map_err(|_| CatError::Io)?;

        // Respect strict de la taille FAT
        data.truncate(entry.size as usize);

        Ok(data)
    }
}
