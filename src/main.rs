use clap::{Parser, ValueHint};
use crcli::ALGO_LIST;
use hex::decode_to_slice;
use std::fmt::Write as _;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;
/// This application calculates crc of a file or hex string based on the type of algorithm requested
#[derive(Parser, Debug)]
#[clap(version = "v1.0", author = "Erik Kallen, <info@erikkallen.nl>")]

struct Opts {
    /// File to calculate crc for
    #[clap(name ="FILE", parse(from_os_str), value_hint = ValueHint::FilePath)]
    file: Option<PathBuf>,
    /// The seperator to be used to parse hex string into bytes
    #[clap(short = 's', long, default_value = " ")]
    seperator: String,
    /// Hex string to calculate crc on
    #[clap(long, conflicts_with = "FILE")]
    hex: Option<String>,
    /// Type of predefined crc function to use
    #[clap(short = 't', long = "type", ignore_case = true, possible_values = ALGO_LIST.iter().map(|x| x.algo_name).collect::<Vec<&str>>())]
    crc_type: String,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    if opts.verbose == 2 {
        println!("{:#?}", opts)
    }

    if let Some(crc) = ALGO_LIST
        .iter()
        .find(|x| x.algo_name == opts.crc_type.to_uppercase())
    {
        let crc_name = opts.crc_type.to_uppercase();
        let mut crc = (crc.crc_func)();

        if let Some(hex) = opts.hex {
            let split: Vec<String> = hex.split(&opts.seperator).map(str::to_string).collect();
            let mut hex_string = String::new();
            for s in split {
                let s = s.trim();
                let patterns: &[_] = &['0', 'x'];
                let result = s.trim_start_matches(patterns);
                let _ = write!(hex_string, "{:0>2}", result);
            }
            let hex_string = hex_string.trim();

            let mut bytes = vec![0u8; hex_string.len() / 2];

            if opts.verbose == 1 {
                println!(
                    "Value for config: hex[{}] hex len[{}] hex str len[{}]",
                    hex,
                    hex.len(),
                    hex_string.len()
                )
            }

            decode_to_slice(hex_string, &mut bytes as &mut [u8]).unwrap();

            if opts.verbose == 2 {
                println!("{:#?}", bytes)
            }

            crc.digest(&bytes)
        }

        if let Some(file) = opts.file {
            let file = std::fs::File::open(file).expect("Cannot open file");
            let mut buf_reader = BufReader::new(file);

            let chunk_size = 0x4000;
            loop {
                let mut chunk = Vec::with_capacity(chunk_size);
                let n = buf_reader
                    .by_ref()
                    .take(chunk_size as u64)
                    .read_to_end(&mut chunk)
                    .expect("Cannot read file data");
                crc.digest(&chunk);
                if n == 0 {
                    break;
                }
                if n < chunk_size {
                    break;
                }
            }
        }
        println!("{}:", crc_name);
        println!("Hex(LE): 0x{}", hex::encode(crc.get_crc_vec_le()));
        println!("Hex(BE): 0x{}", hex::encode(crc.get_crc_vec_be()));
        println!(
            "Dec(LE): {}",
            i64::from_str_radix(&hex::encode(crc.get_crc_vec_le()), 16).expect("ERROR")
        );
        println!(
            "Dec(BE): {}",
            i64::from_str_radix(&hex::encode(crc.get_crc_vec_be()), 16).expect("ERROR")
        );

        if opts.verbose == 2 {
            println!("Extra info crc:");
            println!("{:#?}", crc)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
