use crate::bio::alphabet::Alphabet;
use crate::bio::sequence::Seq;

#[derive(PartialEq, Eq, Debug)]
pub enum Nuc4 {
    A,
    C,
    G,
    T,
}

pub(crate) const NUCLEOTIDES4: [Nuc4; 4] = [Nuc4::A, Nuc4::C, Nuc4::G, Nuc4::T];

#[derive(Debug)]
pub struct Dna4;

impl Dna4 {
    pub fn new(symbols: Vec<char>) -> Seq<Dna4> {
        return Seq {
            alphabet: Dna4,
            residue: symbols,
        };
    }
}

impl Alphabet for Dna4 {
    type Elems = Nuc4;

    fn size(&self) -> usize {
        4
    }

    fn elements(&self) -> &[Nuc4] {
        &NUCLEOTIDES4
    }

    fn char(symbol: char) -> Nuc4 {
        match symbol {
            'a' | 'A' => Nuc4::A,
            'c' | 'C' => Nuc4::C,
            'g' | 'G' => Nuc4::G,
            't' | 'T' => Nuc4::T,
            _ => Nuc4::A,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bio::alphabet::Alphabet;
    use crate::bio::alphabet::dna::{NUCLEOTIDES4, Nuc4};
    use super::Dna4;


    #[test]
    fn standard_iupac_alphabet() {
        assert_eq!(Dna4.size(), 4);
        assert_eq!(Dna4.elements(), NUCLEOTIDES4);
    }

    #[test]
    fn create_new_seq() {
        let seq = Dna4::new(vec!['a', 'c', 'g', 't']);

        assert_eq!(seq.at(0), Nuc4::A);
        assert_eq!(seq.at(1), Nuc4::C);
        assert_eq!(seq.at(2), Nuc4::G);
        assert_eq!(seq.at(3), Nuc4::T);
    }
}
