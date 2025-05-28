use std::ops::RangeInclusive;

pub trait PickBit {
    fn pick_bit(&self, bit: usize) -> bool;
    fn pick_bits(&self, bits: RangeInclusive<usize>) -> Self;
}

macro_rules! gen_pick_bits {
    ($($t: ty),*) => {
        $(
            impl PickBit for $t {
                fn pick_bit(&self, bit: usize) -> bool {
                    (self & (1 << bit)) != 0
                }

                fn pick_bits(&self, bits: RangeInclusive<usize>) -> $t {
                    (self & ((((1 << bits.end()) - 1) << 1) | 1)) >> bits.start()
                }
            }
        )*
    };
}

gen_pick_bits!(u8, u16, u32, u64, u128);
