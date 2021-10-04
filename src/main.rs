use std::path::PathBuf;
use clap::{AppSettings, Clap, ValueHint};
use hex::decode_to_slice;
use crcli::ALGO_LIST;
use std::io::BufReader;
use std::io::Read;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap, Debug)]
#[clap(version = "v1.0", author = "Erik Kallen, <info@erikkallen.nl>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// File to calculate crc for
    #[clap(name ="FILE", parse(from_os_str), value_hint = ValueHint::FilePath)]
    file: Option<PathBuf>,
    /// The seperator to be used to parse hex string into bytes
    #[clap(short, long, default_value = " ")]
    seperator: String,
    /// Hex string to calculate crc on
    #[clap(long, conflicts_with = "FILE")]
    hex: Option<String>,
    /// Type of predefined crc function to use
    #[clap(short = 't', long = "type", possible_values = &ALGO_LIST.iter().map(|x| x.algo_name).collect::<Vec<&str>>())]
    crc_type: String,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match opts.verbose {
        2 => println!("{:#?}", opts),
        _ => (),
    }

    if let Some(crc) = ALGO_LIST.iter().find(|x| x.algo_name == opts.crc_type) {
        let mut crc = (crc.crc_func)();

        if let Some(hex) = opts.hex {
            let mut bytes = vec![0u8; hex.len()/2];

            match opts.verbose {
                1 => println!("Value for config: {} {}", hex, hex.len()),
                _ => (),
            }

            decode_to_slice(hex, &mut bytes).unwrap();
            
            match opts.verbose {
                2 => println!("{:#?}", bytes),
                _ => (),
            }

            crc.digest(&bytes)
        }

        if let Some(file) = opts.file {
            let file = std::fs::File::open(file).expect("Cannot open file");
            let mut buf_reader = BufReader::new(file);

            let chunk_size = 0x4000;
            loop {
                let mut chunk = Vec::with_capacity(chunk_size);
                let n = buf_reader.by_ref().take(chunk_size as u64).read_to_end(&mut chunk).expect("Cannot read file data");
                crc.digest(&chunk);
                if n == 0 { break; }
                if n < chunk_size { break; }
            }
        }
        println!("CRC:\nHex(LE): 0x{}\nHex(BE): 0x{}", hex::encode(crc.get_crc_vec_le()), hex::encode(crc.get_crc_vec_be()))
        // println!("CRC:\nDec(LE): {}\nDec(BE): {}\nHex(LE): 0x{}\nHex(BE): 0x{}", u32::from_be_bytes(crc.get_crc_vec_le()), u32::from_be_bytes(crc.get_crc_vec_be()), hex::encode(crc.get_crc_vec_le()), hex::encode(crc.get_crc_vec_be()))
        // println!("CRC:\nHex(LE): 0x{}\nHex(BE): 0x{}", hex::encode(crc.get_crc_vec_le()), hex::encode(crc.get_crc_vec_be()))
        // println!("CRC:\nDec(LE): {}\nDec(BE): {}\nHex(LE): 0x{}\nHex(BE): 0x{}", u32::from_be_bytes(crc.get_crc_vec_le()), u32::from_be_bytes(crc.get_crc_vec_be()), hex::encode(crc.get_crc_vec_le()), hex::encode(crc.get_crc_vec_be()))
        //println!("CRC:\nDec(LE): {}\nHex(LE): {:#04x}", crc.get_crc(), crc.get_crc())
    }
}