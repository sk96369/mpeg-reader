use std::{fs::File, io};
use bitvec::prelude::*;
use mpeg_file_structure::*;

const SYNC_LEN: usize = 11;

type Bits = BitVec<u8, Msb0>;

mod mpeg_file_structure {
    use std::fmt::{self, Display};

    use super::*;

    pub struct Frame {
        header: Header,
        data: Vec<u8>,
    }

    //Formula for calculating frame length in bytes:
    //FrameLen = ((144 * BitRate / SampleRate ) + Padding) as usize

    pub struct Header {
        pub data: Bits,
    }

    impl From<Bits> for Header {
        fn from(header_data: Bits) -> Self {
            Header { data: header_data }
        }
    }

    impl Header {
        fn get_frame_len(&self) -> usize {
            ((144 * self.get_bitrate() as usize) / self.get_sampling_rate() as usize) + self.get_padding() as usize
        }
    }

    pub trait MpegHeader {
        fn get_syncword(&self) -> String;
        fn get_version_id(&self) -> u8;
        fn get_layer(&self) -> u8;
        fn get_crcp(&self) -> bool;
        fn get_bitrate(&self) -> u16;
        fn get_sampling_rate(&self) -> u16;
        fn get_padding(&self) -> bool;
        fn get_private_bit(&self) -> bool;
        fn get_channel(&self) -> u8;
        fn get_mode_extension(&self) -> u8;
        fn get_copyright(&self) -> bool;
        fn get_orig(&self) -> bool;
        fn get_emphasis(&self) -> u8;
    }

    impl MpegHeader for Frame {
        fn get_syncword(&self) -> String {
            self.header.get_syncword()
        }

        fn get_version_id(&self) -> u8 {
            self.header.get_version_id()
        }

        fn get_layer(&self) -> u8 {
            self.header.get_layer()
        }

        fn get_crcp(&self) -> bool {
            self.header.get_crcp()
        }

        fn get_bitrate(&self) -> u16 {
            self.header.get_bitrate()
        }

        fn get_sampling_rate(&self) -> u16 {
            self.header.get_sampling_rate()
        }

        fn get_padding(&self) -> bool {
            self.header.get_padding()
        }

        fn get_private_bit(&self) -> bool {
            self.header.get_private_bit()
        }
        fn get_channel(&self) -> u8 {
            self.header.get_channel()
        }

        fn get_mode_extension(&self) -> u8 {
            self.header.get_mode_extension()
        }

        fn get_copyright(&self) -> bool {
            self.header.get_copyright()
        }

        fn get_orig(&self) -> bool {
            self.header.get_orig()
        }

        fn get_emphasis(&self) -> u8 {
            self.header.get_emphasis()
        }
    }

    impl MpegHeader for Header {
        fn get_syncword(&self) -> String {
            self.data[0..11].to_string()
        }

        fn get_version_id(&self) -> u8 {
            self.data[11..13].load()
        }

        fn get_layer(&self) -> u8 {
            self.data[13..15].load()
        }

        fn get_crcp(&self) -> bool {
            self.data[15] == true
        }

        fn get_bitrate(&self) -> u16 {
            self.data[16..20].load()
        }

        fn get_sampling_rate(&self) -> u16 {
            self.data[20..22].load()
        }

        fn get_padding(&self) -> bool {
            self.data[22] == true
        }

        fn get_private_bit(&self) -> bool {
            self.data[23] == true
        }

        fn get_channel(&self) -> u8 {
            self.data[22..26].load()
        }

        fn get_mode_extension(&self) -> u8 {
            self.data[26..28].load()
        }

        fn get_copyright(&self) -> bool {
            self.data[28] == true
        }

        fn get_orig(&self) -> bool {
            self.data[29] == true
        }

        fn get_emphasis(&self) -> u8 {
            self.data[30..32].load()
        }
    }

    impl Display for Header {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "MPEG version: {} layer {}\nCRCP protection: {}\nBitrate: {} kbps\nSampling rate: {} Hz\nFrame padding: {}\nPrivate bit: {}\nChannel: {}\nCopyrighted: {}\nOriginal media: {}\nEmphasis: {}",
                   match self.get_version_id() {
                       0 => "2.5",
                       1 => "error",
                       2 => "2",
                       _ => "1",
                   },
                   match self.get_layer() {
                       0 => "error",
                       1 => "III",
                       2 => "II",
                       _ => "I",
                   },
                   match self.get_crcp() {
                       false => "Yes",
                       true => "No ",
                   },
                   match self.get_bitrate() {
                       0 => "free",
                       1 => "32",
                       2 => "40",
                       3 => "48",
                       4 => "56",
                       5 => "64",
                       6 => "80",
                       7 => "96",
                       8 => "112",
                       9 => "128",
                       10 => "160",
                       11 => "192",
                       12 => "224",
                       13 => "256",
                       14 => "320",
                       _ => {
                           println!("{}", self.get_bitrate());
                           "brokey"
                       },
                   },
                   match self.get_sampling_rate() {
                       0 => "44100",
                       1 => "48000",
                       2 => "32000",
                       _ => "error",
                   },
                   match self.get_padding() {
                       true => "yes",
                       false => "no",
                   },
                   self.get_private_bit(),
                   match self.get_channel() {
                       0 => "Stereo".to_string(),
                       1 => format!("Joint stereo: {}", match self.get_mode_extension() {
                            0 => "No intensity stereo or MS stereo",
                            1 => "Intensity stereo on, MS stereo off",
                            2 => "Intensity stereo off, MS stereo off",
                            _ => "Intensity stereo on, MS stereo on",
                       }),
                       2 => "Dual".to_string(),
                       _ => "Mono".to_string(),
                   },
                   match self.get_copyright() {
                       true => "yes",
                       false => "no",
                   },
                   match self.get_orig() {
                       true => "yes",
                       false => "no",
                   },
                   match self.get_emphasis() {
                       0 => "None",
                       1 => "50/15",
                       2 => "error",
                       _ => "CCIT J.17",
                   })
        }
    }
}

fn read_first_frame(path: &str) -> Header {
    let mut contents: Bits = BitVec::new();
    let syncword: Bits = BitVec::repeat(true, SYNC_LEN);
    io::copy(&mut File::open(path).unwrap(), &mut contents).expect("Assuming io::copy is all gucci");
    let mut header_data: Bits = BitVec::new();
    contents.windows(32)
        .skip_while(|x| {
            println!("asd");
            x[..11] != syncword
        })
        .next()
        .unwrap()
        .clone_into(&mut header_data);
    Header::from(header_data)
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
    println!("{}", frame);
    let mut test_field: Bits = BitVec::new();
    io::copy(&mut File::open(path).unwrap(), &mut test_field).unwrap();
    for i in (0..124).step_by(4) {
        println!("{:?}", &test_field[i..i+4]);
    }

    let path = "huh.mp3";
    let frame = read_first_frame(path);
    println!("{}", frame);
    let mut test_field: Bits = BitVec::new();
    io::copy(&mut File::open(path).unwrap(), &mut test_field).unwrap();
    for i in (0..124).step_by(4) {
        println!("{:?}", &test_field[i..i+4]);
    }
}
