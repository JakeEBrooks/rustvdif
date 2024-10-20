# Rust VDIF
A rust crate for interacting with data encoded in the VLBI Data Interchange Format (VDIF), commonly used in radio astronomy experiments. The VDIF data format is defined in the VDIF specification, found [here](https://vlbi.org/vlbi-standards/vdif/).

This is a minimalist crate designed to relieve the problem of dealing with VDIF data in your own applications.

With `rustvdif` you can:

- Parse `VDIFFrame`s from simple `&[u8]` byte slices.
- Easily access fields within a header, using `VDIFHeader`.
- Encode and decode VDIF payloads, with up to 16 bits/sample.
- Easily access VDIF files using `VDIFFileReader`.
- Read VDIF types directly from any type implementing `std::io::Read` using `VDIFReader`.

Documentation is available [here](https://docs.rs/rustvdif/latest/rustvdif/).

## Contributing
I'd love to see contributions from the VLBI community, and if you have any suggestions or questions you can always reach out to me directly or open an issue.

## Licensing
This library is licensed under either the MIT License or the Apache 2.0 License at your option.