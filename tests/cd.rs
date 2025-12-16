mod common;

use common::{make_disk_image, make_boot_sector};

use rust_project::device::block_device::MemoryBlockDevice;
use rust_project::fs::fat::Fat;
use rust_project::fs::clusters::ClusterReader;
use rust_project::fs::directory::DirectoryReader;
use rust_project::fs::path::PathResolver;
use rust_project::fs::cd::{Cd, CdError};

#[test]
fn cd_into_directory() {
    let img = make_disk_image();
    let device = MemoryBlockDevice::new(&img);
    let boot = make_boot_sector();
    let fat = Fat::new(&device, &boot);
    let clusters = ClusterReader::new(&device, &boot, &fat);
    let dir = DirectoryReader::new(&clusters);
    let resolver = PathResolver::new(&boot, &dir);
    let cd = Cd::new(&resolver);

    let new_cwd = cd.cd("/DIR", 2).unwrap();
    assert_eq!(new_cwd, 3);
}

#[test]
fn cd_relative() {
    let img = make_disk_image();
    let device = MemoryBlockDevice::new(&img);
    let boot = make_boot_sector();
    let fat = Fat::new(&device, &boot);
    let clusters = ClusterReader::new(&device, &boot, &fat);
    let dir = DirectoryReader::new(&clusters);
    let resolver = PathResolver::new(&boot, &dir);
    let cd = Cd::new(&resolver);

    let cwd = cd.cd("/DIR", 2).unwrap();
    let new = cd.cd(".", cwd).unwrap();

    assert_eq!(new, cwd);
}

#[test]
fn cd_to_file_fails() {
    let img = make_disk_image();
    let device = MemoryBlockDevice::new(&img);
    let boot = make_boot_sector();
    let fat = Fat::new(&device, &boot);
    let clusters = ClusterReader::new(&device, &boot, &fat);
    let dir = DirectoryReader::new(&clusters);
    let resolver = PathResolver::new(&boot, &dir);
    let cd = Cd::new(&resolver);

    assert_eq!(
        cd.cd("/DIR/FILE.TXT", 2),
        Err(CdError::NotADirectory)
    );
}

#[test]
fn cd_not_found() {
    let img = make_disk_image();
    let device = MemoryBlockDevice::new(&img);
    let boot = make_boot_sector();
    let fat = Fat::new(&device, &boot);
    let clusters = ClusterReader::new(&device, &boot, &fat);
    let dir = DirectoryReader::new(&clusters);
    let resolver = PathResolver::new(&boot, &dir);
    let cd = Cd::new(&resolver);

    assert_eq!(
        cd.cd("/NOPE", 2),
        Err(CdError::NotFound)
    );
}
