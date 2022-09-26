## OP_UPDATE_RECORD_PERMISSIONS
## Opcode: 1
### Header Allocation
| Header | Byte | Name   |
|--------|------|--------|
| 1      | 6    | OPCODE |
| 2      | 7    | ACCESS |
| 3      | 8    | NULL   |
| 4      | 9    | NULL   |
| 5      | 10   | NULL   |

### Header Values
ACCESS is the new access values for the specified record.


### Data
The first 64 bits (8 bytes) of the Data field should be a Network Pointer reference to the Record you wish to update the permissions of.

### Failures
* This operation will fail with an Access Violation ACK if the client is not the Operator of this record.
* This operation will fail with a Segmentation Fault ACK if the specified record address is under 0, above the 64 bit integer limit, or does not exist.

### Response
On success, this operation responds with a GENERIC_OP_SUCCESS_ACK with the XID.