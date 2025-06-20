use std::io::{Cursor, Seek};

use rustvdif::{read_frame, write_frame, VDIFFrame};

fn main() {
    // A Cursor<Vec<u8>> behaves exactly the same as a file. When you're reading VDIF data from
    // a file, you would simply replace the line below with something like:
    //
    // let mut file = std::fs::File::open("/path/to/my/file").unwrap();
    //
    let mut file = Cursor::new(Vec::<u8>::new());
    
    // Write 10 invalid frames to the file
    for _ in 0..10 {
        let mut frame = VDIFFrame::new_empty(1032);
        // Let's mark the frame as invalid, since it is just an empty frame
        frame.set_valid(false);

        // Write the frame to the file
        let _ = write_frame(&mut file, frame).unwrap();
    }

    // Now we want to read back the frames we just wrote, so first call rewind()
    file.rewind().unwrap();

    for _ in 0..10 {
        // Now we read the i'th frame
        let frame = read_frame(&mut file, 1032).unwrap();

        // We marked the frame as invalid, so let's decode the 'valid' header field and print it
        println!("{}", frame.get_valid())
    }
}