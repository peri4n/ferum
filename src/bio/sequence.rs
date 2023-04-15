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
        self.alphabet.char(self.symbols[index])
    }
}

pub struct SeqIter<'a, A: Alphabet> {
    alphabet: &'a A,
    inner: <Vec<char> as IntoIterator>::IntoIter
}

impl<'a, A: Alphabet> Iterator for SeqIter<'a, A> {
    type Item = A::Elems;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|c| *self.alphabet.char(c))
    }
}

impl<'a, A: Alphabet> IntoIterator for Seq<'a, A> {
    type Item = A::Elems;

    fn into_iter(self) -> Self::IntoIter {
        SeqIter {
            alphabet: self.alphabet,
            inner: self.symbols.into_iter()
        }
    }

    type IntoIter = SeqIter<'a, A>;
}

impl<'a, A: Alphabet> Seq<'a, A> {
    pub fn len(&self) -> usize {
        self.symbols.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::bio::alphabet::dna::Nuc4::*;
    use crate::bio::sequence::Seq;

    #[test]
    fn computes_the_length() {
        let seq: Seq<_> = "ACGT".into();

        assert_eq!(seq.len(), 4);
    }

    #[test]
    fn is_accessible_by_index() {
        let seq: Seq<_> = "ACGT".into();

        assert_eq!(seq[0], A);
        assert_eq!(seq[1], C);
        assert_eq!(seq[2], G);
        assert_eq!(seq[3], T);
    }

    #[test]
    fn can_be_converted_to_an_iterator() {
        let seq: Seq<_> = "ACGT".into();
        assert_eq!(seq.into_iter().last(), Some(T));
    }
}
