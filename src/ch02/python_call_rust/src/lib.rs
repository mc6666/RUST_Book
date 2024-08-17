//! EMBEDDING RUST IN PYTHON
//! Examples on how to write Rust libraries callable by a Python script.
//!
//! Author: Davide Aversa <thek3nger@gmail.com>
//! ---------------------------------------------------------------------------


/// Libc provides several C-friendly data types. This is used to write 
/// functions callable by an external languages **or** to call external
///C libraries.  
extern crate libc;

/// Use this to load in the namespace the `size_t` and `int32_t` types.
use libc::{size_t};

/// Use this to load the standard library interface to threading.
use std::thread;

/// Use this to work with Rust slices.
use std::slice;

/// The first function we want to try is a simple no-input/no-outut function.
///
/// The algorithm spawns 10 threads, and each one will increas a counter
/// 5 milion times in order to simulate some work. At the end the function
/// waits for the thread completitions and retun nothing. 
#[no_mangle]
pub extern fn process() {
    let handles: Vec<_> = (0..10).map(|_| {
        thread::spawn(|| {
            let mut x = 0;
            for _ in 0..5_000_000 {
                x += 1
            }
            x
        })
    }).collect();

    for h in handles {
        println!("Thread finished with count={}",
        h.join().map_err(|_| "Could not join a thread!").unwrap());
    }
}

/// Now we want to pass to Rust a Python's list. We cannot do that directly
/// so we need a bit more magic.
#[no_mangle]
pub extern fn sum_list(data: *const i32, length: size_t) -> i32 {
    let nums = unsafe { slice::from_raw_parts(data, length as usize) };
    nums.iter().fold(0, |acc, i| acc + i)
}

/// STUCTS AND OBJECTS

/// We want to represtent a 2D point as a Rust structure and move this point
/// between Python and Rust.

#[repr(C)]
pub struct Point {
    x: f64,
    y: f64,
}

/// Now we write a function who compute the middle point of a segment
/// represented by two Point.
#[no_mangle]
pub extern fn middle(p1: Point, p2: Point) -> Point {
    Point { x: (p1.x + p2.x)/2.0, y: (p1.y + p2.y)/2.0 }
} 