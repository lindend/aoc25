use std::simd::{
    Simd,
    cmp::SimdPartialOrd,
    i64x8,
    num::{SimdInt, SimdUint},
};

static digits: [i64; 65] = [
    19, 19, 19, 19, 18, 18, 18, 17, 17, 17, 16, 16, 16, 16, 15, 15, 15, 14, 14, 14, 13, 13, 13, 13,
    12, 12, 12, 11, 11, 11, 10, 10, 10, 10, 9, 9, 9, 8, 8, 8, 7, 7, 7, 7, 6, 6, 6, 5, 5, 5, 4, 4,
    4, 4, 3, 3, 3, 2, 2, 2, 1, 1, 1, 1, 1,
];
static table: [i64; 65] = [
    999999999999999999,
    999999999999999999,
    999999999999999999,
    999999999999999999,
    999999999999999999,
    999999999999999999,
    999999999999999999,
    99999999999999999,
    99999999999999999,
    99999999999999999,
    9999999999999999,
    9999999999999999,
    9999999999999999,
    9999999999999999,
    999999999999999,
    999999999999999,
    999999999999999,
    99999999999999,
    99999999999999,
    99999999999999,
    9999999999999,
    9999999999999,
    9999999999999,
    9999999999999,
    999999999999,
    999999999999,
    999999999999,
    99999999999,
    99999999999,
    99999999999,
    9999999999,
    9999999999,
    9999999999,
    9999999999,
    999999999,
    999999999,
    999999999,
    99999999,
    99999999,
    99999999,
    9999999,
    9999999,
    9999999,
    9999999,
    999999,
    999999,
    999999,
    99999,
    99999,
    99999,
    9999,
    9999,
    9999,
    9999,
    999,
    999,
    999,
    99,
    99,
    99,
    9,
    9,
    9,
    9,
    0,
];

// https://lemire.me/blog/2025/01/07/counting-the-digits-of-64-bit-integers/
#[inline(always)]
pub fn simd_num_digits(num: i64x8) -> i64x8 {
    let leading_zeros = num.leading_zeros().cast::<usize>();
    let low = Simd::gather_or(&table, leading_zeros, Simd::splat(0));
    let high = Simd::gather_or(&digits, leading_zeros, Simd::splat(0));
    let low_lookup = Simd::simd_gt(num, low).select(Simd::splat(1), Simd::splat(0));
    let num_digits = low_lookup + high;
    num_digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_works() {
        assert_eq!(Simd::splat(3), simd_num_digits(Simd::splat(123)));
    }

    #[test]
    pub fn test_lots() {
        for i in 1..1_000_000 {
            assert_eq!(
                simd_num_digits(Simd::splat(i)),
                Simd::splat(i.ilog10() as i64 + 1),
                "Testing num digits for {}",
                i
            );
        }
    }
}
