use super::Alphabet;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum RNuc4 {
    A,
    C,
    G,
    U,
}

const RIBO_NUCLEOTIDES4: [RNuc4; 4] = [RNuc4::A, RNuc4::C, RNuc4::G, RNuc4::U];

pub struct Rna4;

impl Alphabet for Rna4 {
    type Elems = RNuc4;

    const SIZE: usize = 4;

    fn elements(&self) -> &[RNuc4] {
        &RIBO_NUCLEOTIDES4
    }

    fn char(&self, symbol: char) -> &RNuc4 {
        match symbol {
            'a' | 'A' => &RNuc4::A,
            'c' | 'C' => &RNuc4::C,
            'g' | 'G' => &RNuc4::G,
            'u' | 'U' => &RNuc4::U,
            _ => &RNuc4::A,
        }
    }
}
