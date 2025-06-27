use std::time::Duration;

use rustvdif::{utils::VDIFFIFO, VDIFFrame, VDIFHeader};

fn main() {
    // In certain applications, you want to process VDIF data very quickly 
    // and distribute the workload among many cores of your CPU.
    
    // Generally people will use 'Mutexes' (e.g. Rust's std::sync::Mutex, see this stack overflow answer for a good explanation: https://stackoverflow.com/questions/34524/what-is-a-mutex)
    // for passing/accessing data between threads, but in HPC applications Mutexes are just too slow.

    // An alternative solution is to use Atomic primitives to index into a shared memory resource - no mutexes involved.
    // This is part of the broader field of 'lock-free' programming.
    
    // This crate implements a lock-free single-producer, single-consumer (SPSC) ring buffer for sharing VDIF frames between two threads.

    // First, we create our ring buffer to hold 10 frames of size 8032 bytes.
    let (mut prod, mut cons) = VDIFFIFO::new(100, 8032);

    // We're going to spawn a thread to add data to the buffer, and pull it out in the main thread:
    std::thread::spawn(move || {
        // Make a template VDIFHeader we'll modify on the fly
        let hdr = VDIFHeader::new()
            .ref_epoch(30)
            .log2channels(2)
            .size8(1004)
            .real(true)
            .bits_per_sample_1(3)
            .station(1234)
            .thread(1);

        // Now, we're going to send 50 frames to the buffer
        for f in 0..50 {
            // Construct the frame with the right Frame number field
            let frame = VDIFFrame::from_header(hdr.frameno(f));
            // And then send it to the buffer
            let _ = prod.try_push(frame);
        }

    });

    // Ok now we want to read the frames we are sending above in the main thread.
    for _ in 0..50 {
        // The computation in this thread is slightly faster because we're not making the header,
        // so introduce a slight delay so the other thread can keep pace
        std::thread::sleep(Duration::from_millis(10));

        // Attempt to get a frame from the buffer
        let res = cons.try_pop();
        // But because we're reading from a different thread, there's no guarantee the data is there
        // So if the buffer is empty, print that, otherwise print some info about the frame.
        if let Some(frame) = res {
            println!("{}", frame)
        } else {
            println!("No data in the buffer")
        }
    }
}