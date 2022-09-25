# Specification - NETSWAP

# Client-To-Server (CTS) packet diagram

|  Byte  |   0-4   |    5-10    |        6 - =< 1400       |
|--------|---------|------------|--------------------------|
|  Data  |   XID   | Op Headers |      Operation Data      |

Example:

|        Raw Bytes                    | Label  | Value         |
|-------------------------------------|--------|---------------|
| 00000000 00000000 00000000 00000001 | XID    | Transaction 1 |
| 00000001                            | Opcode | CREATE_RECORD |
| 00000000                            | Access | DENY ALL      |
| 00000001                            | DatLen | 1 Packet Only |
| 00000000                            | NULL   |               |
| 00000000                            | NULL   |               |
| 01010100 01000101 01010011 01010100 | Data   | ASCII "TEST"  |


Due to MTU restrictions, the max size of a TCP packet (going either way) is 1500 bytes.
Accounting for TCP overhead, this makes the maximum size of a CTS packet 1400 bytes.
Accounting for the transaction ID and header information, the maximum size of the Operation Data field is 1390 bytes.

# Client-To-Server (CTS) operations and data layout

Operation Code is always placed in the first Op Header, index 5.

## OP_CREATE_RECORD
