//! Integers that are limited by a bit width, with methods to store them
//! as a native type, packing and unpacking into byte arrays, with MSB/LSB
//! support.

use internal_prelude::v1::*;

use super::types_bits::*;

use super::packing::{PackingError, PackedStruct, PackedStructInfo, PackedStructSlice};

use serde::ser::{Serialize, Serializer};
use serde::de::{Deserialize, Deserializer};

macro_rules! number_type {
    ($T: ident, $TSized: ident, $TAsBytes: ident, $TMsb: ident, $TLsb: ident) => {
        #[derive(Default, Copy, Clone)]
        pub struct $T<T, B> {
            num: T,
            bits: PhantomData<B>
        }
        
        impl<T, B> Debug for $T<T, B> where T: Debug {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:?}", self.num)
            }
        }
        
        impl<T, B> Display for $T<T, B> where T: Display {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.num)
            }
        }
        
        
        impl<T, B> Serialize for $T<T, B> where T: Serialize {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: Serializer
            {
                self.num.serialize(serializer)
            }
        }

        impl<'de, T, B> Deserialize<'de> for $T<T, B> where T: Deserialize<'de>, T: Into<$T<T, B>> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: Deserializer<'de>
            {
                <T>::deserialize(deserializer).map(|n| n.into())
            }
        }
        
        impl<T, B> PartialEq for $T<T, B> where T: PartialEq {
            fn eq(&self, other: &Self) -> bool {
                self.num.eq(&other.num)
            }
        }
        
        impl<T, B> $T<T, B> where Self: Copy {
            /// Convert into a MSB packing helper
            pub fn as_packed_msb(&self) -> $TMsb<T, B, Self> {
                $TMsb(*self, Default::default(), Default::default())
            }
        
            /// Convert into a LSB packing helper
            pub fn as_packed_lsb(&self) -> $TLsb<T, B, Self> {
                $TLsb(*self, Default::default(), Default::default())
            }
        }
        
        /// Convert an integer of a specific bit width into native types.
        pub trait $TSized<T, B: NumberOfBits> {
            /// The bit mask that is used for all incoming values. For an integer
            /// of width 8, that is 0xFF.
            fn value_bit_mask() -> T;
            /// Convert from the platform native type, applying the value mask.
            fn from_primitive(val: T) -> Self;
            /// Convert to the platform's native type.
            fn to_primitive(&self) -> T;
            /// Convert to a MSB byte representation. 0xAABB is converted into [0xAA, 0xBB].
            fn to_msb_bytes(&self) -> <<B as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes;
            /// Convert to a LSB byte representation. 0xAABB is converted into [0xBB, 0xAA].
            fn to_lsb_bytes(&self) -> <<B as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes where B: BitsFullBytes;
            /// Convert from a MSB byte array.
            fn from_msb_bytes(bytes: &<<B as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes) -> Self;
            /// Convert from a LSB byte array.
            fn from_lsb_bytes(bytes: &<<B as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes) -> Self where B: BitsFullBytes;
        }
        
        /// Convert a native platform integer type into a byte array.
        pub trait $TAsBytes where Self: Sized {
            /// The byte array type, for instance [u8; 2].
            type AsBytes;
        
            /// Convert into a MSB byte array.
            fn to_msb_bytes(&self) -> Self::AsBytes;
            /// Convert into a LSB byte array.
            fn to_lsb_bytes(&self) -> Self::AsBytes;
            /// Convert from a MSB byte array.
            fn from_msb_bytes(bytes: &Self::AsBytes) -> Self;
            /// Convert from a LSB byte array.
            fn from_lsb_bytes(bytes: &Self::AsBytes) -> Self;
        }

        /// A wrapper that packages the integer as a MSB packaged byte array. Usually
        /// invoked using code generation.
        pub struct $TMsb<T, B, I>(I, PhantomData<T>, PhantomData<B>);
        impl<T, B, I> Deref for $TMsb<T, B, I> {
            type Target = I;

            fn deref(&self) -> &I {
                &self.0
            }
        }
        impl<T, B, I> From<I> for $TMsb<T, B, I> {
            fn from(i: I) -> Self {
                $TMsb(i, Default::default(), Default::default())
            }
        }

        impl<T, B, I> Debug for $TMsb<T, B, I> where I: Debug {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:?}", self.0)
            }
        }

        impl<T, B, I> Display for $TMsb<T, B, I> where I: Display {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl<T, B, I> PackedStruct<<<B as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes> for $TMsb<T, B, I>
            where B: NumberOfBits, I: $TSized<T, B>
        {
            fn pack(&self) -> <<B as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes {
                self.0.to_msb_bytes()
            }

            #[inline]
            fn unpack(src: &<<B as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes) -> Result<Self, PackingError> {
                let n = I::from_msb_bytes(src);
                let n = $TMsb(n, Default::default(), Default::default());
                Ok(n)
            }
        }

        impl<T, B, I> PackedStructInfo for $TMsb<T, B, I> where B: NumberOfBits {
            #[inline]
            fn packed_bits() -> usize {
                B::number_of_bits() as usize
            }
        }

        impl<T, B, I> PackedStructSlice for $TMsb<T, B, I> where B: NumberOfBits, I: $TSized<T, B> {
            fn pack_to_slice(&self, output: &mut [u8]) -> Result<(), PackingError> {
                let expected_bytes = <B as NumberOfBits>::Bytes::number_of_bytes() as usize;
                if output.len() != expected_bytes {
                    return Err(PackingError::BufferSizeMismatch { expected: expected_bytes, actual: output.len() });
                }
                let packed = self.pack();
                &mut output[..].copy_from_slice(packed.as_bytes_slice());
                Ok(())
            }

            fn unpack_from_slice(src: &[u8]) -> Result<Self, PackingError> {
                let expected_bytes = <B as NumberOfBits>::Bytes::number_of_bytes() as usize;
                if src.len() < expected_bytes {
                    return Err(PackingError::BufferSizeMismatch { expected: expected_bytes, actual: src.len() });
                }
                let mut s = Default::default();
                // hack to infer the type
                {
                    Self::unpack(&s)?;
                }
                s.as_mut_bytes_slice().copy_from_slice(&src[..expected_bytes]);
                Self::unpack(&s)
            }

            fn packed_bytes() -> usize {
                <B as NumberOfBits>::Bytes::number_of_bytes() as usize
            }
        }

        /// A wrapper that packages the integer as a LSB packaged byte array. Usually
        /// invoked using code generation.
        pub struct $TLsb<T, B, I>(I, PhantomData<T>, PhantomData<B>);
        impl<T, B, I> Deref for $TLsb<T, B, I> where B: BitsFullBytes {
            type Target = I;

            fn deref(&self) -> &I {
                &self.0
            }
        }

        impl<T, B, I> From<I> for $TLsb<T, B, I> where B: BitsFullBytes {
            fn from(i: I) -> Self {
                $TLsb(i, Default::default(), Default::default())
            }
        }

        impl<T, B, I> Debug for $TLsb<T, B, I> where I: Debug {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:?}", self.0)
            }
        }

        impl<T, B, I> Display for $TLsb<T, B, I> where I: Display {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl<T, B, I> PackedStruct<<<B as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes> for $TLsb<T, B, I>
            where B: NumberOfBits, I: $TSized<T, B>, B: BitsFullBytes
        {
            fn pack(&self) -> <<B as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes {
                self.0.to_lsb_bytes()        
            }

            #[inline]
            fn unpack(src: &<<B as NumberOfBits>::Bytes as NumberOfBytes>::AsBytes) -> Result<Self, PackingError> {
                let n = I::from_lsb_bytes(src);
                let n = $TLsb(n, Default::default(), Default::default());
                Ok(n)
            }
        }

        impl<T, B, I> PackedStructInfo for $TLsb<T, B, I> where B: NumberOfBits {
            #[inline]
            fn packed_bits() -> usize {
                B::number_of_bits() as usize
            }
        }

        impl<T, B, I> PackedStructSlice for $TLsb<T, B, I> where B: NumberOfBits + BitsFullBytes, I: $TSized<T, B> {
            fn pack_to_slice(&self, output: &mut [u8]) -> Result<(), PackingError> {
                let expected_bytes = <B as NumberOfBits>::Bytes::number_of_bytes() as usize;
                if output.len() != expected_bytes {
                    return Err(PackingError::BufferSizeMismatch { expected: expected_bytes, actual: output.len() });
                }
                let packed = self.pack();
                &mut output[..].copy_from_slice(packed.as_bytes_slice());
                Ok(())
            }

            fn unpack_from_slice(src: &[u8]) -> Result<Self, PackingError> {
                let expected_bytes = <B as NumberOfBits>::Bytes::number_of_bytes() as usize;
                if src.len() < expected_bytes {
                    return Err(PackingError::BufferSizeMismatch { expected: expected_bytes, actual: src.len() });
                }
                let mut s = Default::default();
                // hack to infer the type
                {
                    Self::unpack(&s)?;
                }
                s.as_mut_bytes_slice().copy_from_slice(&src[..expected_bytes]);
                Self::unpack(&s)
            }

            fn packed_bytes() -> usize {
                <B as NumberOfBits>::Bytes::number_of_bytes() as usize
            }
        }
    }
}

number_type!(Integer, SizedInteger, IntegerAsBytes, MsbInteger, LsbInteger);
number_type!(Float, SizedFloat, FloatAsBytes, MsbFloat, LsbFloat);

macro_rules! as_bytes_msb {
    (1, $v: expr) => {
        [
            (($v >> 0) as u8 & 0xFF)
        ]
    };
    (2, $v: expr) => {
        [
            (($v >> 8) as u8 & 0xFF),
            (($v >> 0) as u8 & 0xFF),
        ]
    };
    (4, $v: expr) => {
        [
            (($v >> 24) as u8 & 0xFF),
            (($v >> 16) as u8 & 0xFF),
            (($v >> 8) as u8 & 0xFF),
            (($v >> 0) as u8 & 0xFF)
        ]
    };
    (8, $v: expr) => {
        [
            (($v >> 56) as u8 & 0xFF),
            (($v >> 48) as u8 & 0xFF),
            (($v >> 40) as u8 & 0xFF),
            (($v >> 32) as u8 & 0xFF),
            (($v >> 24) as u8 & 0xFF),
            (($v >> 16) as u8 & 0xFF),
            (($v >> 8) as u8 & 0xFF),
            (($v >> 0) as u8 & 0xFF)
        ]
    }
}

macro_rules! as_bytes_lsb {
    (1, $v: expr) => {
        [
            (($v >> 0) as u8 & 0xFF)
        ]
    };
    (2, $v: expr) => {
        [
            (($v >> 0) as u8 & 0xFF),
            (($v >> 8) as u8 & 0xFF),
        ]
    };
    (4, $v: expr) => {
        [
            (($v >> 0) as u8 & 0xFF),
            (($v >> 8) as u8 & 0xFF),
            (($v >> 16) as u8 & 0xFF),
            (($v >> 24) as u8 & 0xFF),
        ]
    };
    (8, $v: expr) => {
        [
            (($v >> 0) as u8 & 0xFF),
            (($v >> 8) as u8 & 0xFF),
            (($v >> 16) as u8 & 0xFF),
            (($v >> 24) as u8 & 0xFF),
            (($v >> 32) as u8 & 0xFF),
            (($v >> 40) as u8 & 0xFF),
            (($v >> 48) as u8 & 0xFF),
            (($v >> 56) as u8 & 0xFF),
        ]
    }
}

macro_rules! from_bytes_msb {
    (1, $v: expr, $T: ident) => {
        $v[0] as $T
    };
    (2, $v: expr, $T: ident) => {
        (($v[0] as $T) << 8) |
        (($v[1] as $T) << 0)
    };
    (4, $v: expr, $T: ident) => {
        (($v[0] as $T) << 24) |
        (($v[1] as $T) << 16) |
        (($v[2] as $T) << 8) |
        (($v[3] as $T) << 0)
    };
    (8, $v: expr, $T: ident) => {
        (($v[0] as $T) << 56) |
        (($v[1] as $T) << 48) |
        (($v[2] as $T) << 40) |
        (($v[3] as $T) << 32) |
        (($v[4] as $T) << 24) |
        (($v[5] as $T) << 16) |
        (($v[6] as $T) << 8) |
        (($v[7] as $T) << 0)
    };
}

macro_rules! from_bytes_lsb {
    (1, $v: expr, $T: ident) => {
        $v[0] as $T
    };
    (2, $v: expr, $T: ident) => {
        (($v[0] as $T) << 0) |
        (($v[1] as $T) << 8)
    };
    (4, $v: expr, $T: ident) => {
        (($v[0] as $T) << 0) |
        (($v[1] as $T) << 8) |
        (($v[2] as $T) << 16) |
        (($v[3] as $T) << 24)
    };
    (8, $v: expr, $T: ident) => {
        (($v[0] as $T) << 0) |
        (($v[1] as $T) << 8) |
        (($v[2] as $T) << 16) |
        (($v[3] as $T) << 24) |
        (($v[4] as $T) << 32) |
        (($v[5] as $T) << 40) |
        (($v[6] as $T) << 48) |
        (($v[7] as $T) << 56)
    };
}

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

/// A positive bit mask of the desired width.
/// 
/// ones(1) => 0b1
/// ones(2) => 0b11
/// ones(3) => 0b111
/// ...
fn ones(n: u64) -> u64 {
	if n == 0 { return 0; }
	if n >= 64 { return !0; }

	(1 << n) - 1
}

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
