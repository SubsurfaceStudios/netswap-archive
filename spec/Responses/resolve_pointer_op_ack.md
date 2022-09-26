# RESOLVE_POINTER_OP_ACK
# Opcode (SERVER): 3

Dereferences a pointer on the global buffer.

### Header Allocation
| Header | Byte | Name   |
|--------|------|--------|
| 1      | 6    | OPCODE |
| 2      | 7    | NULL   |
| 3      | 8    | NULL   |
| 4      | 9    | NULL   |
| 5      | 10   | NULL   |

### Header Values

This ACK contains no data in headers.

### Data

Data contained in this ACK is the raw data set in the global buffer by the owner of this address.