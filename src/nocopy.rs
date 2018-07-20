#[macro_export]
macro_rules! struct_buffer {
    ( $name:ident, $struct_name:ident, $( $field_name:ident : $type:ty ),* ) => {
        #[derive(Debug,PartialEq,Copy,Clone)]
        #[repr(C,packed)]
        pub struct $struct_name {
            $( $field_name: $type ),*
        }

        pub union $name {
            structure: $struct_name,
            buffer: [u8; mem::size_of::<$struct_name>()],
        }

        impl $name {
            pub fn new_buffer(buf: [u8; mem::size_of::<$struct_name>()]) -> Self {
                $name { buffer: buf }
            }

            pub fn get_buffer(&self) -> &[u8; mem::size_of::<$struct_name>()] {
                unsafe { &self.buffer }
            }

            pub fn new_struct(v: $struct_name) -> Self {
                $name { structure: v }
            }

            pub fn get_struct(&self) -> &$struct_name {
                unsafe { &self.structure }
            }

            pub fn get_struct_mut(&mut self) -> &mut $struct_name {
                unsafe { &mut self.structure }
            }
        }

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &[u8] {
                unsafe { &self.buffer }
            }
        }

        impl AsMut<[u8]> for $name {
            fn as_mut(&mut self) -> &mut [u8] {
                unsafe { &mut self.buffer }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::mem;

    #[test]
    fn test_macro_succeed() {
        struct_buffer!(MyNetworkPacketUnion, MyNetworkPacket, field1: u8, field2: u16, field3: [u8; 4]);

        let mut un = MyNetworkPacketUnion::new_struct(MyNetworkPacket {
            field1: 1, field2: 2, field3: [1, 2, 3, 4]
        });
        assert_eq!(un.get_buffer(), &[1, 2, 0, 1, 2, 3, 4]);
        un.get_struct_mut().field2 = 5;
        assert_eq!(un.get_buffer(), &[1, 5, 0, 1, 2, 3, 4]);

        let un = MyNetworkPacketUnion::new_buffer([1, 2, 0, 1, 2, 3, 4]);
        assert_eq!(un.get_struct(), &MyNetworkPacket {
            field1: 1, field2: 2, field3: [1, 2, 3, 4]
        });
    }
}
