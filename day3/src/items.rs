#![allow(non_camel_case_types)]

use strum::{EnumIter, IntoEnumIterator};

pub type Mask = u64;

#[derive(Copy, Clone, EnumIter)]
enum ItemChar {
    a = 1,
    b = 2,
    c = 3,
    d = 4,
    e = 5,
    f = 6,
    g = 7,
    h = 8,
    i = 9,
    j = 10,
    k = 11,
    l = 12,
    m = 13,
    n = 14,
    o = 15,
    p = 16,
    q = 17,
    r = 18,
    s = 19,
    t = 20,
    u = 21,
    v = 22,
    w = 23,
    x = 24,
    y = 25,
    z = 26,
    A = 27,
    B = 28,
    C = 29,
    D = 30,
    E = 31,
    F = 32,
    G = 33,
    H = 34,
    I = 35,
    J = 36,
    K = 37,
    L = 38,
    M = 39,
    N = 40,
    O = 41,
    P = 42,
    Q = 43,
    R = 44,
    S = 45,
    T = 46,
    U = 47,
    V = 48,
    W = 49,
    X = 50,
    Y = 51,
    Z = 52,
}
impl From<char> for ItemChar {
    fn from(c: char) -> Self {
        match c {
            'a' => ItemChar::a,
            'b' => ItemChar::b,
            'c' => ItemChar::c,
            'd' => ItemChar::d,
            'e' => ItemChar::e,
            'f' => ItemChar::f,
            'g' => ItemChar::g,
            'h' => ItemChar::h,
            'i' => ItemChar::i,
            'j' => ItemChar::j,
            'k' => ItemChar::k,
            'l' => ItemChar::l,
            'm' => ItemChar::m,
            'n' => ItemChar::n,
            'o' => ItemChar::o,
            'p' => ItemChar::p,
            'q' => ItemChar::q,
            'r' => ItemChar::r,
            's' => ItemChar::s,
            't' => ItemChar::t,
            'u' => ItemChar::u,
            'v' => ItemChar::v,
            'w' => ItemChar::w,
            'x' => ItemChar::x,
            'y' => ItemChar::y,
            'z' => ItemChar::z,
            'A' => ItemChar::A,
            'B' => ItemChar::B,
            'C' => ItemChar::C,
            'D' => ItemChar::D,
            'E' => ItemChar::E,
            'F' => ItemChar::F,
            'G' => ItemChar::G,
            'H' => ItemChar::H,
            'I' => ItemChar::I,
            'J' => ItemChar::J,
            'K' => ItemChar::K,
            'L' => ItemChar::L,
            'M' => ItemChar::M,
            'N' => ItemChar::N,
            'O' => ItemChar::O,
            'P' => ItemChar::P,
            'Q' => ItemChar::Q,
            'R' => ItemChar::R,
            'S' => ItemChar::S,
            'T' => ItemChar::T,
            'U' => ItemChar::U,
            'V' => ItemChar::V,
            'W' => ItemChar::W,
            'X' => ItemChar::X,
            'Y' => ItemChar::Y,
            'Z' => ItemChar::Z,
            _ => panic!(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Item {
    pub prio: u8,
    mask: Mask,
}
impl From<ItemChar> for Item {
    fn from(ic: ItemChar) -> Self {
        Item {
            prio: ic as u8,
            mask: 0b1 << (ic as u8 - 1),
        }
    }
}
impl From<char> for Item {
    fn from(c: char) -> Self {
        let ic = ItemChar::from(c);
        Item::from(ic)
    }
}

pub fn mask_from_iter<'a, I>(iter: I) -> Mask
where
    I: IntoIterator<Item = &'a Item>,
{
    iter.into_iter().fold(0, |mask, next| mask | next.mask)
}

pub fn items_from_mask(mask: Mask) -> Vec<Item> {
    let mut out: Vec<Item> = Vec::new();
    for ic in ItemChar::iter() {
        let item = Item::from(ic);
        if item.mask & mask != 0 {
            out.push(item);
        }
    }
    out
}
