use internal_prelude::v1::*;

use super::types_bits::*;

use super::packing::{PackingError, PackedStruct, PackedStructInfo, PackedStructSlice};

use serde::ser::{Serialize, Serializer};
use serde::de::{Deserialize, Deserializer};

use super::types_num::*;
use super::types_int::*;

number_type!(Float, SizedFloat, FloatAsBytes, MsbFloat, LsbFloat);

macro_rules! float_to_int {
    ($TF: ident, $TI: ident, $NUM: expr) => {
        {
            #[repr(C)]
            union NumberConverter { float: $TF, int: $TI }
            unsafe { NumberConverter { float : $NUM }.int }
        }
    }
}

macro_rules! int_to_float {
    ($TF: ident, $TI: ident, $NUM: expr) => {
        {
            #[repr(C)]
            union NumberConverter { float: $TF, int: $TI }
            unsafe { NumberConverter { int : $NUM }.float }
        }
    }
}

macro_rules! float_as_bytes {
    ($TF: ident, $TI: ident, $N: tt) => {
        impl FloatAsBytes for $TF {
            type AsBytes = [u8; $N];

            #[inline]
            fn to_msb_bytes(&self) -> [u8; $N] {
                float_to_int!($TF, $TI, *self).to_msb_bytes()
            }

            #[inline]
            fn to_lsb_bytes(&self) -> [u8; $N] {
                float_to_int!($TF, $TI, *self).to_lsb_bytes()
            }

            #[inline]
            fn from_msb_bytes(bytes: &[u8; $N]) -> Self {
                int_to_float!($TF, $TI, <$TI>::from_msb_bytes(bytes))
            }

            #[inline]
            fn from_lsb_bytes(bytes: &[u8; $N]) -> Self {
                int_to_float!($TF, $TI, <$TI>::from_lsb_bytes(bytes))
            }
        }
    };
}

float_as_bytes!(f32, u32, 4);
float_as_bytes!(f64, u64, 8);

macro_rules! float_bytes_impl {
    ($TF: ident, $TI: ident, $TB: ident) => {
        impl SizedFloat<$TF, $TB> for Float<$TF, $TB> {
            #[inline]
            fn value_bit_mask() -> $TF {
                int_to_float!($TF, $TI, ones($TB::number_of_bits() as u64) as $TI)
            }

            #[inline]
            fn from_primitive(val: $TF) -> Self {
                let val : $TI = float_to_int!($TF, $TI, val);
                let mask : $TI = float_to_int!($TF, $TI, Self::value_bit_mask());
                let new_val : $TF = int_to_float!($TF, $TI, val & mask);
                Float { num: new_val, bits: Default::default() }
            }

            #[inline]
            fn to_primitive(&self) -> $TF {
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
                    <$TF>::from_msb_bytes(&native_bytes);
                }
                let skip = native_bytes.len() - bytes.len();
                {
                    let native_bytes = &mut native_bytes[skip..];
                    native_bytes.copy_from_slice(&bytes[..]);
                }
                let v = <$TF>::from_msb_bytes(&native_bytes);
                Self::from_primitive(v)
            }

            #[inline]
            fn from_lsb_bytes(bytes: &<<$TB as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes) -> Self
            {
                let mut native_bytes = Default::default();
                {
                    // hack that infers the size of the native array...
                    <$TF>::from_lsb_bytes(&native_bytes);
                }

                {
                    let take = bytes.len();
                    let native_bytes = &mut native_bytes[..take];
                    native_bytes.copy_from_slice(&bytes[..]);
                }

                let v = <$TF>::from_lsb_bytes(&native_bytes);
                Self::from_primitive(v)
            }
        }

        impl From<$TF> for Float<$TF, $TB> {
            fn from(v: $TF) -> Self {
                Self::from_primitive(v)
            }
        }

        impl From<Float<$TF, $TB>> for $TF {
            fn from(v: Float<$TF, $TB>) -> Self {
                v.to_primitive()
            }
        }

        impl Deref for Float<$TF, $TB> {
            type Target = $TF;

            fn deref(&self) -> &$TF {
                &self.num
            }
        }
    };
}

macro_rules! float_bytes1_impl {
    ($TF: ident, $TI: ident) => {
        float_bytes_impl!($TF, $TI, Bits1);
        float_bytes_impl!($TF, $TI, Bits2);
        float_bytes_impl!($TF, $TI, Bits3);
        float_bytes_impl!($TF, $TI, Bits4);
        float_bytes_impl!($TF, $TI, Bits5);
        float_bytes_impl!($TF, $TI, Bits6);
        float_bytes_impl!($TF, $TI, Bits7);
        float_bytes_impl!($TF, $TI, Bits8);
    };
}

macro_rules! float_bytes2_impl {
    ($TF: ident, $TI: ident) => {
        float_bytes_impl!($TF, $TI, Bits9);
        float_bytes_impl!($TF, $TI, Bits10);
        float_bytes_impl!($TF, $TI, Bits11);
        float_bytes_impl!($TF, $TI, Bits12);
        float_bytes_impl!($TF, $TI, Bits13);
        float_bytes_impl!($TF, $TI, Bits14);
        float_bytes_impl!($TF, $TI, Bits15);
        float_bytes_impl!($TF, $TI, Bits16);
    };
}

macro_rules! float_bytes3_impl {
    ($TF: ident, $TI: ident) => {
        float_bytes_impl!($TF, $TI, Bits17);
        float_bytes_impl!($TF, $TI, Bits18);
        float_bytes_impl!($TF, $TI, Bits19);
        float_bytes_impl!($TF, $TI, Bits20);
        float_bytes_impl!($TF, $TI, Bits21);
        float_bytes_impl!($TF, $TI, Bits22);
        float_bytes_impl!($TF, $TI, Bits23);
        float_bytes_impl!($TF, $TI, Bits24);
    };
}

macro_rules! float_bytes4_impl {
    ($TF: ident, $TI: ident) => {
        float_bytes_impl!($TF, $TI, Bits25);
        float_bytes_impl!($TF, $TI, Bits26);
        float_bytes_impl!($TF, $TI, Bits27);
        float_bytes_impl!($TF, $TI, Bits28);
        float_bytes_impl!($TF, $TI, Bits29);
        float_bytes_impl!($TF, $TI, Bits30);
        float_bytes_impl!($TF, $TI, Bits31);
        float_bytes_impl!($TF, $TI, Bits32);
    };
}

macro_rules! float_bytes5_impl {
    ($TF: ident, $TI: ident) => {
        float_bytes_impl!($TF, $TI, Bits33);
        float_bytes_impl!($TF, $TI, Bits34);
        float_bytes_impl!($TF, $TI, Bits35);
        float_bytes_impl!($TF, $TI, Bits36);
        float_bytes_impl!($TF, $TI, Bits37);
        float_bytes_impl!($TF, $TI, Bits38);
        float_bytes_impl!($TF, $TI, Bits39);
        float_bytes_impl!($TF, $TI, Bits40);
    };
}

macro_rules! float_bytes6_impl {
    ($TF: ident, $TI: ident) => {
        float_bytes_impl!($TF, $TI, Bits41);
        float_bytes_impl!($TF, $TI, Bits42);
        float_bytes_impl!($TF, $TI, Bits43);
        float_bytes_impl!($TF, $TI, Bits44);
        float_bytes_impl!($TF, $TI, Bits45);
        float_bytes_impl!($TF, $TI, Bits46);
        float_bytes_impl!($TF, $TI, Bits47);
        float_bytes_impl!($TF, $TI, Bits48);
    };
}

macro_rules! float_bytes7_impl {
    ($TF: ident, $TI: ident) => {
        float_bytes_impl!($TF, $TI, Bits49);
        float_bytes_impl!($TF, $TI, Bits50);
        float_bytes_impl!($TF, $TI, Bits51);
        float_bytes_impl!($TF, $TI, Bits52);
        float_bytes_impl!($TF, $TI, Bits53);
        float_bytes_impl!($TF, $TI, Bits54);
        float_bytes_impl!($TF, $TI, Bits55);
        float_bytes_impl!($TF, $TI, Bits56);
    };
}

macro_rules! float_bytes8_impl {
    ($TF: ident, $TI: ident) => {
        float_bytes_impl!($TF, $TI, Bits57);
        float_bytes_impl!($TF, $TI, Bits58);
        float_bytes_impl!($TF, $TI, Bits59);
        float_bytes_impl!($TF, $TI, Bits60);
        float_bytes_impl!($TF, $TI, Bits61);
        float_bytes_impl!($TF, $TI, Bits62);
        float_bytes_impl!($TF, $TI, Bits63);
        float_bytes_impl!($TF, $TI, Bits64);
    };
}

float_bytes1_impl!(f32, u32);
float_bytes2_impl!(f32, u32);
float_bytes3_impl!(f32, u32);
float_bytes4_impl!(f32, u32);
float_bytes5_impl!(f64, u64);
float_bytes6_impl!(f64, u64);
float_bytes7_impl!(f64, u64);
float_bytes8_impl!(f64, u64);

#[test]
fn test_f32_literal() {
    let num: f32 = 4405.475;
    assert_eq!([0x45, 0x89, 0xAB, 0xCD], num.to_msb_bytes());
    assert_eq!([0xCD, 0xAB, 0x89, 0x45], num.to_lsb_bytes());
}

#[test]
fn test_f64_literal() {
    let num: f64 = 3.841412024471731e-226;
    assert_eq!([0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88], num.to_msb_bytes());
    assert_eq!([0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11], num.to_lsb_bytes());
}

#[test]
fn test_f32() {
    let val = int_to_float!(f32, u32, 0x4589ABCD);
    let num: Float<f32, Bits32> = val.into();
    assert_eq!(val, *num);
    assert_eq!([0x45, 0x89, 0xAB, 0xCD], num.to_msb_bytes());
    assert_eq!([0xCD, 0xAB, 0x89, 0x45], num.to_lsb_bytes());
}

#[test]
fn test_f64() {
    let val = int_to_float!(f64, u64, 0x1122334455667788);
    let num: Float<f64, Bits64> = val.into();
    assert_eq!(val, *num);
    assert_eq!([0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88], num.to_msb_bytes());
    assert_eq!([0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11], num.to_lsb_bytes());
}

#[test]
fn test_roundtrip_f32_literal() {
    let val = int_to_float!(f32, u32, 0x11223344);
    let num: Float<f32, Bits32> = val.into();
    let msb_bytes = num.to_msb_bytes();
    let from_msb = f32::from_msb_bytes(&msb_bytes);
    assert_eq!(val, from_msb);

    let lsb_bytes = num.to_lsb_bytes();
    let from_lsb = f32::from_lsb_bytes(&lsb_bytes);
    assert_eq!(val, from_lsb);
}

#[test]
fn test_roundtrip_f32() {
    let val = int_to_float!(f32, u32, 0x11223344);
    let num: Float<f32, Bits32> = val.into();
    let msb_bytes = num.to_msb_bytes();
    let from_msb = <Float<f32, Bits32>>::from_msb_bytes(&msb_bytes);
    assert_eq!(val, *from_msb);

    let lsb_bytes = num.to_lsb_bytes();
    let from_lsb = <Float<f32, Bits32>>::from_lsb_bytes(&lsb_bytes);
    assert_eq!(val, *from_lsb);
}

#[test]
fn test_roundtrip_f64_literal() {
    let val = int_to_float!(f64, u64, 0xCCBBAA);
    let num: Float<f64, Bits64> = val.into();
    let msb_bytes = num.to_msb_bytes();
    assert_eq!([0x00, 0x00, 0x00, 0x00, 0x00, 0xCC, 0xBB, 0xAA], msb_bytes);
    let from_msb = f64::from_msb_bytes(&msb_bytes);
    assert_eq!(val, from_msb);

    let lsb_bytes = num.to_lsb_bytes();
    assert_eq!([0xAA, 0xBB, 0xCC, 0x00, 0x00, 0x00, 0x00, 0x00], lsb_bytes);
    let from_lsb = f64::from_lsb_bytes(&lsb_bytes);
    assert_eq!(val, from_lsb);
}

#[test]
fn test_roundtrip_f64() {
    let val = int_to_float!(f64, u64, 0xCCBBAA);
    let num: Float<f64, Bits64> = val.into();
    let msb_bytes = num.to_msb_bytes();
    assert_eq!([0x00, 0x00, 0x00, 0x00, 0x00, 0xCC, 0xBB, 0xAA], msb_bytes);
    let from_msb = <Float<f64, Bits64>>::from_msb_bytes(&msb_bytes);
    assert_eq!(val, *from_msb);

    let lsb_bytes = num.to_lsb_bytes();
    assert_eq!([0xAA, 0xBB, 0xCC, 0x00, 0x00, 0x00, 0x00, 0x00], lsb_bytes);
    let from_lsb = <Float<f64, Bits64>>::from_lsb_bytes(&lsb_bytes);
    assert_eq!(val, *from_lsb);
}

#[test]
fn test_packed_float_msb() {
    let val = int_to_float!(f32, u32, 0xAABBCCDD);
    let typed: Float<f32, Bits32> = val.into();
    let endian = typed.as_packed_msb();
    let packed = endian.pack();
    assert_eq!([0xAA, 0xBB, 0xCC, 0xDD], packed);

    let unpacked: MsbFloat<_, _, Float<f32, Bits32>> = MsbFloat::unpack(&packed).unwrap();
    assert_eq!(val, **unpacked);
}

#[test]
fn test_packed_float_partial() {
    let val = int_to_float!(f32, u32, 0b10_10101010);
    let typed: Float<f32, Bits10> = val.into();
    let endian = typed.as_packed_msb();
    let packed = endian.pack();
    assert_eq!([0b00000010, 0b10101010], packed);

    let unpacked: MsbFloat<_, _, Float<f32, Bits10>> = MsbFloat::unpack(&packed).unwrap();
    assert_eq!(val, **unpacked);
}

#[test]
fn test_packed_float_lsb() {
    let val = int_to_float!(f32, u32, 0xAABBCCDD);
    let typed: Float<f32, Bits32> = val.into();
    let endian = typed.as_packed_lsb();
    let packed = endian.pack();
    assert_eq!([0xDD, 0xCC, 0xBB, 0xAA], packed);

    let unpacked: LsbFloat<_, _, Float<f32, Bits32>> = LsbFloat::unpack(&packed).unwrap();
    assert_eq!(val, **unpacked);
}

#[test]
fn test_float_struct_info() {
    fn get_bits<P: PackedStructInfo>(_s: &P) -> usize { P::packed_bits() }

    let val = int_to_float!(f32, u32, 123);

    let typed: Float<f32, Bits30> = val.into();
    let msb = typed.as_packed_msb();
    assert_eq!(30, get_bits(&msb));
}

#[test]
fn test_float_slice_packing() {
    let mut data = vec![0xAA, 0xBB, 0xCC, 0xDD];
    let unpacked = <MsbFloat<_, _, Float<f32, Bits32>>>::unpack_from_slice(&data).unwrap();
    assert_eq!(int_to_float!(f32, u32, 0xAABBCCDD), **unpacked);

    unpacked.pack_to_slice(&mut data).unwrap();
    assert_eq!(&[0xAA, 0xBB, 0xCC, 0xDD], &data[..]);
}

#[test]
fn test_packed_float_lsb_sub() {
    let val = int_to_float!(f32, u32, 0xAABBCC);
    let typed: Float<f32, Bits24> = val.into();
    let endian = typed.as_packed_lsb();
    let packed = endian.pack();
    assert_eq!([0xCC, 0xBB, 0xAA], packed);
}

#[test]
fn test_packed_float_lsb_bits4() {
    let typed: Float<f32, Bits4> = 1.0000001.into();
    let endian = typed.as_packed_msb();
    let packed = endian.pack();
    assert_eq!([0x01], packed);
}

#[test]
fn test_float_big_slice_unpacking() {
    let data = vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE];
    let unpacked = <MsbFloat<_, _, Float<f32, Bits32>>>::unpack_from_slice(&data).unwrap();
    assert_eq!(int_to_float!(f32, u32, 0xAABBCCDD), **unpacked);
}
