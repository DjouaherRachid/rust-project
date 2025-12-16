mod common;

use common::{make_disk_image, make_boot_sector};

use rust_project::device::block_device::MemoryBlockDevice;
use rust_project::fs::fat::Fat;
use rust_project::fs::clusters::ClusterReader;
use rust_project::fs::directory::DirectoryReader;
use rust_project::fs::path::PathResolver;
use rust_project::fs::cat::{Cat, CatError};

#[test]
fn cat_reads_file() {
    let img = make_disk_image();
    let device = MemoryBlockDevice::new(&img);
    let boot = make_boot_sector();
    let fat = Fat::new(&device, &boot);
    let clusters = ClusterReader::new(&device, &boot, &fat);
    let dir = DirectoryReader::new(&clusters);
    let resolver = PathResolver::new(&boot, &dir);
    let cat = Cat::new(&resolver, &clusters);

    let data = cat.cat("/DIR/FILE.TXT", 2).unwrap();

    // Le fichier fait 123 octets, remplis de 0
    assert_eq!(data.len(), 123);
    assert!(data.iter().all(|b| *b == 0));
}

#[test]
fn cat_not_found() {
    let img = make_disk_image();
    let device = MemoryBlockDevice::new(&img);
    let boot = make_boot_sector();
    let fat = Fat::new(&device, &boot);
    let clusters = ClusterReader::new(&device, &boot, &fat);
    let dir = DirectoryReader::new(&clusters);
    let resolver = PathResolver::new(&boot, &dir);
    let cat = Cat::new(&resolver, &clusters);

    assert_eq!(
        cat.cat("/DIR/NOPE.TXT", 2),
        Err(CatError::NotFound)
    );
}

#[test]
fn cat_on_directory_fails() {
    let img = make_disk_image();
    let device = MemoryBlockDevice::new(&img);
    let boot = make_boot_sector();
    let fat = Fat::new(&device, &boot);
    let clusters = ClusterReader::new(&device, &boot, &fat);
    let dir = DirectoryReader::new(&clusters);
    let resolver = PathResolver::new(&boot, &dir);
    let cat = Cat::new(&resolver, &clusters);

    assert_eq!(
        cat.cat("/DIR", 2),
        Err(CatError::NotAFile)
    );
}
