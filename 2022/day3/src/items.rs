#![allow(non_camel_case_types)]

use strum::{EnumIter, IntoEnumIterator};

pub type Mask = u64;

#[derive(Copy, Clone, EnumIter)]
enum ItemId {
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
impl From<char> for ItemId {
    fn from(c: char) -> Self {
        match c {
            'a' => ItemId::a,
            'b' => ItemId::b,
            'c' => ItemId::c,
            'd' => ItemId::d,
            'e' => ItemId::e,
            'f' => ItemId::f,
            'g' => ItemId::g,
            'h' => ItemId::h,
            'i' => ItemId::i,
            'j' => ItemId::j,
            'k' => ItemId::k,
            'l' => ItemId::l,
            'm' => ItemId::m,
            'n' => ItemId::n,
            'o' => ItemId::o,
            'p' => ItemId::p,
            'q' => ItemId::q,
            'r' => ItemId::r,
            's' => ItemId::s,
            't' => ItemId::t,
            'u' => ItemId::u,
            'v' => ItemId::v,
            'w' => ItemId::w,
            'x' => ItemId::x,
            'y' => ItemId::y,
            'z' => ItemId::z,
            'A' => ItemId::A,
            'B' => ItemId::B,
            'C' => ItemId::C,
            'D' => ItemId::D,
            'E' => ItemId::E,
            'F' => ItemId::F,
            'G' => ItemId::G,
            'H' => ItemId::H,
            'I' => ItemId::I,
            'J' => ItemId::J,
            'K' => ItemId::K,
            'L' => ItemId::L,
            'M' => ItemId::M,
            'N' => ItemId::N,
            'O' => ItemId::O,
            'P' => ItemId::P,
            'Q' => ItemId::Q,
            'R' => ItemId::R,
            'S' => ItemId::S,
            'T' => ItemId::T,
            'U' => ItemId::U,
            'V' => ItemId::V,
            'W' => ItemId::W,
            'X' => ItemId::X,
            'Y' => ItemId::Y,
            'Z' => ItemId::Z,
            _ => panic!(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Item {
    pub prio: u8,
    mask: Mask,
}
impl From<ItemId> for Item {
    fn from(id: ItemId) -> Self {
        Item {
            prio: id as u8,
            mask: 0b1 << (id as u8 - 1),
        }
    }
}
impl From<char> for Item {
    fn from(c: char) -> Self {
        Item::from(ItemId::from(c))
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
    for ic in ItemId::iter() {
        let item = Item::from(ic);
        if item.mask & mask != 0 {
            out.push(item);
        }
    }
    out
}
