pub mod dna;
pub mod rna;

pub trait Alphabet {
    type Elems: Eq;

    const SIZE: usize;

    fn elements(&self) -> &[Self::Elems];

    fn char(&self, symbol: char) -> &Self::Elems;

    fn bits_per_element(&self) -> u32 {
        Self::SIZE.ilog2()
    }
}
