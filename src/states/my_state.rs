use crate::{helpers::Transmutable, types::{F32Bytes, U64Bytes}};

pub struct MyState {
    field_a: U64Bytes,
    field_b: u8,
    field_c: F32Bytes,
}

// SAFETY: Is only represented by bytes and byte arrays.
unsafe impl Transmutable for MyState {
    const LEN: usize = size_of::<Self>();
}

impl MyState {
    pub fn field_a(&self) -> u64 {
        u64::from_le_bytes(self.field_a)
    }

    pub fn field_c(&self) -> f32 {
        f32::from_le_bytes(self.field_c)
    }
}
