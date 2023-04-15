pub mod dna;
pub mod rna;

pub trait Alphabet {
    type Elems: Eq + Copy;

    const SIZE: usize;

    const BITS_PER_ELEMENT: usize = Self::SIZE.ilog2() as usize;

    fn elements(&self) -> &[Self::Elems];

    fn char(&self, symbol: char) -> &Self::Elems;
}
