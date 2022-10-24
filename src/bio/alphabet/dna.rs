use crate::bio::alphabet::Alphabet;
use crate::bio::sequence::Seq;

#[derive(PartialEq, Debug)]
pub enum Nucleotides4 { A, C, G, T }

const NUCLEOTIDES4: [Nucleotides4; 4] = [
    Nucleotides4::A,
    Nucleotides4::C,
    Nucleotides4::G,
    Nucleotides4::T
];

pub struct Dna4;

impl Dna4 {
    pub(crate) fn new(symbols: Vec<char>) -> Seq<Dna4> {
        return Seq {
            alphabet: Dna4,
            residue: symbols,
        };
    }
}

impl Alphabet for Dna4 {
    type Elems = Nucleotides4;

    fn size(&self) -> usize {
        4
    }

    fn elements(&self) -> &[Nucleotides4] {
        &NUCLEOTIDES4
    }

    fn char(symbol: char) -> Nucleotides4 {
        match symbol {
            'a' | 'A' => Nucleotides4::A,
            'c' | 'C' => Nucleotides4::C,
            'g' | 'G' => Nucleotides4::G,
            't' | 'T' => Nucleotides4::T,
            _ => Nucleotides4::A
        }
    }
}

#[test]
fn standard_iupac_alphabet() {
    assert_eq!(Dna4.size(), 4);
    assert_eq!(Dna4.elements(), NUCLEOTIDES4);
}
