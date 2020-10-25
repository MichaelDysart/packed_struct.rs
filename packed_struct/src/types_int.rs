
use internal_prelude::v1::*;

use super::types_bits::*;

use super::packing::{PackingError, PackedStruct, PackedStructInfo, PackedStructSlice};

use serde::ser::{Serialize, Serializer};
use serde::de::{Deserialize, Deserializer};

use super::types_num::*;

number_type!(Integer, SizedInteger, IntegerAsBytes, MsbInteger, LsbInteger);

macro_rules! integer_as_bytes {
    ($T: ident, $N: tt) => {
        impl IntegerAsBytes for $T {
            type AsBytes = [u8; $N];

            #[inline]
            fn to_msb_bytes(&self) -> [u8; $N] {
                as_bytes_msb!($N, self)
            }

            #[inline]
            fn to_lsb_bytes(&self) -> [u8; $N] {
                as_bytes_lsb!($N, self)
            }

            #[inline]
            fn from_msb_bytes(bytes: &[u8; $N]) -> Self {
                from_bytes_msb!($N, bytes, $T)
            }

            #[inline]
            fn from_lsb_bytes(bytes: &[u8; $N]) -> Self {
                from_bytes_lsb!($N, bytes, $T)
            }
        }
    };
}

integer_as_bytes!(u8, 1);
integer_as_bytes!(i8, 1);

integer_as_bytes!(u16, 2);
integer_as_bytes!(i16, 2);

integer_as_bytes!(u32, 4);
integer_as_bytes!(i32, 4);

integer_as_bytes!(u64, 8);
integer_as_bytes!(i64, 8);

macro_rules! integer_bytes_impl {
    ($T: ident, $TB: ident) => {
        impl SizedInteger<$T, $TB> for Integer<$T, $TB> {
            #[inline]
            fn value_bit_mask() -> $T {
                ones($TB::number_of_bits() as u64) as $T
            }

            #[inline]
            fn from_primitive(val: $T) -> Self {
                let v = val & Self::value_bit_mask();
                Integer { num: v, bits: Default::default() }
            }

            #[inline]
            fn to_primitive(&self) -> $T {
                self.num
            }

            #[inline]
            fn to_msb_bytes(&self) -> <<$TB as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes
            {
                let mut ret: <<$TB as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes = Default::default();
                let b = self.num.to_msb_bytes();
                let skip = b.len() - ret.len();
                ret.copy_from_slice(&b[skip..]);
                ret
            }

            #[inline]
            fn to_lsb_bytes(&self) -> <<$TB as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes
            {
                let mut ret: <<$TB as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes = Default::default();
                let b = self.num.to_lsb_bytes();
                let take = ret.len();
                ret.copy_from_slice(&b[0..take]);
                ret
            }

            #[inline]
            fn from_msb_bytes(bytes: &<<$TB as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes) -> Self
            {
                let mut native_bytes = Default::default();
                {
                    // hack that infers the size of the native array...
                    <$T>::from_msb_bytes(&native_bytes);
                }
                let skip = native_bytes.len() - bytes.len();
                {
                    let native_bytes = &mut native_bytes[skip..];
                    native_bytes.copy_from_slice(&bytes[..]);
                }
                let v = <$T>::from_msb_bytes(&native_bytes);
                Self::from_primitive(v)
            }

            #[inline]
            fn from_lsb_bytes(bytes: &<<$TB as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes) -> Self
            {
                let mut native_bytes = Default::default();
                {
                    // hack that infers the size of the native array...
                    <$T>::from_lsb_bytes(&native_bytes);
                }

                {
                    let take = bytes.len();
                    let native_bytes = &mut native_bytes[..take];
                    native_bytes.copy_from_slice(&bytes[..]);
                }

                let v = <$T>::from_lsb_bytes(&native_bytes);
                Self::from_primitive(v)
            }
        }

        impl From<$T> for Integer<$T, $TB> {
            fn from(v: $T) -> Self {
                Self::from_primitive(v)
            }
        }

        impl From<Integer<$T, $TB>> for $T {
            fn from(v: Integer<$T, $TB>) -> Self {
                v.to_primitive()
            }
        }

        impl Deref for Integer<$T, $TB> {
            type Target = $T;

            fn deref(&self) -> &$T {
                &self.num
            }
        }
    };
}

macro_rules! bytes1_impl {
    ($T: ident) => {
        integer_bytes_impl!($T, Bits1);
        integer_bytes_impl!($T, Bits2);
        integer_bytes_impl!($T, Bits3);
        integer_bytes_impl!($T, Bits4);
        integer_bytes_impl!($T, Bits5);
        integer_bytes_impl!($T, Bits6);
        integer_bytes_impl!($T, Bits7);
        integer_bytes_impl!($T, Bits8);
    };
}

macro_rules! bytes2_impl {
    ($T: ident) => {
        integer_bytes_impl!($T, Bits9);
        integer_bytes_impl!($T, Bits10);
        integer_bytes_impl!($T, Bits11);
        integer_bytes_impl!($T, Bits12);
        integer_bytes_impl!($T, Bits13);
        integer_bytes_impl!($T, Bits14);
        integer_bytes_impl!($T, Bits15);
        integer_bytes_impl!($T, Bits16);
    };
}

macro_rules! bytes3_impl {
    ($T: ident) => {
        integer_bytes_impl!($T, Bits17);
        integer_bytes_impl!($T, Bits18);
        integer_bytes_impl!($T, Bits19);
        integer_bytes_impl!($T, Bits20);
        integer_bytes_impl!($T, Bits21);
        integer_bytes_impl!($T, Bits22);
        integer_bytes_impl!($T, Bits23);
        integer_bytes_impl!($T, Bits24);
    };
}

macro_rules! bytes4_impl {
    ($T: ident) => {
        integer_bytes_impl!($T, Bits25);
        integer_bytes_impl!($T, Bits26);
        integer_bytes_impl!($T, Bits27);
        integer_bytes_impl!($T, Bits28);
        integer_bytes_impl!($T, Bits29);
        integer_bytes_impl!($T, Bits30);
        integer_bytes_impl!($T, Bits31);
        integer_bytes_impl!($T, Bits32);
    };
}

macro_rules! bytes5_impl {
    ($T: ident) => {
        integer_bytes_impl!($T, Bits33);
        integer_bytes_impl!($T, Bits34);
        integer_bytes_impl!($T, Bits35);
        integer_bytes_impl!($T, Bits36);
        integer_bytes_impl!($T, Bits37);
        integer_bytes_impl!($T, Bits38);
        integer_bytes_impl!($T, Bits39);
        integer_bytes_impl!($T, Bits40);
    };
}

macro_rules! bytes6_impl {
    ($T: ident) => {
        integer_bytes_impl!($T, Bits41);
        integer_bytes_impl!($T, Bits42);
        integer_bytes_impl!($T, Bits43);
        integer_bytes_impl!($T, Bits44);
        integer_bytes_impl!($T, Bits45);
        integer_bytes_impl!($T, Bits46);
        integer_bytes_impl!($T, Bits47);
        integer_bytes_impl!($T, Bits48);
    };
}

macro_rules! bytes7_impl {
    ($T: ident) => {
        integer_bytes_impl!($T, Bits49);
        integer_bytes_impl!($T, Bits50);
        integer_bytes_impl!($T, Bits51);
        integer_bytes_impl!($T, Bits52);
        integer_bytes_impl!($T, Bits53);
        integer_bytes_impl!($T, Bits54);
        integer_bytes_impl!($T, Bits55);
        integer_bytes_impl!($T, Bits56);
    };
}

macro_rules! bytes8_impl {
    ($T: ident) => {
        integer_bytes_impl!($T, Bits57);
        integer_bytes_impl!($T, Bits58);
        integer_bytes_impl!($T, Bits59);
        integer_bytes_impl!($T, Bits60);
        integer_bytes_impl!($T, Bits61);
        integer_bytes_impl!($T, Bits62);
        integer_bytes_impl!($T, Bits63);
        integer_bytes_impl!($T, Bits64);
    };
}

bytes1_impl!(u8);
bytes1_impl!(i8);

bytes2_impl!(u16);
bytes2_impl!(i16);

bytes3_impl!(u32);
bytes3_impl!(i32);

bytes4_impl!(u32);
bytes4_impl!(i32);

bytes5_impl!(u64);
bytes5_impl!(i64);

bytes6_impl!(u64);
bytes6_impl!(i64);

bytes7_impl!(u64);
bytes7_impl!(i64);

bytes8_impl!(u64);
bytes8_impl!(i64);

#[test]
fn test_u8() {
    let byte: Integer<u8, Bits8> = 0.into();
    assert_eq!(0, *byte);
    assert_eq!(0xFF, <Integer<u8, Bits8>>::value_bit_mask());
}

#[test]
fn test_i8() {
    let byte: Integer<i8, Bits8> = 0.into();
    assert_eq!(0, *byte);
    assert_eq!(-1, <Integer<i8, Bits8>>::value_bit_mask());
}

#[test]
fn test_u16() {
    let val = 0xABCD;
    let num: Integer<u16, Bits16> = val.into();
    assert_eq!(val, *num);
    assert_eq!([0xAB, 0xCD], num.to_msb_bytes());
    assert_eq!([0xCD, 0xAB], num.to_lsb_bytes());
}

#[test]
fn test_i16() {
    let val = 0x7BCD;
    let num: Integer<i16, Bits16> = val.into();
    assert_eq!(val, *num);
    assert_eq!([0x7B, 0xCD], num.to_msb_bytes());
    assert_eq!([0xCD, 0x7B], num.to_lsb_bytes());
}

#[test]
fn test_u32() {
    let val = 0x4589ABCD;
    let num: Integer<u32, Bits32> = val.into();
    assert_eq!(val, *num);
    assert_eq!([0x45, 0x89, 0xAB, 0xCD], num.to_msb_bytes());
    assert_eq!([0xCD, 0xAB, 0x89, 0x45], num.to_lsb_bytes());
}

#[test]
fn test_i32() {
    let val = 0x4589ABCD;
    let num: Integer<i32, Bits32> = val.into();
    assert_eq!(val, *num);
    assert_eq!([0x45, 0x89, 0xAB, 0xCD], num.to_msb_bytes());
    assert_eq!([0xCD, 0xAB, 0x89, 0x45], num.to_lsb_bytes());
}

#[test]
fn test_u64() {
    let val = 0x1122334455667788;
    let num: Integer<u64, Bits64> = val.into();
    assert_eq!(val, *num);
    assert_eq!([0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88], num.to_msb_bytes());
    assert_eq!([0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11], num.to_lsb_bytes());
}

#[test]
fn test_i64() {
    let val = 0x1122334455667788;
    let num: Integer<i64, Bits64> = val.into();
    assert_eq!(val, *num);
    assert_eq!([0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88], num.to_msb_bytes());
    assert_eq!([0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11], num.to_lsb_bytes());
}

#[test]
fn test_roundtrip_u32() {
    let val = 0x11223344;
    let num: Integer<u32, Bits32> = val.into();
    let msb_bytes = num.to_msb_bytes();
    let from_msb = u32::from_msb_bytes(&msb_bytes);
    assert_eq!(val, from_msb);

    let lsb_bytes = num.to_lsb_bytes();
    let from_lsb = u32::from_lsb_bytes(&lsb_bytes);
    assert_eq!(val, from_lsb);
}

#[test]
fn test_roundtrip_i32() {
    let val = 0x11223344;
    let num: Integer<i32, Bits32> = val.into();
    let msb_bytes = num.to_msb_bytes();
    let from_msb = i32::from_msb_bytes(&msb_bytes);
    assert_eq!(val, from_msb);

    let lsb_bytes = num.to_lsb_bytes();
    let from_lsb = i32::from_lsb_bytes(&lsb_bytes);
    assert_eq!(val, from_lsb);
}

#[test]
fn test_roundtrip_u24() {
    let val = 0xCCBBAA;
    let num: Integer<u32, Bits24> = val.into();
    let msb_bytes = num.to_msb_bytes();
    assert_eq!([0xCC, 0xBB, 0xAA], msb_bytes);
    let from_msb = <Integer<u32, Bits24>>::from_msb_bytes(&msb_bytes);
    assert_eq!(val, *from_msb);

    let lsb_bytes = num.to_lsb_bytes();
    assert_eq!([0xAA, 0xBB, 0xCC], lsb_bytes);
    let from_lsb = <Integer<u32, Bits24>>::from_lsb_bytes(&lsb_bytes);
    assert_eq!(val, *from_lsb);
}

#[test]
fn test_roundtrip_u20() {
    let val = 0xFBBAA;
    let num: Integer<u32, Bits20> = val.into();
    let msb_bytes = num.to_msb_bytes();
    assert_eq!([0x0F, 0xBB, 0xAA], msb_bytes);
    let from_msb = <Integer<u32, Bits20>>::from_msb_bytes(&msb_bytes);
    assert_eq!(val, *from_msb);    
}

#[test]
fn test_packed_int_msb() {
    let val = 0xAABBCCDD;
    let typed: Integer<u32, Bits32> = val.into();
    let endian = typed.as_packed_msb();
    let packed = endian.pack();
    assert_eq!([0xAA, 0xBB, 0xCC, 0xDD], packed);
    
    let unpacked: MsbInteger<_, _, Integer<u32, Bits32>> = MsbInteger::unpack(&packed).unwrap();
    assert_eq!(val, **unpacked);
}

#[test]
fn test_packed_int_partial() {
    let val = 0b10_10101010;
    let typed: Integer<u16, Bits10> = val.into();
    let endian = typed.as_packed_msb();
    let packed = endian.pack();
    assert_eq!([0b00000010, 0b10101010], packed);
    
    let unpacked: MsbInteger<_, _, Integer<u16, Bits10>> = MsbInteger::unpack(&packed).unwrap();
    assert_eq!(val, **unpacked);
}

#[test]
fn test_packed_int_lsb() {
    let val = 0xAABBCCDD;
    let typed: Integer<u32, Bits32> = val.into();
    let endian = typed.as_packed_lsb();
    let packed = endian.pack();
    assert_eq!([0xDD, 0xCC, 0xBB, 0xAA], packed);
    
    let unpacked: LsbInteger<_, _, Integer<u32, Bits32>> = LsbInteger::unpack(&packed).unwrap();
    assert_eq!(val, **unpacked);
}

#[test]
fn test_struct_info() {
    fn get_bits<P: PackedStructInfo>(_s: &P) -> usize { P::packed_bits() }

    let typed: Integer<u32, Bits30> = 123.into();
    let msb = typed.as_packed_msb();
    assert_eq!(30, get_bits(&msb));
}

#[test]
fn test_slice_packing() {
    let mut data = vec![0xAA, 0xBB, 0xCC, 0xDD];
    let unpacked = <MsbInteger<_, _, Integer<u32, Bits32>>>::unpack_from_slice(&data).unwrap();
    assert_eq!(0xAABBCCDD, **unpacked);

    unpacked.pack_to_slice(&mut data).unwrap();
    assert_eq!(&[0xAA, 0xBB, 0xCC, 0xDD], &data[..]);
}

#[test]
fn test_packed_int_lsb_sub() {
    let val = 0xAABBCC;
    let typed: Integer<u32, Bits24> = val.into();
    let endian = typed.as_packed_lsb();
    let packed = endian.pack();
    assert_eq!([0xCC, 0xBB, 0xAA], packed);
}

#[test]
fn test_packed_int_lsb_bits4() {
    let typed: Integer<u8, Bits4> = 1.into();
    let endian = typed.as_packed_msb();
    let packed = endian.pack();
    assert_eq!([0x01], packed);
}

#[test]
fn test_big_slice_unpacking() {
    let data = vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE];
    let unpacked = <MsbInteger<_, _, Integer<u32, Bits32>>>::unpack_from_slice(&data).unwrap();
    assert_eq!(0xAABBCCDD, **unpacked);
}