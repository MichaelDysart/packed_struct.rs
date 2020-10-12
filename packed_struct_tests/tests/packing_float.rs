extern crate packed_struct;
#[macro_use]
extern crate packed_struct_codegen;

use packed_struct::prelude::*;

macro_rules! int_to_float {
    ($TF: ident, $TI: ident, $NUM: expr) => {
        {
            #[repr(C)]
            union Number { float: $TF, int: $TI }
            unsafe { Number { int : $NUM }.float }
        }
    }
}

#[test]
#[cfg(test)]
fn test_packed_struct_f32_lsb() {
    
    #[derive(PackedStruct, PartialEq, Debug)]
    #[packed_struct(endian="lsb")]
    pub struct Velocity {
        speed: f32,
        direction: f32
    }

    let reg = Velocity {
        speed: int_to_float!(f32, u32, 0xAABBCCDD),
        direction: int_to_float!(f32, u32, 0x44332211),
    };

    let packed = reg.pack();
    assert_eq!(&packed, &[0xDD, 0xCC, 0xBB, 0xAA, 0x11, 0x22, 0x33, 0x44]);

    let unpacked = Velocity::unpack(&packed).unwrap();
    assert_eq!(&unpacked, &reg);
}

#[test]
#[cfg(test)]
fn test_packed_struct_f32_msb() {
    
    #[derive(PackedStruct, PartialEq, Debug)]
    #[packed_struct(endian="msb")]
    pub struct Velocity {
        speed: f32,
        direction: f32
    }

    let reg = Velocity {
        speed: int_to_float!(f32, u32, 0xAABBCCDD),
        direction: int_to_float!(f32, u32, 0x44332211),
    };

    let packed = reg.pack();
    assert_eq!(&packed, &[0xAA, 0xBB, 0xCC, 0xDD, 0x44, 0x33, 0x22, 0x11]);

    let unpacked = Velocity::unpack(&packed).unwrap();
    assert_eq!(&unpacked, &reg);
}

#[test]
#[cfg(test)]
fn test_packed_struct_f64_lsb() {
    
    #[derive(PackedStruct, PartialEq, Debug)]
    #[packed_struct(endian="lsb")]
    pub struct Velocity {
        speed: f64,
        direction: f64
    }

    let reg = Velocity {
        speed: int_to_float!(f64, u64, 0xAABBCCDD),
        direction: int_to_float!(f64, u64, 0x44332211),
    };

    let packed = reg.pack();
    assert_eq!(&packed, &[0xDD, 0xCC, 0xBB, 0xAA, 0x00, 0x00, 0x00, 0x00, 0x11, 0x22, 0x33, 0x44, 0x00, 0x00, 0x00, 0x00 ]);

    let unpacked = Velocity::unpack(&packed).unwrap();
    assert_eq!(&unpacked, &reg);
}

#[test]
#[cfg(test)]
fn test_packed_struct_f64_msb() {
    
    #[derive(PackedStruct, PartialEq, Debug)]
    #[packed_struct(endian="msb")]
    pub struct Velocity {
        speed: f64,
        direction: f64
    }

    let reg = Velocity {
        speed: int_to_float!(f64, u64, 0xAABBCCDD),
        direction: int_to_float!(f64, u64, 0x44332211),
    };

    let packed = reg.pack();
    assert_eq!(&packed, &[0x00, 0x00, 0x00, 0x00, 0xAA, 0xBB, 0xCC, 0xDD, 0x00, 0x00, 0x00, 0x00, 0x44, 0x33, 0x22, 0x11]);

    let unpacked = Velocity::unpack(&packed).unwrap();
    assert_eq!(&unpacked, &reg);
}

#[test]
#[cfg(test)]
fn test_packed_struct_float_values() {
    
    #[derive(PackedStruct, PartialEq, Debug)]
    pub struct TestStruct {
        #[packed_field(endian="lsb")]
        float1: f32,
        #[packed_field(endian="msb")]
        float2: f64
    }

    let reg = TestStruct {
        float1: 1.0,
        float2: -1.0,
    };

    let packed = reg.pack();
    assert_eq!(&packed, &[0x00, 0x00, 0x80, 0x3F, 0xBF, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

    let unpacked = TestStruct::unpack(&packed).unwrap();
    assert_eq!(&unpacked, &reg);
}


#[test]
#[cfg(test)]
fn test_packed_struct_float_bits() {
    
    #[derive(PackedStruct, PartialEq, Debug)]
    #[packed_struct(bit_numbering="msb0")]
    pub struct TestStruct {
        #[packed_field(bits="0..=3")]
        float1: Float<f32, packed_bits::Bits4>,
        #[packed_field(bits="4..=7")]
        float2: Integer<i8, packed_bits::Bits4>,
    }

    let reg = TestStruct {
        float1: 1.0000001.into(),
        float2: 2.into(),
    };

    let packed = reg.pack();
    assert_eq!(&packed, &[0x12]);

    // let unpacked = TestStruct::unpack(&packed).unwrap();
    // assert_eq!(&unpacked, &reg);
}
