use std::{fs::File, os::unix::prelude::FileExt};

use chunks32h::*;

extern crate chunks32h;

/// Lists all directory entries in stdout
fn main() {
    // Open the chunks file
    let file = File::open("./src/Chunks32h.dat");
    
    // Create a buffer to store the dictionary
    let mut raw_dictionary: [u8; 786432] = [0; 786432];

    // Read the dictionary data to a file
    // Should probably be moved into the library
    file.unwrap().read_exact_at(&mut raw_dictionary, 0).unwrap();

    // Create a directory from the bytes
    let directory = Directory::deserialize_from_bytes(raw_dictionary);

    for entry in directory.entries.into_iter() {
        if entry.index != -1 {
            dbg!(entry);
        }
    }
}