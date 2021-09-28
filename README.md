# CRcli

Commandline crc calculator

Features:

- Calculate any type of crc for files, hex strings, text

## Examples

```shell
# CRC a file
crcli -t crc16 file.txt

# CRC a string of hex
crcli -t crc16_modbus --hex "34 56 34 76"

# CRC a hex string with custom seperator
crcli -t crc16 --hex "34, 56, 34, 76" -s ", "
```