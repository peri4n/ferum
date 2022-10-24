pub(crate) mod dna;
pub(crate) mod rna;

use crate::bio::alphabet::dna::Nucleotides4;
use crate::bio::alphabet::rna::RiboNucleotides4;

pub trait Alphabet {
    type Elems;

    fn size(&self) -> usize;

    fn elements(&self) -> &[Self::Elems];

    fn char(symbol: char) -> Self::Elems;
}

trait Transcribe<A, B> {
    fn transcribe(symbol: A) -> B;
}

struct Dna4ToRna4;

impl Transcribe<Nucleotides4, RiboNucleotides4> for Dna4ToRna4 {
    fn transcribe(symbol: Nucleotides4) -> RiboNucleotides4 {
        match symbol {
            Nucleotides4::A => RiboNucleotides4::A,
            Nucleotides4::C => RiboNucleotides4::C,
            Nucleotides4::G => RiboNucleotides4::G,
            Nucleotides4::T => RiboNucleotides4::U
        }
    }
}
