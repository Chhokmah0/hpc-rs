//! There is nothing public in this module. Just for testing alignment.
//! repr(rust) and repr(C) are used to control the layout of structs in Rust.

struct DataRust {
    a: u8,
    b: u16,
    c: u32,
    d: u8,
}

#[repr(C)]
struct DataC {
    a: u8,
    b: u16,
    c: u32,
    d: u8,
}

#[repr(C)]
struct DataCAligned {
    c: u32,
    b: u16,
    a: u8,
    d: u8,
}

struct Data2Rust {
    a: u64,
    b: u8,
}

#[repr(C)]
struct Data2C {
    a: u64,
    b: u8,
}

#[repr(C, packed)]
struct Data2CPacked {
    a: u64,
    b: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_layout() {
        assert_eq!(std::mem::size_of::<DataRust>(), 8);
        assert_eq!(std::mem::size_of::<DataC>(), 12);
        assert_eq!(std::mem::size_of::<DataCAligned>(), 8);
        assert_eq!(std::mem::align_of::<DataRust>(), 4);
        assert_eq!(std::mem::align_of::<DataC>(), 4);
        assert_eq!(std::mem::align_of::<DataCAligned>(), 4);
    }

    #[test]
    fn test_data2_layout() {
        assert_eq!(std::mem::size_of::<Data2Rust>(), 16);
        assert_eq!(std::mem::size_of::<Data2C>(), 16);
        assert_eq!(std::mem::size_of::<Data2CPacked>(), 9);
        assert_eq!(std::mem::align_of::<Data2Rust>(), 8);
        assert_eq!(std::mem::align_of::<Data2C>(), 8);
        assert_eq!(std::mem::align_of::<Data2CPacked>(), 1);
    }
}
