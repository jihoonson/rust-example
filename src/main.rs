#![feature(alloc)]
#![feature(heap_api)]
extern crate alloc;
extern crate rand;

pub mod qsort_example;
pub mod unsafe_example;

use qsort_example::{qsort, parallel_qsort};

fn main() {
  let p = unsafe_example::my_allocate();
  unsafe_example::assign_and_print(p);
  // unsafe_example::my_deallocate(p);
}

// fn main() {
//     // let mut v = [10, 1, 3, 5, 2, 10];
//     let mut v = (0..1000000).map(|_| rand::random::<i32>()).collect::<Vec<i32>>();
//
//     // qsort(&mut v);
//     parallel_qsort(&mut v);
//
//     // println!("{:?}", v);
// }
