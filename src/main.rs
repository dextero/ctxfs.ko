#![allow(ctypes)]
#![no_std]
#![feature(intrinsics)]
#![feature(lang_items)]

#[lang="sized"]
trait Sized {}

extern "rust-intrinsic" {
    pub fn transmute<T, U>(val: T) -> U;
}

extern {
    pub fn printk(fmt: *mut u8);
}

unsafe fn print(s: &str) {
    let (ptr, _): (*mut u8, uint) = transmute(s);
    printk(ptr);
}

#[no_mangle]
pub extern fn rustfs_module_init() {
    unsafe {
        print("hello\n");
    }
}

#[no_mangle]
pub extern fn rustfs_module_exit() {
    unsafe {
        print("goodbye\n");
    }
}
