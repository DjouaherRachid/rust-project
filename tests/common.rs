use rust_project::fs::boot_sector::BootSector;

/// Construction d’un BootSector FAT32 simulé
pub fn make_boot_sector() -> BootSector {
    BootSector {
        bytes_per_sector: 512,
        sectors_per_cluster: 1,
        reserved_sectors: 1,
        fat_count: 1,
        sectors_per_fat: 1,
        root_cluster: 2,
    }
}

/// Construire une image mémoire avec :
///
/// Cluster 2 : racine → contient DIR  
/// Cluster 3 : DIR → contient FILE.TXT  
/// Cluster 4 : FILE.TXT données
pub fn make_disk_image() -> Vec<u8> {
    let mut img = vec![0u8; 512]; // Reserved

    // FAT simulée (1 FAT, 512 octets suffisent pour nos 5 clusters)
    let mut fat = vec![0u8; 512];
    // cluster 2 → EOC (root)
    fat[2*4..2*4+4].copy_from_slice(&0x0FFF_FFFFu32.to_le_bytes());
    // cluster 3 → cluster 4 (DIR → FILE.TXT)
    fat[3*4..3*4+4].copy_from_slice(&4u32.to_le_bytes());
    // cluster 4 → EOC (FILE.TXT)
    fat[4*4..4*4+4].copy_from_slice(&0x0FFF_FFFFu32.to_le_bytes());
    img.extend_from_slice(&fat);

    // Cluster 2 : root
    let mut root = vec![0u8; 512];
    let dir = make_dir_entry("DIR", "", 0x10, 3, 0); // dossier DIR → cluster 3
    root[0..32].copy_from_slice(&dir);
    root[32] = 0x00;
    img.extend_from_slice(&root);

    // Cluster 3 : DIR
    let mut cluster3 = vec![0u8; 512];
    let file = make_dir_entry("FILE", "TXT", 0x20, 4, 123); // FILE.TXT → cluster 4
    cluster3[0..32].copy_from_slice(&file);
    cluster3[32] = 0x00;
    img.extend_from_slice(&cluster3);

    // Cluster 4 : données du fichier (rempli de 0)
    img.extend_from_slice(&vec![0u8; 512]);

    img
}

/// Helper pour créer une entrée FAT32 (8.3)
fn make_dir_entry(name: &str, ext: &str, attr: u8, start_cluster: u32, size: u32) -> [u8; 32] {
    let mut e = [0u8; 32];

    // Nom (8 octets)
    let mut n = [b' '; 8];
    n[..name.len()].copy_from_slice(name.as_bytes());
    e[0..8].copy_from_slice(&n);

    // Extension (3 octets)
    let mut x = [b' '; 3];
    x[..ext.len()].copy_from_slice(ext.as_bytes());
    e[8..11].copy_from_slice(&x);

    e[11] = attr;

    // cluster bas et haut
    e[20..22].copy_from_slice(&((start_cluster >> 16) as u16).to_le_bytes());
    e[26..28].copy_from_slice(&(start_cluster as u16).to_le_bytes());

    // taille
    e[28..32].copy_from_slice(&size.to_le_bytes());

    e
}
