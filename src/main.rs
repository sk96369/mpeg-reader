use std::{fs::File, io};
use bitvec::prelude::*;

const SYNC_LEN: usize = 11;

type Bits = BitVec<u8, Msb0>;

mod mpeg_file_structure {
    use std::fmt::{self, Display};

    use super::*;

    pub struct MPEG {
        //-----------------Consider removing the filename field
        filename: String,
        frames: Vec<Frame>,
    }

    impl MPEG {
        pub fn open(path: &str) -> MPEG {
            let mut file = File::open(path).unwrap();
            let mut file_data: Bits = BitVec::new();
            io::copy(&mut file, &mut file_data).unwrap();
            let mut mpeg = MPEG::from(file_data);
            mpeg.filename = path.to_string();
            mpeg
        }
    }

    impl Display for MPEG {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut frame_lengths = String::new();
            self.frames.iter()
                .for_each(|f| frame_lengths += &f.get_frame_len().to_string()[..]);
            write!(f, "Filename: {}\nFrames: {}\n", self.filename, self.frames.len())
        }
    }

    impl From<Bits> for MPEG {
        fn from(file_data: Bits) -> Self {
            let mut mpeg = MPEG {
                filename: "unknown".to_string(),
                frames: Vec::new(),
            };
            let syncword: Bits = BitVec::repeat(true, SYNC_LEN);
            let mut cursor_pos = 0;
            let mut block_end = 0;
            let header_size = 32;
            loop {
                //println!("test: {}", cursor_pos);
                while file_data[cursor_pos..(cursor_pos + SYNC_LEN)] != syncword {
                    cursor_pos += 1;
                }

                let mut header_data: Bits = BitVec::new();
                file_data[cursor_pos..cursor_pos + header_size].clone_into(&mut header_data);
                println!("header data: {:?}", &header_data[11..]);
                let header = Header::from(header_data);
                //println!("cursor pos: {}", cursor_pos);

                let mut frame_data: Bits = BitVec::new();
                //header_size is included in the frame length
                block_end = cursor_pos + header.get_frame_len();
                println!("header bitrate: {}", header.get_bitrate());
                println!("header samplerate: {}", header.get_sampling_rate());
                println!("header frame_len: {}", header.get_frame_len());
                println!("block end: {block_end}");
                file_data[cursor_pos..(block_end)].clone_into(&mut frame_data);

                println!("header: {header}\n");
                mpeg.frames.push(Frame {
                    data: frame_data,
                    header: header,
                });

                //println!("block end: {}", block_end);
                if mpeg.frames.len() > 2 || block_end + 32 > file_data.len() {
                    return mpeg;
                }

                cursor_pos = block_end;
            }
        }
    }

    pub struct Frame {
        header: Header,
        data: Bits,
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
        fn get_frame_len(&self) -> usize;
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

        fn get_frame_len(&self) -> usize {
            self.header.get_frame_len()
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
            println!("test: {}", &self.data[16..20]);
            match self.data[16..20].load() {
                0 => 0,
                1 => 32,
                2 => 40,
                3 => 48,
                4 => 56,
                5 => 64,
                6 => 80,
                7 => 96,
                8 => 112,
                9 => 128,
                10 => 160,
                11 => 192,
                12 => 224,
                13 => 256,
                14 => 320,
                _ => 0,
            }
        }

        fn get_sampling_rate(&self) -> u16 {
            match self.data[20..22].load() {
                0 => 44100,
                1 => 48000,
                2 => 32000,
                _ => 0,
            }
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

        fn get_frame_len(&self) -> usize {
            ((self.get_bitrate() as usize * 144000) / self.get_sampling_rate() as usize) + self.get_padding() as usize
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
                       0 => "error".to_string(),
                       val => val.to_string(),
                   },
                   match self.get_sampling_rate() {
                       0 => "error".to_string(),
                       val => val.to_string(),
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





fn main() {
    use mpeg_file_structure::*;

    let path = "test.mp3";
    let mpeg_1 = MPEG::open(path);

    let path = "huh.mp3";
    let mpeg_2 = MPEG::open(path);

    println!("test.mp3:\n{}\n", mpeg_1);
    println!("huh.mp3:\n{}", mpeg_2);
}
