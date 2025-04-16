# Rust VDIF

A rust crate for interacting with data encoded in the VLBI Data Interchange Format (VDIF), commonly used in radio astronomy experiments. The VDIF data format is defined in the VDIF specification, found [here](https://vlbi.org/vlbi-standards/vdif/).

This is a minimalist crate designed to relieve the problem of dealing with VDIF data in your own applications.

With `rustvdif` you can:

- Read VDIF frames from and write to various sources, including files and sockets
- Read/write VDIF data encoded using the VDIF Transport Protocol (VTP)
- Easily encode and decode VDIF header fields
- Access VDIF payload data in `u32` or byte form
- Encode and decode VDIF payloads, with up to 32 bits/sample

Documentation is available [here](https://docs.rs/rustvdif/latest/rustvdif/).

## Contributing

I'd love to see contributions from the VLBI community, and if you have any suggestions or questions you can always reach out to me directly or open an issue.

## Known Issues

Since VDIF is an explicitly little-endian format, supporting big-endian systems takes a bit of extra effort. So big-endian systems aren't currently supported, but I could probably be persuaded to implement support if someone needs it.

## Licensing

This library is licensed under either the MIT License or the Apache 2.0 License at your option.
