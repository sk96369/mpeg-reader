use std::{fs::File, io};
use bitvec::prelude::*;

const SYNC_LEN: usize = 11;

type Bits = BitVec<u8, Msb0>;

fn main() {
    use mpeg_file_structure::*;

    let path = "test.mp3";
    let mpeg_1 = MPEG::open(path);

    let path = "huh.mp3";
    let mpeg_2 = MPEG::open(path);

    println!("test.mp3:\n{}\n", mpeg_1);
    println!("huh.mp3:\n{}", mpeg_2);
}
