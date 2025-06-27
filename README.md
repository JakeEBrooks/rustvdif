# Rust VDIF

A rust crate for interacting with data encoded in the VLBI Data Interchange Format (VDIF), commonly used in radio astronomy experiments. The VDIF data format is defined in the [VDIF specification](https://vlbi.org/vlbi-standards/vdif/).

This is a minimalist crate designed to relieve the problem of dealing with VDIF data in your own applications.

With `rustvdif` you can:

- Read VDIF frames from and write to various sources, including files and sockets
- Create and interact with VDIF headers
- Read/write VDIF data encoded using the VDIF Transport Protocol (VTP)
- Easily encode and decode VDIF header fields
- Access VDIF payload data in `u32` or byte form
- Encode and decode VDIF payloads, with up to 32 bits/sample

Documentation is available [here](https://docs.rs/rustvdif/latest/rustvdif/). If you haven't come across VDIF before, I recommend reading the VDIF specification linked above as it is actually quite readable.

## Extra features

This crate also contains a number of utilities that are useful for building applications using VDIF data. These include:

- A single-producer single-consumer lock-free ring buffer specifically for VDIF frames. Based on the [rtrb](https://github.com/mgeier/rtrb) crate
- A UDP/VTP receiver type built specifically to utilise the [recvmmsg](https://man7.org/linux/man-pages/man2/recvmmsg.2.html) Linux system call, since VDIF frames are often received over UDP in large volumes

## Contributing

I'd love to see contributions from the VLBI community, and if you have any suggestions or questions you can always reach out to me directly or open an issue.

## Known Issues

Since VDIF is an explicitly little-endian format, supporting big-endian systems takes a bit of extra effort. So big-endian systems aren't currently supported, but I could probably be persuaded to implement support if someone needs it.

## Licensing

This library is licensed under either the MIT License or the Apache 2.0 License at your option.
