use std::io::{self, Read, Write, BufReader};
use std::env;
use std::fs::File;

struct Header {
    s_riff: u32,
    filesize: u32,
    s_wave: u32,
    s_fmt: u32,
    fmt_size :u32,
    fmt_id: u16,
    channel: u16,
    rate_sample: u32,
    rate_data: u32,
    block_size: u16,
    bit_per_sample: u16,
    s_data: u32,
    data_size: u32
}

impl Header{
    fn new( dsize: u32 ) -> Header{
        Header {
            s_riff : 0x52494646, //RIFF
            filesize : dsize+20,
            s_wave : 0x57415645, //WAVE
            s_fmt : 0x666D7420, //fmt
            fmt_size : 0x10000000, //16(10 00 00 00)
            fmt_id : 0x0100, // 1
            channel : 0x0100, // mono:1(01 00) steleo:2(02 00)
            rate_sample : 0x44AC0000, //44100
            rate_data : 0x44AC0000, //mono:44100 steleo:176400(10 B1 02 00)
            block_size : 0x0100, //16bit:4(04 00) 8bit:1(01 00) Byte/sample*channel
            bit_per_sample : 0x0800, // 8bit or 16bit bit/sample
            s_data : 0x64617461, //data
            data_size : dsize,
        }
    }
    fn create(Header { s_riff, filesize, s_wave, s_fmt, fmt_size, fmt_id, channel, rate_sample, rate_data, block_size, bit_per_sample, s_data, data_size }: Header) -> [u32; 11] {
        let mut res: [u32; 11] = [0; 11];
        res[0] = s_riff;
        res[1] = filesize;
        res[2] = s_wave;
        res[3] = s_fmt;
        res[4] = fmt_size;
        res[5] = (fmt_id as u32) << 16 | channel as u32;
        res[6] = rate_sample;
        res[7] = rate_data;
        res[8] = (block_size as u32)<< 16 | bit_per_sample as u32;
        res[9] = s_data;
        res[10] = data_size;
        res
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();
    if argc != 2{
        println!("Not enough Argument");
        return;
    }
    let filepath = &args[1];

    let mut ifile = File::open(filepath).expect("File Not Found");
    let mut ofile = File::create(format!("{}.wav", filepath)).expect("File Not Found");

    let mut buf = Vec::new();
    let _ = ifile.read_to_end(&mut buf);
    let filesize = buf.len() as u32;
    let wavheader = Header::new(filesize);
    let wavheader = Header::create(wavheader);

    //let _ = ofile.write_all(&wavheader);
    for byte in &wavheader {
        let b1: u8 = ((byte >> 24) & 0xff) as u8;
        let b2: u8 = ((byte >> 16) & 0xff) as u8;
        let b3: u8 = ((byte >> 8) & 0xff) as u8;
        let b4: u8 = (byte & 0xff) as u8;
        let _ = ofile.write_all(&[b1]);
        let _ = ofile.write_all(&[b2]);
        let _ = ofile.write_all(&[b3]);
        let _ = ofile.write_all(&[b4]);
    }
    let _ = ofile.write_all(&buf);
    let _ = ofile.flush();
    println!("Complete");
}
