use std::net::UdpSocket;

use rustvdif::{net::{recv_frame, send_frame}, VDIFFrame};

fn main() {
    // Create a UDP socket that we'll use to send our frames
    let sendsock = UdpSocket::bind("0.0.0.0:8000").unwrap();
    // Shift the sending operation into its own thread
    std::thread::spawn(move || {
        // Connect to the loopback interface
        sendsock.connect("127.0.0.1:9096").unwrap();
        // We're going to send 10 VDIF frames, incrementing the frame counter by one each time
        for frameno in 0u32..10u32 {
            // Create a new empty frame (i.e. just a bunch of zeros)
            let mut frame = VDIFFrame::new_empty(1024);

            // Set the header parameters we want
            frame.set_time(42);
            frame.set_ref_epoch(6);
            frame.set_frameno(frameno);
            frame.set_log2channels(0); // single channel
            frame.set_size8(1024/8);
            frame.set_real(true);
            frame.set_bits_per_sample_1(1); // 2 bit

            // Send the frame on the socket
            send_frame(&sendsock, &frame).unwrap();
        }
    });

    // Create a UDP socket to receive the frames, bind it to the loopback interface
    let recvsock = UdpSocket::bind("127.0.0.1:9096").unwrap();

    // Receive 10 frames and print them
    for _ in 0..10 {
        println!("{}", recv_frame(&recvsock, 1024).unwrap())
    }
}