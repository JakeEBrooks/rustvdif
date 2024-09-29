# Rust VDIF
A rust crate for interacting with data encoded in the VLBI Data Interchange Format (VDIF). The VDIF data format is defined in the VDIF specification, found [here](https://vlbi.org/vlbi-standards/vdif/).

This is a minimalist crate with a single dependency ([time](https://crates.io/crates/time)) designed to relieve the problem of dealing with VDIF data in your own applications.

# Usage
In `rustvdif`, any type implementing `std::io::Read` and `std::io::Seek` can be wrapped in the central `VDIFReader` type, like so:
```rust
let file = BufReader::new(File::open("my/vdif/file").unwrap());
let mut reader = VDIFReader::new(file).unwrap();
// Print the first 10 headers.
for i in 0..10 {
    let header = reader.get_header().unwrap();
    println!("{}", header);
    reader.next(&header).unwrap();
};
```
A `VDIFReader` interprets VDIF data as a continuous stream, which makes it easy to adapt this to networking scenarios.

# Wishlist
I'd like to add the following over time:

- Implement an easy to use TCP buffer.
- Implement an easy to use UDP buffer, which maintains the order of incoming frames as much as possible.
- Better API for EDV data.
- Better datetime interfaces for the header.
- Implement common operations for the payload such as those contained in the [baseband-tasks](https://baseband.readthedocs.io/projects/baseband-tasks/en/stable/) library.
- Tests!

## Contributing
I'd love to see contributions from the VLBI community, but if you have any suggestions or questions you can always reach out to me directly.

## Licensing
This library is licensed under either the MIT License or the Apache 2.0 License at your option.