use crate::bio::alphabet::Alphabet;

#[derive(PartialEq, Debug)]
pub enum RiboNucleotides4 { A, C, G, U }

const RIBO_NUCLEOTIDES4: [RiboNucleotides4; 4] = [
    RiboNucleotides4::A,
    RiboNucleotides4::C,
    RiboNucleotides4::G,
    RiboNucleotides4::U
];

struct Rna4;

impl Alphabet for Rna4 {
    type Elems = RiboNucleotides4;

    fn size(&self) -> usize {
        4
    }

    fn elements(&self) -> &[RiboNucleotides4] {
        &RIBO_NUCLEOTIDES4
    }

    fn char(symbol: char) -> RiboNucleotides4 {
        match symbol {
            'a' | 'A' => RiboNucleotides4::A,
            'c' | 'C' => RiboNucleotides4::C,
            'g' | 'G' => RiboNucleotides4::G,
            'u' | 'U' => RiboNucleotides4::U,
            _ => RiboNucleotides4::A
        }
    }
}

