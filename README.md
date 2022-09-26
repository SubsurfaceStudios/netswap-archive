# NetSwap

`netswap_rs` is a networked buffering system designed as an Out-Of-Band memory system for multiplayer frameworks, server synchronization, and other low-bandwidth applications.

# Name Origin

The name `netswap` originates from the original concept of swap space. In most operating systems, when low on memory, the system will pause for a moment and *swap* less commonly-used memory to a special location on on the disk. This frees up memory, but has some performance overhead.  
The same concept applies to Netswap. The goal is to be able to send a pointer across any other band, and read or even write memory from any other location. This allows for sharing massive amounts of memory compared to sending it directly, similar to passing values by reference into a function instead of by value.  

# Speed
While working over the network does incur a relatively large overhead, the protocol is optimized as much as possible, by using only 10 bytes for internal headers, and up to 1.39 KB per packet of pure binary data for use by applications. The other ~100 bytes per packet are purely overhead from TCP, which cannot be reasonably avoided while maintaining a near-zero data corruption rate.  
Rather than HTTP, which incurs the cost of request headers, request processing, response headers, and more, `netswap` is built entirely on top of TCP/IP, again, to optimize bandwidth and speed as much as possible.

# Authentication (or lack thereof)
Authentication is performed purely off IP hash, and there are no plans for token-based or certificate based authentication.

# Platforms
Due to being based entirely on TCP/IP, essentially any device with a network connection can run `netswap` without issue.  
  
Language wrappers are either planned or already in progress for the following languages, with more to come depending on the outcome of this project:  
* C#
* Node.js
* Rust

# Roadmap
- [ ] Basic implementation of CRUD operations
    - [ ] Multi-packet request buffering
    - [ ] Create record
    - [ ] Replace record data
    - [ ] Update record permissions
    - [ ] Delete record
- [ ] Ownership takeover / transfer
    - [ ] `OP_TAKEOVER_RECORD`
    - [ ] Change owner IP hash
- [ ] C# Client Wrapper
- [ ] Node.js Client Wrapper
- [ ] Rust Client Wrapper