//! Integers that are limited by a bit width, with methods to store them
//! as a native type, packing and unpacking into byte arrays, with MSB/LSB
//! support.

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

/// A positive bit mask of the desired width.
/// 
/// ones(1) => 0b1
/// ones(2) => 0b11
/// ones(3) => 0b111
/// ...
pub fn ones(n: u64) -> u64 {
	if n == 0 { return 0; }
	if n >= 64 { return !0; }

	(1 << n) - 1
}
