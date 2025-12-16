use alloc::vec::Vec;
use crate::fs::directory::DirectoryEntry;
use crate::fs::path::{PathResolver, PathError};
use crate::device::block_device::BlockDevice;

pub struct Ls<'a, D: BlockDevice> {
    resolver: &'a PathResolver<'a, D>,
}

impl<'a, D: BlockDevice> Ls<'a, D> {
    pub fn new(resolver: &'a PathResolver<'a, D>) -> Self {
        Self { resolver }
    }

    pub fn list(
        &self,
        path: Option<&str>,
        cwd: u32,
    ) -> Result<Vec<DirectoryEntry>, PathError> {
        let (cluster, entry) = match path {
            Some(p) => self.resolver.resolve(p, cwd)?,
            None => (cwd, None),
        };

        if let Some(e) = entry {
            if !e.entry_type.is_dir() {
                return Err(PathError::NotADirectory);
            }
        }

        self.resolver.read_dir(cluster)
    }
}
