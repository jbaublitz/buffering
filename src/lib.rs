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
        #[name = "TestBufferThing"]
        pub struct Test {
            test: u8,
        }

        let mut tb = TestBufferThing::new_buffer([0]);
        tb.set_test(5);
        assert_eq!(tb.as_buffer(), [5]);
    }
}
