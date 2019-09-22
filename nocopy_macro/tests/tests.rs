#[macro_use]
extern crate buffering_nocopy_macro;

#[test]
fn test_union() {
    #[derive(Copy, Clone, NoCopy)]
    #[nocopy_macro(endian = "big")]
    #[repr(C)]
    pub struct TestDefault {
        one: u8,
        two: u16,
        three: u32,
        four: u64,
        five: [u8; 10],
    }

    let mut test_big = TestDefaultBuffer::new_buffer([0; std::mem::size_of::<TestDefault>()]);
    test_big.set_one(1);
    test_big.set_two(20);
    test_big.set_three(50);
    test_big.set_four(50);
    test_big.set_five([1; 10]);

    assert_eq!(test_big.get_one(), 1);
    assert_eq!(test_big.get_two(), 20);
    assert_eq!(test_big.get_three(), 50);
    assert_eq!(test_big.get_four(), 50);
    assert_eq!(test_big.get_five(), [1; 10]);
    assert_eq!(
        test_big.as_buffer(),
        [
            1, 0, 0, 20, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 50, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0,
            0, 0, 0, 0
        ]
    )
}
