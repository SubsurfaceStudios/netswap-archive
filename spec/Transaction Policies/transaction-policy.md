# Transaction Policy

The Transaction ID is a 32 bit unsigned integer which increases for every operation performed.
When the integer experiences an overflow, it should simply wrap around to 0.

Operation Code is always placed in the first Op Header, byte 6.