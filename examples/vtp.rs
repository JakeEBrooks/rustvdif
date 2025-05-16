use std::net::UdpSocket;

use rustvdif::{encoding::header::{encode_bits_per_sample_1, encode_frameno, encode_is_real, encode_log2channels, encode_ref_epoch, encode_size8, encode_time}, net::udp::{recv_frame, send_frame}, VDIFFrame};

fn main() {
    // Create a UDP socket that we'll use to send our frames
    let sendsock = UdpSocket::bind("0.0.0.0:8000").unwrap();
    // Shoft the sending operation into its own thread
    std::thread::spawn(move || {
        // Connect to the loopback interface
        sendsock.connect("127.0.0.1:9096").unwrap();
        // We're going to send 10 VDIF frames, incrementing the frame counter by one each time
        for frameno in 0u32..10u32 {
            // Create a new empty frame (i.e. just a bunch of zeros)
            let mut frame = VDIFFrame::new_empty(1024);

            // Set the header parameters we want
            encode_time(&mut frame, 42);
            encode_ref_epoch(&mut frame, 6);
            encode_frameno(&mut frame, frameno);
            encode_log2channels(&mut frame, 0); // single channel
            encode_size8(&mut frame, 1024/8);
            encode_is_real(&mut frame, true);
            encode_bits_per_sample_1(&mut frame, 1); // 2 bit

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