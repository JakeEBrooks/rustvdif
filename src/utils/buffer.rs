use std::{mem::ManuallyDrop, sync::{atomic::{AtomicUsize, Ordering}, Arc}};

use crossbeam_utils::CachePadded;

use crate::VDIFFrame;

/// A high performance lock-free SPSC queue for safely transferring VDIF frames between threads.
/// 
/// The implementation is heavily based on the [rtrb](https://docs.rs/rtrb/latest/rtrb/) crate, but fixed to contain only `u32`s in segments of 
/// `frame_size`.
pub struct VDIFFIFO {
    frame_head: CachePadded<AtomicUsize>,
    frame_tail: CachePadded<AtomicUsize>,
    data_ptr: *mut u32,
    frame_len: usize,
    frame_cap: usize
}

impl VDIFFIFO {
    /// Create a new buffer with an associated [`VDIFProducer`] and [`VDIFConsumer`]. 
    #[allow(clippy::new_ret_no_self)]
    #[allow(clippy::arc_with_non_send_sync)]
    pub fn new(frame_cap: usize, frame_size: usize) -> (VDIFProducer, VDIFConsumer) {
        let frame_len = frame_size / 4;
        let buffer = Arc::new(VDIFFIFO {
            frame_head: CachePadded::new(AtomicUsize::new(0)),
            frame_tail: CachePadded::new(AtomicUsize::new(0)),
            data_ptr: ManuallyDrop::new(Vec::with_capacity(frame_cap*frame_len)).as_mut_ptr(),
            frame_len,
            frame_cap
        });

        let p = VDIFProducer {
            buffer: buffer.clone()
        };

        let c = VDIFConsumer {
            buffer,
        };

        return (p, c)
    }

    /// Get the capacity of this buffer in number of frames.
    pub fn frame_capacity(&self) -> usize {
        return self.frame_cap
    }

    fn collapse_position(&self, pos: usize) -> usize {
        debug_assert!(pos == 0 || pos < 2 * self.frame_cap);
        if pos < self.frame_cap {
            return pos
        } else {
            return pos - self.frame_cap
        }
    }

    /// Get an unsafe mutable pointer to the frame slot indicated by `pos`.
    /// 
    /// # Safety
    /// Ensure that when modifying data at this location, any modifications are restricted to the size of a single frame. Any more than that
    /// and the contents of the buffer may be corrupted.
    /// 
    /// In release builds, this method will cause undefined behaviour if `pos` is larger than twice the frame capacity of the buffer.
    pub unsafe fn frame_ptr(&self, pos: usize) -> *mut u32 {
        debug_assert!(pos == 0 || pos < 2 * self.frame_cap);
        let pos = self.collapse_position(pos);
        return unsafe { self.data_ptr.add(pos*self.frame_len) }
    }

    fn increment(&self, pos: usize) -> usize {
        debug_assert_ne!(self.frame_cap, 0);
        debug_assert!(pos < 2 * self.frame_cap);
        if pos < 2 * self.frame_cap - 1 {
            return pos + 1
        } else {
            return 0
        }
    }

    fn distance(&self, a: usize, b: usize) -> usize {
        debug_assert!(a == 0 || a < 2 * self.frame_cap);
        debug_assert!(b == 0 || b < 2 * self.frame_cap);
        if a <= b {
            return b - a
        } else {
            return 2 * self.frame_cap - a + b
        }
    }
}

impl Drop for VDIFFIFO {
    fn drop(&mut self) {
        // Since unlike rtrb, VDIFFIFO is just a block of u32's, we don't need a more complicated Drop implementation
        unsafe { Vec::from_raw_parts(self.data_ptr, 0, self.frame_cap*self.frame_len) };
    }
}

/// The producer part of a [`VDIFFIFO`] buffer.
pub struct VDIFProducer {
    buffer: Arc<VDIFFIFO>
}

impl VDIFProducer {
    /// Attempt to push data onto the buffer, returning an empty [`Some`] value if the buffer is full.
    pub fn try_push_from(&mut self, src: &[u32]) -> Option<()> {
        debug_assert!(src.len() == self.buffer.frame_len);
        if let Some(tail) = self.next_tail() {
            unsafe { self.buffer.frame_ptr(tail).copy_from(src.as_ptr(), src.len()) };
            let tail = self.buffer.increment(tail);
            self.buffer.frame_tail.store(tail, Ordering::Release);
            return None
        } else {
            return Some(())
        }
    }

    /// Attempt to push a [`VDIFFrame`] onto the buffer, returning the same frame as [`Some`] if the buffer is full.
    pub fn try_push(&mut self, frame: VDIFFrame) -> Option<VDIFFrame> {
        return self.try_push_from(frame.as_slice()).map(|_| return frame)
    }

    fn next_tail(&self) -> Option<usize> {
        let head = self.buffer.frame_head.load(Ordering::Acquire);
        let tail = self.buffer.frame_tail.load(Ordering::Acquire);
        if self.buffer.distance(head, tail) == self.buffer.frame_cap {
            return None
        }
        return Some(tail)
    }
}

unsafe impl Send for VDIFProducer {}

/// The consumer part of a [`VDIFFIFO`] buffer.
pub struct VDIFConsumer {
    buffer: Arc<VDIFFIFO>
}

impl VDIFConsumer {
    /// Attempt to pop data from the buffer, returning [`None`] if the buffer is empty.
    pub fn try_pop_to(&mut self, dest: &mut [u32]) -> Option<()> {
        debug_assert!(dest.len() == self.buffer.frame_len);
        if let Some(head) = self.next_head() {
            unsafe { dest.as_mut_ptr().copy_from(self.buffer.frame_ptr(head), dest.len()) };
            let head = self.buffer.increment(head);
            self.buffer.frame_head.store(head, Ordering::Release);
            return Some(())
        } else {
            return None
        }
    }

    /// Attempt to pop a [`VDIFFrame`] from the buffer, returning [`None`] if the buffer is empty.
    pub fn try_pop(&mut self) -> Option<VDIFFrame> {
        let mut frame = VDIFFrame::new_empty(self.buffer.frame_len*4);
        self.try_pop_to(frame.as_mut_slice())?;
        return Some(frame)
    }

    fn next_head(&self) -> Option<usize> {
        let head = self.buffer.frame_head.load(Ordering::Acquire);
        let tail = self.buffer.frame_tail.load(Ordering::Acquire);
        if head == tail {
            return None
        } else {
            return Some(head)
        }
    }
}

unsafe impl Send for VDIFConsumer {}