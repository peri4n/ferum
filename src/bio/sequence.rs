use std::ops::Index;

use crate::bio::alphabet::Alphabet;

use super::alphabet::dna::Dna4;

#[derive(Debug)]
pub struct Seq<'a, A>
where
    A: Alphabet,
{
    pub alphabet: &'a A,
    pub symbols: Vec<char>,
}

impl<'a> From<&str> for Seq<'a, Dna4> {
    fn from(str: &str) -> Self {
        Seq {
            alphabet: &Dna4,
            symbols: str.to_string().chars().collect(),
        }
    }
}

impl<'a, A: Alphabet> Index<usize> for Seq<'a, A> {
    type Output = A::Elems;

    fn index(&self, index: usize) -> &Self::Output {
        todo!()
    }
}

impl<'a, A: Alphabet> Seq<'a, A> {
    pub fn at(&self, index: usize) -> &A::Elems {
        self.alphabet.char(self.symbols[index])
    }

    pub fn length(&self) -> usize {
        self.symbols.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::bio::alphabet::dna::Nuc4::{A, C, G, T};
    use crate::bio::sequence::Seq;

    #[test]
    fn computes_the_length() {
        let seq: Seq<_> = "ACGT".into();

        assert_eq!(seq.length(), 4);
    }

    #[test]
    fn is_accessible_by_index() {
        let seq: Seq<_> = "ACGT".into();

        assert_eq!(seq.at(0), &A);
        assert_eq!(seq.at(1), &C);
        assert_eq!(seq.at(2), &G);
        assert_eq!(seq.at(3), &T);
    }
}
