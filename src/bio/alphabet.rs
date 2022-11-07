pub mod dna;
pub mod rna;

use crate::bio::alphabet::dna::Nuc4;
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

impl Transcribe<Nuc4, RiboNucleotides4> for Dna4ToRna4 {
    fn transcribe(symbol: Nuc4) -> RiboNucleotides4 {
        match symbol {
            Nuc4::A => RiboNucleotides4::A,
            Nuc4::C => RiboNucleotides4::C,
            Nuc4::G => RiboNucleotides4::G,
            Nuc4::T => RiboNucleotides4::U
        }
    }
}
