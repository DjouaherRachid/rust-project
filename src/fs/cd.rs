use crate::device::block_device::BlockDevice;
use crate::fs::path::{PathResolver, PathError};
use crate::fs::directory::EntryType;

#[derive(Debug, PartialEq, Eq)]
pub enum CdError {
    NotFound,
    NotADirectory,
    Io,
}

pub struct Cd<'a, D: BlockDevice> {
    resolver: &'a PathResolver<'a, D>,
}

impl<'a, D: BlockDevice> Cd<'a, D> {
    pub fn new(resolver: &'a PathResolver<'a, D>) -> Self {
        Self { resolver }
    }

    pub fn cd(&self, path: &str, cwd: u32) -> Result<u32, CdError> {
        let (cluster, entry) =
            self.resolver.resolve(path, cwd).map_err(|e| match e {
                PathError::NotFound => CdError::NotFound,
                _ => CdError::Io,
            })?;

        match entry {
            None => Ok(cluster), // racine
            Some(e) => {
                if e.entry_type != EntryType::Directory {
                    Err(CdError::NotADirectory)
                } else {
                    Ok(e.start_cluster)
                }
            }
        }
    }
}
