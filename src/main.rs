use std::{fs::File, io};
use bitvec::prelude::*;

const SYNC_LEN: usize = 11;

type Bits = BitVec<u8, Lsb0>;

struct Frame {
    header: Vec<u8>,
    data: Vec<u8>,
    datasize: usize,
}

//Formula for calculating frame length in bytes:
//FrameLen = ((144 * BitRate / SampleRate ) + Padding) as usize

struct Header {
    data: Bits,
}


fn read_first_frame(path: &str) -> Header {
    let mut contents: Bits = BitVec::new();
    let syncword: Bits = BitVec::repeat(true, 11);
    io::copy(&mut File::open(path).unwrap(), &mut contents).expect("Assuming io::copy is all gucci");
    let mut header_data: Bits = BitVec::new();
    contents.windows(32)
        .skip_while(|x| x[..11] != syncword)
        .next()
        .unwrap()
        .clone_into(&mut header_data);
    Header { data: header_data }
}

fn read_file(path: &str) {
    let mut contents: Bits = BitVec::new();
    let mut syncword: Bits = BitVec::new();
    io::copy(&mut File::open(path).unwrap(), &mut contents).expect("Assuming io::copy is all gucci");
    let mut lengths: Vec<usize> = Vec::new();
    let mut counter = SYNC_LEN;
    let mut filesize = 3;
    contents.iter()
        .take(SYNC_LEN)
        .for_each(|x| syncword.push(*x));

    contents.windows(SYNC_LEN)
        .step_by(4)
        .for_each(|x| {
            counter += 4;
            filesize += 1;
            if counter > 32 && x == syncword {
                lengths.push(counter);
                counter = SYNC_LEN;
            }
        });
    println!("{:#?}\n", lengths);
    println!("file size: {} bytes", filesize / 2);
}

fn main() {
    let path = "test.mp3";
    let frame = read_first_frame(path);
    println!("{:?}", frame.data);
}
