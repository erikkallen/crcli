use std::path::PathBuf;
use clap::{AppSettings, Clap, ValueHint};
use hex::decode_to_slice;
use crc_any::CRC;

type CRCFn = fn() -> CRC;

struct CrcType<'a> {
    algo_name: &'a str,
    crc_func: CRCFn,
}

const ALGO_LIST: [CrcType; 8] = [
        CrcType { algo_name: "CRC8", crc_func: CRC::crc8 },
        CrcType { algo_name: "CRC8_CDMA2000", crc_func: CRC::crc8cdma2000 },
        CrcType { algo_name: "CRC8_DARC", crc_func: CRC::crc8darc },
        CrcType { algo_name: "CRC8_DVB_S2", crc_func: CRC::crc8dvb_s2 },
        // CrcType { algo_name: "CRC8_GSM_A", crc_func: CRC:: },
        // CrcType { algo_name: "CRC8_GSM_B", crc_func: CRC:: },
        // CrcType { algo_name: "CRC8_I_432_1", crc_func: CRC:: },
        CrcType { algo_name: "CRC8_I_CODE", crc_func: CRC::crc8icode },
        // CrcType { algo_name: "CRC8_LTE", crc_func: CRC:: },
        CrcType { algo_name: "CRC8_MAXIM_DOW", crc_func: CRC::crc8maxim },
        // CrcType { algo_name: "CRC8_MIFARE_MAD", crc_func: CRC:: },
        // CrcType { algo_name: "CRC8_NRSC_5", crc_func: CRC:: },
        // CrcType { algo_name: "CRC8_OPENSAFETY", crc_func: CRC:: },
        CrcType { algo_name: "CRC8_ROHC", crc_func: CRC::crc8rohc },
        // CrcType { algo_name: "CRC8_SAE_J1850", crc_func: CRC:: },
        // CrcType { algo_name: "CRC8_SMBUS", crc_func: CRC:: },
        // CrcType { algo_name: "CRC8_TECH_3250", crc_func: CRC:: },
        CrcType { algo_name: "CRC8_WCDMA", crc_func: CRC::crc8wcdma },
        // CrcType { algo_name: "CRC16_ARC", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_CDMA2000", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_CMS", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_DDS_110", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_DECT_R", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_DECT_X", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_DNP", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_EN_13757", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_GENIBUS", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_GSM", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_IBM_3740", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_IBM_SDLC", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_ISO_IEC_14443_3_A", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_KERMIT", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_LJ1200", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_MAXIM_DOW", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_MCRF4XX", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_MODBUS", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_NRSC_5", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_OPENSAFETY_A", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_OPENSAFETY_B", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_PROFIBUS", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_RIELLO", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_SPI_FUJITSU", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_T10_DIF", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_TELEDISK", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_TMS37157", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_UMTS", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_USB", crc_func: CRC:: },
        // CrcType { algo_name: "CRC16_XMODEM", crc_func: CRC:: },
        // CrcType { algo_name: "CRC32_AIXM", crc_func: CRC:: },
        // CrcType { algo_name: "CRC32_AUTOSAR", crc_func: CRC:: },
        // CrcType { algo_name: "CRC32_BASE91_D", crc_func: CRC:: },
        // CrcType { algo_name: "CRC32_BZIP2", crc_func: CRC:: },
        // CrcType { algo_name: "CRC32_CD_ROM_EDC", crc_func: CRC:: },
        // CrcType { algo_name: "CRC32_CKSUM", crc_func: CRC:: },
        // CrcType { algo_name: "CRC32_ISCSI", crc_func: CRC:: },
        // CrcType { algo_name: "CRC32_ISO_HDLC", crc_func: CRC:: },
        // CrcType { algo_name: "CRC32_JAMCRC", crc_func: CRC:: },
        // CrcType { algo_name: "CRC32_MPEG_2", crc_func: CRC:: },
        // CrcType { algo_name: "CRC32_XFER", crc_func: CRC:: },
        // CrcType { algo_name: "CRC64_ECMA_182", crc_func: CRC:: },
        // CrcType { algo_name: "CRC64_GO_ISO", crc_func: CRC:: },
        // CrcType { algo_name: "CRC64_WE", crc_func: CRC:: },
        // CrcType { algo_name: "CRC64_XZ", crc_func: CRC:: },
    ];
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
