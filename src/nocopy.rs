#[macro_export]
macro_rules! struct_buffer {
    ( $name:ident, $struct_name:ident, $( $field_name:ident : $type:ty ),* ) => {
        #[derive(Copy,Clone)]
        #[repr(C,packed)]
        pub struct $struct_name {
            $( $field_name: $type ),*
        }

        pub union $name {
            structure: $struct_name,
            buffer: [u8; mem::size_of::<$struct_name>()],
        }
    }
}

#[cfg(test)]
mod test {
    use std::mem;

    #[test]
    fn test_macro_succeed() {
        struct_buffer!(MyNetworkPacketUnion, MyNetworkPacket, field1: u8, field2: u16, field3: [u8; 4]);

        let un = MyNetworkPacketUnion {
            structure: MyNetworkPacket { field1: 1, field2: 2, field3: [1, 2, 3, 4] }
        };
        assert_eq!(unsafe { un.buffer }, [1, 2, 0, 1, 2, 3, 4]);
    }
}
