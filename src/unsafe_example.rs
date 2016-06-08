use alloc::heap;

pub fn my_allocate() -> *mut u8{
  unsafe {
      heap::allocate(10, 10)
  }
}

pub fn assign_and_print(p: *mut u8) {
  unsafe {
    *p = 10;
    println!("{}", *p);
  }
}

pub fn my_deallocate(p: *mut u8) {
  unsafe {
    heap::deallocate(p, 10, 10);
  }
}
