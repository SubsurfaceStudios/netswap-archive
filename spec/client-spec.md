# Specification - NETSWAP Client

# Client-To-Server (CTS) packet diagram

|  Byte  |   0-3   |    6-10    |        11 - =< 1400      |
|--------|---------|------------|--------------------------|
|  Data  |   XID   | Op Headers |      Operation Data      |

Example:

|        Raw Bytes                    | Label    | Value          |
|-------------------------------------|----------|----------------|
| 00000000 00000000 00000000 00000000 | XID      | Transaction 0  |
| 00000000                            | PINDEX   | Packet 1       |
| 00000001                            | Data Len | 1 Packet Total |
| 00000001                            | Opcode   | CREATE_RECORD  |
| 00000000                            | Access   | DENY ALL       |
| 00000001                            | NULL     |                |
| 00000000                            | NULL     |                |
| 00000000                            | NULL     |                |
| 01010100 01000101 01010011 01010100 | Data     | ASCII "TEST"   |


Due to MTU restrictions, the max size of a TCP packet (going either way) is 1500 bytes.  
Accounting for TCP overhead, this makes the maximum size of a CTS packet 1400 bytes.  
Accounting for the transaction ID and header information, the maximum size of the Operation Data field is 1390 bytes.