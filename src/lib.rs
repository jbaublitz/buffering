#![feature(custom_attribute)]

#[macro_use]
extern crate nocopy_macro;

pub mod copy;

pub use copy::*;

#[cfg(test)]
mod test {
    #[test]
    fn test_proc_macro() {
        #[derive(Copy,Clone,NoCopy)]
        #[name = "TestBuffer"]
        pub struct Test {
            test: u8,
        }

        let tb = TestBuffer::new_buffer([0]);
    }
}
