use crc_any::CRC;

pub type CRCFn = fn() -> CRC;

pub struct CrcType<'a> {
    pub algo_name: &'a str,
    pub crc_func: CRCFn,
}

pub const ALGO_LIST: [CrcType; 101] = [
    CrcType {
        algo_name: "CRC3_GSM",
        crc_func: CRC::crc3gsm,
    },
    CrcType {
        algo_name: "CRC4_ITU",
        crc_func: CRC::crc4itu,
    },
    CrcType {
        algo_name: "CRC4_INTERLAKEN",
        crc_func: CRC::crc4interlaken,
    },
    CrcType {
        algo_name: "CRC5_EPC",
        crc_func: CRC::crc5epc,
    },
    CrcType {
        algo_name: "CRC5_ITU",
        crc_func: CRC::crc5itu,
    },
    CrcType {
        algo_name: "CRC5_USB",
        crc_func: CRC::crc5usb,
    },
    CrcType {
        algo_name: "CRC6_CDMA2000_A",
        crc_func: CRC::crc6cdma2000_a,
    },
    CrcType {
        algo_name: "CRC6_CDMA2000_B",
        crc_func: CRC::crc6cdma2000_b,
    },
    CrcType {
        algo_name: "CRC6_ARC",
        crc_func: CRC::crc6darc,
    },
    CrcType {
        algo_name: "CRC6_GSM",
        crc_func: CRC::crc6gsm,
    },
    CrcType {
        algo_name: "CRC6_ITU",
        crc_func: CRC::crc6itu,
    },
    CrcType {
        algo_name: "CRC7",
        crc_func: CRC::crc7,
    },
    CrcType {
        algo_name: "CRC7_UMTS",
        crc_func: CRC::crc7umts,
    },
    CrcType {
        algo_name: "CRC8",
        crc_func: CRC::crc8,
    },
    CrcType {
        algo_name: "CRC8_AUTOSAR",
        crc_func: || CRC::create_crc(0x2f, 8, 0xff, 0xff, false),
    },
    CrcType {
        algo_name: "CRC8_BLUETOOTH",
        crc_func: || CRC::create_crc(0xa7, 8, 0x00, 0xff, true),
    },
    CrcType {
        algo_name: "CRC8_CDMA2000",
        crc_func: CRC::crc8cdma2000,
    },
    CrcType {
        algo_name: "CRC8_DARC",
        crc_func: CRC::crc8darc,
    },
    CrcType {
        algo_name: "CRC8_DVB_S2",
        crc_func: CRC::crc8dvb_s2,
    },
    CrcType {
        algo_name: "CRC8_GSM_A",
        crc_func: || CRC::create_crc(0x1d, 8, 0x00, 0x00, false),
    },
    CrcType {
        algo_name: "CRC8_GSM_B",
        crc_func: || CRC::create_crc(0x49, 8, 0x00, 0xff, false),
    },
    CrcType {
        algo_name: "CRC8_EBU",
        crc_func: CRC::crc8ebu,
    },
    CrcType {
        algo_name: "CRC8_I_432_1",
        crc_func: || CRC::create_crc(0x07, 8, 0x00, 0xff, false),
    },
    CrcType {
        algo_name: "CRC8_I_CODE",
        crc_func: CRC::crc8icode,
    },
    CrcType {
        algo_name: "CRC8_ITU",
        crc_func: CRC::crc8itu,
    },
    CrcType {
        algo_name: "CRC8_LTE",
        crc_func: || CRC::create_crc(0x9b, 8, 0x00, 0x00, false),
    },
    CrcType {
        algo_name: "CRC8_MAXIM_DOW",
        crc_func: CRC::crc8maxim,
    },
    CrcType {
        algo_name: "CRC8_MIFARE_MAD",
        crc_func: || CRC::create_crc(0x1d, 8, 0xc7, 0x00, false),
    },
    CrcType {
        algo_name: "CRC8_NRSC_5",
        crc_func: || CRC::create_crc(0x31, 8, 0xff, 0x00, false),
    },
    CrcType {
        algo_name: "CRC8_OPENSAFETY",
        crc_func: || CRC::create_crc(0x2f, 8, 0x00, 0x00, false),
    },
    CrcType {
        algo_name: "CRC8_ROHC",
        crc_func: CRC::crc8rohc,
    },
    CrcType {
        algo_name: "CRC8_SAE_J1850",
        crc_func: || CRC::create_crc(0x1d, 8, 0xff, 0xff, false),
    },
    CrcType {
        algo_name: "CRC8_SMBUS",
        crc_func: || CRC::create_crc(0x07, 8, 0x00, 0x00, false),
    },
    CrcType {
        algo_name: "CRC8_TECH_3250",
        crc_func: || CRC::create_crc(0x1d, 8, 0xff, 0x00, true),
    },
    CrcType {
        algo_name: "CRC8_WCDMA",
        crc_func: CRC::crc8wcdma,
    },
    CrcType {
        algo_name: "CRC10",
        crc_func: CRC::crc10,
    },
    CrcType {
        algo_name: "CRC10_CDMA2000",
        crc_func: CRC::crc10cdma2000,
    },
    CrcType {
        algo_name: "CRC10_GSM",
        crc_func: CRC::crc10gsm,
    },
    CrcType {
        algo_name: "CRC11",
        crc_func: CRC::crc11,
    },
    CrcType {
        algo_name: "CRC12",
        crc_func: CRC::crc12,
    },
    CrcType {
        algo_name: "CRC12_CDMA2000",
        crc_func: CRC::crc12cdma2000,
    },
    CrcType {
        algo_name: "CRC12_GSM",
        crc_func: CRC::crc12gsm,
    },
    CrcType {
        algo_name: "CRC13_BBC",
        crc_func: CRC::crc13bbc,
    },
    CrcType {
        algo_name: "CRC14_DARC",
        crc_func: CRC::crc14darc,
    },
    CrcType {
        algo_name: "CRC14_GSM",
        crc_func: CRC::crc14gsm,
    },
    CrcType {
        algo_name: "CRC15_CAN",
        crc_func: CRC::crc15can,
    },
    CrcType {
        algo_name: "CRC15_MPT1327",
        crc_func: CRC::crc15mpt1327,
    },
    CrcType {
        algo_name: "CRC16",
        crc_func: CRC::crc16,
    },
    CrcType {
        algo_name: "CRC16_A",
        crc_func: CRC::crc_a,
    },
    CrcType {
        algo_name: "CRC16_ARC",
        crc_func: || CRC::create_crc(0x8005, 16, 0x0000, 0x0000, true),
    },
    CrcType {
        algo_name: "CRC16_AUG_CCIT",
        crc_func: CRC::crc16aug_ccitt,
    },
    CrcType {
        algo_name: "CRC16_CCITT_FALSE",
        crc_func: CRC::crc16ccitt_false,
    },
    CrcType {
        algo_name: "CRC16_BUYPASS",
        crc_func: CRC::crc16buypass,
    },
    CrcType {
        algo_name: "CRC16_CDMA2000",
        crc_func: CRC::crc10cdma2000,
    },
    CrcType {
        algo_name: "CRC16_CMS",
        crc_func: || CRC::create_crc(0x8005, 16, 0xffff, 0x0000, false),
    },
    CrcType {
        algo_name: "CRC16_DDS_110",
        crc_func: CRC::crc16dds_110,
    },
    CrcType {
        algo_name: "CRC16_DECT_R",
        crc_func: CRC::crc16dect_r,
    },
    CrcType {
        algo_name: "CRC16_DECT_X",
        crc_func: CRC::crc16dect_x,
    },
    CrcType {
        algo_name: "CRC16_DNP",
        crc_func: CRC::crc16dnp,
    },
    CrcType {
        algo_name: "CRC16_EN_13757",
        crc_func: CRC::crc16en_13757,
    },
    CrcType {
        algo_name: "CRC16_GENIBUS",
        crc_func: CRC::crc16genibus,
    },
    CrcType {
        algo_name: "CRC16_GSM",
        crc_func: || CRC::create_crc(0x1021, 16, 0x0000, 0xffff, false),
    },
    CrcType {
        algo_name: "CRC16_IBM_3740",
        crc_func: || CRC::create_crc(0x1021, 16, 0xffff, 0x0000, false),
    },
    CrcType {
        algo_name: "CRC16_IBM_SDLC",
        crc_func: || CRC::create_crc(0x1021, 16, 0xffff, 0xffff, true),
    },
    CrcType {
        algo_name: "CRC16_ISO_IEC_14443_3_A",
        crc_func: || CRC::create_crc(0x1021, 16, 0xc6c6, 0x0000, true),
    },
    CrcType {
        algo_name: "CRC16_KERMIT",
        crc_func: CRC::crc16kermit,
    },
    CrcType {
        algo_name: "CRC16_LJ1200",
        crc_func: || CRC::create_crc(0x6f63, 16, 0x0000, 0x0000, false),
    },
    CrcType {
        algo_name: "CRC16_MAXIM_DOW",
        crc_func: CRC::crc16maxim,
    },
    CrcType {
        algo_name: "CRC16_MCRF4XX",
        crc_func: CRC::crc16mcrf4cc,
    },
    CrcType {
        algo_name: "CRC16_MODBUS",
        crc_func: CRC::crc16modbus,
    },
    CrcType {
        algo_name: "CRC16_NRSC_5",
        crc_func: || CRC::create_crc(0x080b, 16, 0xffff, 0x0000, true),
    },
    CrcType {
        algo_name: "CRC16_OPENSAFETY_A",
        crc_func: || CRC::create_crc(0x5935, 16, 0x0000, 0x0000, false),
    },
    CrcType {
        algo_name: "CRC16_OPENSAFETY_B",
        crc_func: || CRC::create_crc(0x755b, 16, 0x0000, 0x0000, false),
    },
    CrcType {
        algo_name: "CRC16_PROFIBUS",
        crc_func: || CRC::create_crc(0x1dcf, 16, 0xffff, 0xffff, false),
    },
    CrcType {
        algo_name: "CRC16_RIELLO",
        crc_func: CRC::crc16riello,
    },
    CrcType {
        algo_name: "CRC16_SPI_FUJITSU",
        crc_func: || CRC::create_crc(0x1021, 16, 0x1d0f, 0x0000, false),
    },
    CrcType {
        algo_name: "CRC16_T10_DIF",
        crc_func: CRC::crc16t10_dif,
    },
    CrcType {
        algo_name: "CRC16_TELEDISK",
        crc_func: CRC::crc16teledisk,
    },
    CrcType {
        algo_name: "CRC16_TMS37157",
        crc_func: CRC::crc16tms13157,
    },
    CrcType {
        algo_name: "CRC16_UMTS",
        crc_func: || CRC::create_crc(0x8005, 16, 0x0000, 0x0000, false),
    },
    CrcType {
        algo_name: "CRC16_USB",
        crc_func: CRC::crc16usb,
    },
    CrcType {
        algo_name: "CRC16_X25",
        crc_func: CRC::crc16_x25,
    },
    CrcType {
        algo_name: "CRC16_XMODEM",
        crc_func: CRC::crc16xmodem,
    },
    CrcType {
        algo_name: "CRC17_CAN",
        crc_func: CRC::crc17can,
    },
    CrcType {
        algo_name: "CRC21_CAN",
        crc_func: CRC::crc21can,
    },
    CrcType {
        algo_name: "CRC24",
        crc_func: CRC::crc24,
    },
    CrcType {
        algo_name: "CRC24_BLE",
        crc_func: CRC::crc24ble,
    },
    CrcType {
        algo_name: "CRC24_FLEXRAY_A",
        crc_func: CRC::crc24flexray_a,
    },
    CrcType {
        algo_name: "CRC24_FLEXRAY_A",
        crc_func: CRC::crc24flexray_b,
    },
    CrcType {
        algo_name: "CRC24_LTE_A",
        crc_func: CRC::crc24lte_a,
    },
    CrcType {
        algo_name: "CRC24_LTE_B",
        crc_func: CRC::crc24lte_b,
    },
    CrcType {
        algo_name: "CRC24_OS9",
        crc_func: CRC::crc24os9,
    },
    CrcType {
        algo_name: "CRC30_CDMA",
        crc_func: CRC::crc30cdma,
    },
    CrcType {
        algo_name: "CRC32",
        crc_func: CRC::crc32,
    },
    // CrcType { algo_name: "CRC32_AIXM", crc_func: CRC:: },
    // CrcType { algo_name: "CRC32_AUTOSAR", crc_func: CRC:: },
    // CrcType { algo_name: "CRC32_BASE91_D", crc_func: CRC:: },
    CrcType {
        algo_name: "CRC32_BZIP2",
        crc_func: CRC::crc32bzip2,
    },
    // CrcType { algo_name: "CRC32_CD_ROM_EDC", crc_func: CRC:: },
    CrcType {
        algo_name: "CRC32_CKSUM",
        crc_func: CRC::crc32mhash,
    },
    // CrcType { algo_name: "CRC32_ISCSI", crc_func: CRC:: },
    // CrcType { algo_name: "CRC32_ISO_HDLC", crc_func: CRC:: },
    // CrcType { algo_name: "CRC32_JAMCRC", crc_func: CRC:: },
    CrcType {
        algo_name: "CRC32_MPEG_2",
        crc_func: CRC::crc32mpeg2,
    },
    // CrcType { algo_name: "CRC32_XFER", crc_func: CRC:: },
    CrcType {
        algo_name: "CRC64",
        crc_func: CRC::crc64,
    },
    // CrcType { algo_name: "CRC64_ECMA_182", crc_func: CRC:: },
    CrcType {
        algo_name: "CRC64_GO_ISO",
        crc_func: CRC::crc64iso,
    },
    CrcType {
        algo_name: "CRC64_JONES",
        crc_func: CRC::crc64jones,
    },
    CrcType {
        algo_name: "CRC64_WE",
        crc_func: CRC::crc64we,
    },
    // CrcType { algo_name: "CRC64_XZ", crc_func: CRC:: },
];
