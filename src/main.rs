use std::path::PathBuf;
use clap::{AppSettings, Clap, ValueHint};
use hex::decode_to_slice;
use crcli::ALGO_LIST;

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
    
    // Gets a value for config if supplied by user, or defaults to "default.conf"
    println!("{:#?}", opts);

    if let Some(hex) = opts.hex {
        let mut bytes = vec![0u8; hex.len()/2];
        println!("Value for config: {} {}", hex, hex.len());
        decode_to_slice(hex, &mut bytes).unwrap();
        println!("{:#?}", bytes);
        calc_crc(&opts.crc_type, bytes)
    }

    

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match opts.verbose {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        _ => println!("Don't be ridiculous"),
    }
}

fn calc_crc(crc_algo: &str, data: Vec<u8>) {
    if let Some(crc) = ALGO_LIST.iter().find(|x| x.algo_name == crc_algo) {
        let mut crc = (crc.crc_func)();
        crc.digest(&data);
        let crc = crc.get_crc();
        
        println!("CRC:\nDec(LE): {}\nDec(BE): {}\nHex(LE): {:#04x}\nHex(BE): {:#04x}", crc, crc.swap_bytes(), crc, crc.swap_bytes())
        //println!("CRC:\nDec(LE): {}\nHex(LE): {:#04x}", crc.get_crc(), crc.get_crc())
    }
}
