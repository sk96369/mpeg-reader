use std::{fs::File, io};
use bitvec::prelude::*;
use mpeg_file_structure::*;

const SYNC_LEN: usize = 11;

type Bits = BitVec<u8, Lsb0>;

mod mpeg_file_structure {
    use std::fmt::Display;

    use super::*;

    pub struct Frame {
        header: Header,
        data: Vec<u8>,
    }

    //Formula for calculating frame length in bytes:
    //FrameLen = ((144 * BitRate / SampleRate ) + Padding) as usize

    pub struct Header {
        data: Bits,
    }

    pub trait MpegHeader {
        fn get_syncword(&self) -> String;
        fn get_version_id(&self) -> &u8;
        fn get_layer(&self) -> &u8;
        fn get_crcp(&self) -> bool;
        fn get_bitrate(&self) -> &u16;
        fn get_sampling_rate(&self) -> &u16;
        fn get_padding(&self) -> bool;
        fn get_private_bit(&self) -> bool;
        fn get_channel(&self) -> &u8;
        fn get_mode_extension(&self) -> &u8;
        fn get_copyright(&self) -> bool;
        fn get_orig(&self) -> bool;
        fn get_emphasis(&self) -> &u8;
    }

    impl MpegHeader for Frame {
        fn get_syncword(&self) -> String {
            self.header.get_syncword()
        }

        fn get_version_id(&self) -> &u8 {
            self.header.get_version_id()
        }

        fn get_layer(&self) -> &u8 {
            self.header.get_layer()
        }

        fn get_crcp(&self) -> bool {
            self.header.get_crcp()
        }

        fn get_bitrate(&self) -> &u16 {
            self.header.get_bitrate()
        }

        fn get_sampling_rate(&self) -> &u16 {
            self.header.get_sampling_rate()
        }

        fn get_padding(&self) -> bool {
            self.header.get_padding()
        }

        fn get_private_bit(&self) -> bool {
            self.header.get_private_bit()
        }
        fn get_channel(&self) -> &u8 {
            self.header.get_channel()
        }

        fn get_mode_extension(&self) -> &u8 {
            self.header.get_mode_extension()
        }

        fn get_copyright(&self) -> bool {
            self.header.get_copyright()
        }

        fn get_orig(&self) -> bool {
            self.header.get_orig()
        }

        fn get_emphasis(&self) -> &u8 {
            self.header.get_emphasis()
        }
    }

    impl MpegHeader for Header {
        fn get_syncword(&self) -> String {
            self.data[0..11].to_string()
        }

        fn get_version_id(&self) -> &u8 {
            &self.data[11..13].load()
        }

        fn get_layer(&self) -> &u8 {
            &self.data[13..15].load()
        }

        fn get_crcp(&self) -> bool {
            self.data[15] == true
        }

        fn get_bitrate(&self) -> &u16 {
            &self.data[16..21].load()
        }

        fn get_sampling_rate(&self) -> &u16 {
            &self.data[21..23].load()
        }

        fn get_padding(&self) -> bool {
            self.data[23] == true
        }

        fn get_private_bit(&self) -> bool {
            self.data[24] == true
        }

        fn get_channel(&self) -> &u8 {
            &self.data[25..27].load()
        }

        fn get_mode_extension(&self) -> &u8 {
            &self.data[27..29].load()
        }

        fn get_copyright(&self) -> bool {
            self.data[29] == true
        }

        fn get_orig(&self) -> bool {
            self.data[30] == true
        }

        fn get_emphasis(&self) -> &u8 {
            &self.data[31..32].load()
        }
    }

    impl Display for Header {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
                   self.get















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
