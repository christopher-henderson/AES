// Typed as a u32, but don't think of it as an integer.
// Think of it as the polynomial x^8 + x^4 + x^3 + x + 1
// where x = 2. Produces the prime number, 283.
const IRREDUCIBLE_POLYNOMIAL: u32 = 0x011b;

type GF28 = u8;

fn add(a: GF28, b: GF28) -> GF28 {
    a ^ b
}

fn multiply(mut a: GF28, mut b: GF28) -> GF28 {
    let mut p: GF28 = 0;
    let mut carry;
    for _ in 0..8 {
        if b & 1 == 1 {
            p ^= a;
        }
        b >>= 1;
        carry = (a >> 7) == 1;
        a <<= 1;
        if carry {
            a ^= IRREDUCIBLE_POLYNOMIAL as GF28;
        }
    }
    p
}

mod tests {
    extern crate test;
    use super::*;

    #[test]
    fn add_test() {
        assert_eq!(add(0b01010111, 0b10000011), 0b11010100);
        assert_eq!(add(0x57, 0x83), 0xd4);
    }

    #[test]
    fn multiply_test() {
        assert_eq!(multiply(0x57, 0x83), 0xc1);
        assert_eq!(multiply(0x57, 0x13), 0xfe);
    }

    #[bench]
    fn multiply_bench(b: &mut test::Bencher) {
        b.iter(|| multiply(0x57, 0x83))
    }
}
