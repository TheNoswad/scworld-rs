use std::fs::File;
use std::io::prelude::*;
use std::os::unix::prelude::FileExt;

// fn main() {
//     let mut file = File::open("./src/Chunks32h.dat");
//     let mut raw_dictionary: [u8; 786432] = [0; 786432];
//     file.unwrap().read_exact_at(&mut raw_dictionary, 0).unwrap();

//     let directory = Directory::deserialize_from_bytes(raw_dictionary);
//     dbg!(directory);

//     println!("Reading into struct");
// }

const MAGIC1: u32 = 0xDEADBEEF;
const MAGIC2: u32 = 0xFFFFFFFF;

pub struct World {
    pub directory: [DirectoryEntry; 65535],
    pub chunks: Vec<Chunk>,
}

#[derive(Clone, Copy, Debug)]
pub struct DirectoryEntry {
    pub chunkx: i32,
    pub chunkz: i32,
    pub index: i32,
}

impl DirectoryEntry {
    pub fn new() -> DirectoryEntry {
        DirectoryEntry {
            chunkx: 0,
            chunkz: 0,
            index: -255,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Directory {
    pub entries: [DirectoryEntry; 65536],
}

impl Directory {
    pub fn deserialize_from_bytes(buf: [u8; 786432]) -> Directory {
        let mut directory = Directory::new();
        for i in (0..buf.iter().len()).step_by(12) {
            //println!("i is {}", i);
            let mut xbuf: [u8; 4] = [0, 0, 0, 0];
            xbuf.clone_from_slice(&buf[i..i+4]);
            let x = i32::from_le_bytes(xbuf);

            let mut ybuf: [u8; 4] = [0, 0, 0, 0];
            ybuf.clone_from_slice(&buf[(4 + i)..(i + 8)]);
            let y = i32::from_le_bytes(ybuf);

            let mut indexbuf: [u8; 4] = [0, 0, 0, 0];
            indexbuf.clone_from_slice(&buf[(8 + i)..(i + 12)]);
            let index = i32::from_be_bytes(indexbuf);

            let directoryentry = DirectoryEntry {
                chunkx: x,
                chunkz: y,
                index: index,
            };
            directory.entries[i/12] = directoryentry;
        }
        return directory
    }

    pub fn new() -> Directory {
        Directory {
            entries: [DirectoryEntry::new(); 65536],
        }
    }
}

pub struct Chunk {
    chunkheader: ChunkHeader,
    blocks: [Block; 65536],
    surfacepoints: [SurfacePoint; 256],
}

pub struct SurfacePoint {
    maxheight: u8,
    temphumidity: u8,
    unused1: u8,
    unused2: u8,
}

pub struct ChunkHeader {
    magic1: u32,
    magic2: u32,
    chunkx: i32,
    chunkz: i32,
}

pub struct Block {
    blocktype: u32,
    blockdata: u8,
}
