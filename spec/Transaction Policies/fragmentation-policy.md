# Fragmentation Policy
PINDEX is the Packet Index, and should be incremented from 0 for every packet sent.
Server implementations should concatenate the Data fields of packets with the same Transaction ID in the order specified by the Packet index.  
Duplicate packets with the same Packet Index and XID are considered a protocol error and should result in the connection being terminated.

Data Len is the number of packets (indexed from 1) that the current operation takes up.  
Servers should buffer packets with the same XID using the concatenation method specified above.