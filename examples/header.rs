use rustvdif::{VDIFFrame, VDIFHeader};

fn main() {
    // Often you want to create a stream of VDIF data but you don't want to have to specify
    // the exact contents of the header for every frame. With rustvdif, you can set up a header template
    // and then modify only the bits you want to change for each frame.

    // To do this we first create our template header:
    let hdr = VDIFHeader::new()
        .ref_epoch(30)
        .log2channels(2)
        .size8(1004)
        .real(true)
        .bits_per_sample_1(3)
        .station(1234);

    for f in 0..10 {
        // Then we can construct a frame in every loop by modifying only the bit of the header that changes
        // We can do the same for any combination of header fields!
        let newframe = VDIFFrame::from_header(hdr.frameno(f));
        println!("{}", newframe)
    }

    // This should print:
    // <Valid: true, Time: 0, Epoch: 30, Frame: 0, Chans: 4, Size: 8032, Real: true, Bits per sample: 4, Thread: 0, Station: 1234>
    // <Valid: true, Time: 0, Epoch: 30, Frame: 1, Chans: 4, Size: 8032, Real: true, Bits per sample: 4, Thread: 0, Station: 1234>
    // <Valid: true, Time: 0, Epoch: 30, Frame: 2, Chans: 4, Size: 8032, Real: true, Bits per sample: 4, Thread: 0, Station: 1234>
    // <Valid: true, Time: 0, Epoch: 30, Frame: 3, Chans: 4, Size: 8032, Real: true, Bits per sample: 4, Thread: 0, Station: 1234>
    // <Valid: true, Time: 0, Epoch: 30, Frame: 4, Chans: 4, Size: 8032, Real: true, Bits per sample: 4, Thread: 0, Station: 1234>
    // <Valid: true, Time: 0, Epoch: 30, Frame: 5, Chans: 4, Size: 8032, Real: true, Bits per sample: 4, Thread: 0, Station: 1234>
    // <Valid: true, Time: 0, Epoch: 30, Frame: 6, Chans: 4, Size: 8032, Real: true, Bits per sample: 4, Thread: 0, Station: 1234>
    // <Valid: true, Time: 0, Epoch: 30, Frame: 7, Chans: 4, Size: 8032, Real: true, Bits per sample: 4, Thread: 0, Station: 1234>
    // <Valid: true, Time: 0, Epoch: 30, Frame: 8, Chans: 4, Size: 8032, Real: true, Bits per sample: 4, Thread: 0, Station: 1234>
    // <Valid: true, Time: 0, Epoch: 30, Frame: 9, Chans: 4, Size: 8032, Real: true, Bits per sample: 4, Thread: 0, Station: 1234>
}