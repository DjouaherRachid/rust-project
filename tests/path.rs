mod common;

use common::{make_disk_image, make_boot_sector};

use rust_project::device::block_device::MemoryBlockDevice;
use rust_project::fs::fat::Fat;
use rust_project::fs::clusters::ClusterReader;
use rust_project::fs::directory::{DirectoryReader, EntryType};
use rust_project::fs::path::{PathResolver, PathError};

#[test]
fn resolve_absolute_path() {
    let img = make_disk_image();
    let device = MemoryBlockDevice::new(&img);
    let boot = make_boot_sector();
    let fat = Fat::new(&device, &boot);
    let clusters = ClusterReader::new(&device, &boot, &fat);
    let dirs = DirectoryReader::new(&clusters);
    let resolver = PathResolver::new(&boot, &dirs);

    let (cluster, entry) = resolver.resolve("/DIR", 2).unwrap();

    assert_eq!(cluster, 3);
    assert_eq!(entry.unwrap().entry_type, EntryType::Directory);
}

#[test]
fn resolve_file_path() {
    let img = make_disk_image();
    let device = MemoryBlockDevice::new(&img);
    let boot = make_boot_sector();
    let fat = Fat::new(&device, &boot);
    let clusters = ClusterReader::new(&device, &boot, &fat);
    let dirs = DirectoryReader::new(&clusters);
    let resolver = PathResolver::new(&boot, &dirs);

    let (_, entry) = resolver.resolve("/DIR/FILE.TXT", 2).unwrap();
    assert_eq!(entry.unwrap().entry_type, EntryType::File);
}

#[test]
fn path_not_found() {
    let img = make_disk_image();
    let device = MemoryBlockDevice::new(&img);
    let boot = make_boot_sector();
    let fat = Fat::new(&device, &boot);
    let clusters = ClusterReader::new(&device, &boot, &fat);
    let dirs = DirectoryReader::new(&clusters);
    let resolver = PathResolver::new(&boot, &dirs);

    assert_eq!(
        resolver.resolve("/NOPE", 2),
        Err(PathError::NotFound)
    );
}
