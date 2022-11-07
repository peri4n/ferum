pub mod dna;
pub mod rna;

use crate::bio::alphabet::dna::Nuc4;
use crate::bio::alphabet::rna::RNuc4;

pub trait Alphabet {
    type Elems: Eq;

    fn size(&self) -> usize;

    fn elements(&self) -> &[Self::Elems];

    fn char(symbol: char) -> Self::Elems;
}

trait Transcribe<A, B> {
    fn transcribe(symbol: A) -> B;
}

struct Dna4ToRna4;

impl Transcribe<Nuc4, RNuc4> for Dna4ToRna4 {
    fn transcribe(symbol: Nuc4) -> RNuc4 {
        match symbol {
            Nuc4::A => RNuc4::A,
            Nuc4::C => RNuc4::C,
            Nuc4::G => RNuc4::G,
            Nuc4::T => RNuc4::U
        }
    }
}
