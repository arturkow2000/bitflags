#![cfg(feature = "const_operator")]
#![allow(incomplete_features)]
#![feature(const_trait_impl)]

#[macro_use]
extern crate bitflags;

bitflags! {
    /// baz
    struct Flags: u32 {
        const A = 0b00000001;
        #[doc = "bar"]
        const B = 0b00000010;
        const C = 0b00000100;
        #[doc = "foo"]
        const ABC = Flags::A.bits | Flags::B.bits | Flags::C.bits;
    }
}

const TEST0: Flags = Flags::A | Flags::B;
const TEST1: Flags = Flags::ABC - Flags::B;
const TEST2: Flags = (Flags::B | Flags::C) & Flags::B;
const TEST3: Flags = !Flags::B;
const TEST4: Flags = Flags::A ^ (Flags::A | Flags::C);

#[test]
fn main() {
    assert_eq!(TEST0, Flags::A | Flags::B);
    assert_eq!(TEST1, Flags::A | Flags::C);
    assert_eq!(TEST2, Flags::B);
    assert_eq!(TEST3, Flags::A | Flags::C);
    assert_eq!(TEST4, Flags::C);
}
