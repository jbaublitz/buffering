#![deny(missing_docs)]

//! # Approach
//! This crate is meant to provide a macro which behaves as follows:
//! * The provided macro uses a more restrictive approach. It uses procedural macros and
//! unions to provide some level of safety when writing to fields in a struct while allowing the
//! underlying struct to be interpreted as a slice. This is a C-like workflow but does provide
//! some helpful guarantees that come with Rust like protection against buffer overflows and
//! bounds checking. One very important note is that structs that use the provided
//! procedural macro must be completely stack allocated. Compilation will fail if certain
//! constructs that prevent size computation at compile time are used. As a result this really
//! should only be used as a Rust substitute with some additional safety for the common C workflow
//! when doing things like parsing network packets.

pub use buffering_nocopy_macro::NoCopy;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_proc_macro() {
        #[derive(Copy, Clone, NoCopy)]
        #[repr(C)]
        #[nocopy_macro(name = "TestBufferThing")]
        pub struct Test {
            test: u8,
        }

        let mut tb = TestBufferThing::new_buffer([0]);
        tb.set_test(5);
        assert_eq!(tb.as_buffer(), [5]);

        #[derive(Copy, Clone, NoCopy)]
        #[repr(C)]
        #[nocopy_macro(name = "TestBufferThingTwo", endian = "big")]
        pub struct TestTwo {
            test: u16,
        }

        let tb = TestBufferThingTwo::new_buffer([0, 5]);
        assert_eq!(tb.get_test(), 5);

        #[derive(Copy, Clone, NoCopy)]
        #[repr(C)]
        #[nocopy_macro(name = "TestBufferThingThree", endian = "little")]
        pub struct TestThree {
            test: u16,
        }

        let tb = TestBufferThingThree::new_buffer([5, 0]);
        assert_eq!(tb.get_test(), 5);
    }
}
