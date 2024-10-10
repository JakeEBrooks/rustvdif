# Rust VDIF
A rust crate for interacting with data encoded in the VLBI Data Interchange Format (VDIF), commonly used in radio astronomy experiments. The VDIF data format is defined in the VDIF specification, found [here](https://vlbi.org/vlbi-standards/vdif/).

This is a minimalist crate designed to relieve the problem of dealing with VDIF data in your own applications.

# Wishlist
I'd like to add the following over time:

- Implement an easy to use VDIF buffer.
- Implement an easy to use UDP reader, which maintains the order of incoming frames as much as possible.
- Better API for specifying and reading EDV data.
- Better datetime interfaces for the header.
- Implement common operations for the payload such as those contained in the [baseband-tasks](https://baseband.readthedocs.io/projects/baseband-tasks/en/stable/) library.

## Contributing
I'd love to see contributions from the VLBI community, but if you have any suggestions or questions you can always reach out to me directly.

## Licensing
This library is licensed under either the MIT License or the Apache 2.0 License at your option.