#![allow(unused_macros)]

macro_rules! struct_and_union {
    ( $name:ident, $struct_name:ident, $( $field_name:ident : $type:ty ),* ) => (
        #[derive(Debug,PartialEq,Copy,Clone)]
        #[repr(C,packed)]
        pub struct $struct_name {
            $( $field_name : $type ),*
        }

        pub union $name {
            structure: $struct_name,
            buffer: [u8; mem::size_of::<$struct_name>()],
        }
    );
}

macro_rules! impl_funcs {
    ( $name:ident, $struct_name:ident ) => (
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
    );
}

macro_rules! impl_sub_macro {
    ( $name:ident, $struct_name:ident, $( $field_name:ident : $type:ty ),* => $( $field_conv:ident => $conv_type:ident ),* ) => (
        mashup! {
            $(
                getset["get" $field_name] = get_ $field_name;
                getset["set" $field_name] = set_ $field_name;
            )*
            $(
                getset["getfrom" $field_conv] = get_from_ $field_conv;
                getset["setinto" $field_conv] = set_into_ $field_conv;
            )*
        }

        getset! {
            impl $name {
                impl_funcs!( $name, $struct_name );

                $(
                    pub fn "get" $field_name(&self) -> &$type {
                        unsafe { &self.structure.$field_name }
                    }

                    pub fn "set" $field_name(&mut self, val: $type) {
                        unsafe { self.structure.$field_name = val };
                    }
                )*
                $(
                    pub fn "getfrom" $field_conv(&self) -> $conv_type {
                        unsafe { $conv_type::from(self.structure.$field_conv) }
                    }

                    pub fn "setinto" $field_conv(&mut self, val: $conv_type) {
                        unsafe { self.structure.$field_conv = val.into() };
                    }
                )*
            }
        }
    );
    ( $name:ident, $struct_name:ident, $( $field_name:ident : $type:ty ),* ) => (
        mashup! {
            $(
                getset["get" $field_name] = get_ $field_name;
                getset["set" $field_name] = set_ $field_name;
            )*
        }

        getset! {
            impl $name {
                impl_funcs!( $name, $struct_name );

                $(
                    pub fn "get" $field_name(&self) -> &$type {
                        unsafe { &self.structure.$field_name }
                    }

                    pub fn "set" $field_name(&mut self, val: $type) {
                        unsafe { self.structure.$field_name = val };
                    }
                )*
            }
        }
    );
}

macro_rules! as_ref {
    ( $name:ident ) => (
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
    );
}

#[macro_export]
macro_rules! struct_buffer {
    ( $name:ident, $struct_name:ident, $( $field_name:ident : $type:ty ),* => $( $field_conv:ident => $conv_type:ident ),* ) => (
        struct_and_union!( $name, $struct_name, $( $field_name: $type ),* );

        impl_sub_macro!( $name, $struct_name, $( $field_name: $type ),* => $( $field_conv => $conv_type ),* );

        as_ref!( $name );
    );
    ( $name:ident, $struct_name:ident, $( $field_name:ident : $type:ty ),*) => (
        struct_and_union!( $name, $struct_name, $( $field_name: $type ),* );

        impl_sub_macro!( $name, $struct_name, $( $field_name: $type ),* );

        as_ref!( $name );
    );
}

#[cfg(test)]
mod test {
    use std::mem;

    #[test]
    #[allow(dead_code)]
    fn test_macro() {
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

    #[test]
    #[allow(dead_code)]
    fn test_macro_conv() {
        #[derive(Debug,PartialEq)]
        pub enum TestEnum {
            One,
            Two,
        }

        impl From<u8> for TestEnum {
            fn from(v: u8) -> Self {
                match v {
                    1 => TestEnum::One,
                    2 => TestEnum::Two,
                    _ => panic!(),
                }
            }
        }

        impl Into<u8> for TestEnum {
            fn into(self) -> u8 {
                match self {
                    TestEnum::One => 1,
                    TestEnum::Two => 2,
                }
            }
        }

        struct_buffer!(MyNetworkPacketUnion, MyNetworkPacket, field1: u8, field2: u16, field3: [u8; 4] => field1 => TestEnum);

        let un = MyNetworkPacketUnion::new_struct(MyNetworkPacket {
            field1: 1, field2: 2, field3: [1, 2, 3, 4]
        });
        assert_eq!(un.get_from_field1(), TestEnum::One);
    }
}
