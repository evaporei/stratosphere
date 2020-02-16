use std::ops::Add;

pub trait Number: Add<Output = Self> + Default + Copy + Into<f64> {}

macro_rules! number_trait_impl {
    ($name:ident for $($t:ty)*) => ($(
        impl $name for $t {}
    )*)
}

number_trait_impl!(Number for u8 u16 u32 i8 i16 i32 f32 f64);
