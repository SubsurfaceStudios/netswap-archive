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
* Access header values are inserted into the integer using bitwise operations at client implementation discretion.
* Bit 8 is the READ value, which allows clients other than the operator to read the value.
* Bit 7 is the WRITE value, which allows clients other than the operator to write the value.
* Bit 6 allows other clients to take Operator status on a value, and relinquishes it from the current client.
* Bit 6 is reset to 0 after a client takes Operator status on a value (or after the OP_PERMIT_TRANSFER operation), and can be set back to 1 with the OP_FORBID_TRANSFER operation with the VALUE header 1.
* Values are cleaned up 5 minutes after a client disconnects, therefore in order to make data persistent another client must take ownership within that timeframe. Bit 6 is the solution to this issue.

### Data

Data is optional for this operation, however if it is specified it must be concatenated in accordance with the Fragmentation policy.
