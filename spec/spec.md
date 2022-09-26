# Specification - NETSWAP

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

# Client-To-Server (CTS) operations and data layout

The Transaction ID is a 32 bit unsigned integer which increases for every operation performed.
When the integer experiences an overflow, it should simply wrap around to 0.

PINDEX is the Packet Index, and should be incremented from 0 for every packet sent.
Server implementations should concatenate the Data fields of packets with the same Transaction ID in the order specified by the Packet index.  
Duplicate packets with the same Packet Index and XID are considered a protocol error and should result in the connection being terminated.

Data Len is the number of packets (indexed from 1) that the current operation takes up.  
Servers should buffer packets with the same XID using the concatenation method specified above.  

Operation Code is always placed in the first Op Header, byte 6.

## OP_CREATE_RECORD
Creates a record in the memswap_rs global buffer.
### Header Allocation
Header 2 : byte 7 : ACCESS
Header 3 : byte 8 : NULL
Header 4 : byte 9 : NULL
Header 5 : byte 10 : NULL

### Header Values

ACCESS:
* The Access header consists of a single 8-bit integer, specifying the access permissions of other clients when accessing this record via a Network Pointer.
* Access header values are inserted into the integer using bitwise operations at implementation discretion.
* Bit 8 is the READ value, which allows clients other than the operator to read the value.
* Bit 7 is the WRITE value, which allows clients other than the operator to write the value.
* Bit 6 allows other clients to take Operator status on a value, and relinquishes it from the current client.
* Values are cleaned up 5 minutes after a client disconnects, therefore in order to make data persistent another client must take ownership within that timeframe. Bit 6 is the solution to this issue.