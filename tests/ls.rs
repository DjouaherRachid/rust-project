mod common;

use common::{make_disk_image, make_boot_sector};

use rust_project::device::block_device::MemoryBlockDevice;
use rust_project::fs::fat::Fat;
use rust_project::fs::clusters::ClusterReader;
use rust_project::fs::directory::DirectoryReader;
use rust_project::fs::path::PathResolver;
use rust_project::fs::ls::Ls;

#[test]
fn ls_root() {
    let img = make_disk_image();
    let device = MemoryBlockDevice::new(&img);
    let boot = make_boot_sector();
    let fat = Fat::new(&device, &boot);
    let clusters = ClusterReader::new(&device, &boot, &fat);
    let dirs = DirectoryReader::new(&clusters);
    let resolver = PathResolver::new(&boot, &dirs);
    let ls = Ls::new(&resolver);

    let entries = ls.list(None, boot.root_cluster).unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].name, "DIR");
}

#[test]
fn ls_subdir() {
    let img = make_disk_image();
    let device = MemoryBlockDevice::new(&img);
    let boot = make_boot_sector();
    let fat = Fat::new(&device, &boot);
    let clusters = ClusterReader::new(&device, &boot, &fat);
    let dirs = DirectoryReader::new(&clusters);
    let resolver = PathResolver::new(&boot, &dirs);
    let ls = Ls::new(&resolver);

    let entries = ls.list(Some("/DIR"), boot.root_cluster).unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].name, "FILE.TXT");
}
