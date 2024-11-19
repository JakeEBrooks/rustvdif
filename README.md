# Rust VDIF

A rust crate for interacting with data encoded in the VLBI Data Interchange Format (VDIF), commonly used in radio astronomy experiments. The VDIF data format is defined in the VDIF specification, found [here](https://vlbi.org/vlbi-standards/vdif/).

This is a minimalist crate designed to relieve the problem of dealing with VDIF data in your own applications.

With `rustvdif` you can:

- Read VDIF frames from and write to various sources, including files, TCP Streams and UDP Sockets.
- Easily access fields within a VDIF header.
- Access VDIF payload data in `u32` or byte form.
- Encode and decode VDIF payloads, with up to 16 bits/sample.

Documentation is available [here](https://docs.rs/rustvdif/latest/rustvdif/).

## Usage

Reading VDIF frames is made easy by wrapping around types implementing the Rust [Read](https://doc.rust-lang.org/std/io/trait.Read.html) trait.

For example, frames can be easily read from a file:

```rust
fn main() {
    // A file of 8032 byte VDIF frames
    let mut file = VDIFReader::open("path/to/my/vdif", 8032).unwrap();
    // Read the first 100 frames and print header information on each one
    for _ in 0..100 {
        let frame = file.read_frame().unwrap();
        println!("{}", frame.get_header());
    }
}
```

## Contributing

I'd love to see contributions from the VLBI community, and if you have any suggestions or questions you can always reach out to me directly or open an issue.

## Licensing

This library is licensed under either the MIT License or the Apache 2.0 License at your option.
