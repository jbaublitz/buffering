#![feature(custom_attribute)]

#[cfg(feature = "nocopy")]
#[macro_use]
#[allow(unused_imports)]
extern crate buffering_nocopy_macro;

#[cfg(feature = "copy")]
pub mod copy;

#[cfg(feature = "copy")]
pub use copy::*;

#[cfg(test)]
mod test {
    use std::mem;

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

        #[derive(Copy,Clone,NoCopy)]
        #[name = "TestBufferThingTwo"]
        pub struct TestTwo {
            #[endian = "big"]
            test: u16,
        }

        let tb = TestBufferThingTwo::new_buffer([0, 5]);
        assert_eq!(tb.get_test(), 5);

        #[derive(Copy,Clone,NoCopy)]
        #[name = "TestBufferThingThree"]
        pub struct TestThree {
            #[endian = "little"]
            test: u16,
        }

        let tb = TestBufferThingThree::new_buffer([5, 0]);
        assert_eq!(tb.get_test(), 5);
    }
}
