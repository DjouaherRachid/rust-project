mod common;

use rust_project::device::block_device::MemoryBlockDevice;
use rust_project::fs::fat::Fat;
use rust_project::fs::clusters::ClusterReader;
use rust_project::fs::directory::{DirectoryReader, EntryType};
use common::{make_disk_image, make_boot_sector};

#[test]
fn read_directory_entries() {
    let img = make_disk_image();
    let device = MemoryBlockDevice::new(&img);
    let boot = make_boot_sector();
    let fat = Fat::new(&device, &boot);
    let cluster_reader = ClusterReader::new(&device, &boot, &fat);
    let dir_reader = DirectoryReader::new(&cluster_reader);

    // Lire root cluster
    let entries = dir_reader.read_dir(2).unwrap();
    assert_eq!(entries.len(), 1); // seul DIR dans root
    assert_eq!(entries[0].name, "DIR");
    assert_eq!(entries[0].entry_type, EntryType::Directory);

    // Lire DIR cluster
    let entries_dir = dir_reader.read_dir(3).unwrap();
    assert_eq!(entries_dir.len(), 1); // FILE.TXT
    assert_eq!(entries_dir[0].name, "FILE.TXT");
    assert_eq!(entries_dir[0].entry_type, EntryType::File);
    assert_eq!(entries_dir[0].size, 123);
}
