use crate::bio::alphabet::Alphabet;
use crate::bio::sequence::Seq;
use crate::bio::alphabet::dna::Dna4;

struct Alignment<'a, A: Alphabet> {
    seq1: &'a Seq<A>,
    seq2: &'a Seq<A>,
    score: u32,
}

#[test]
fn it_compiles() {
    let seq1 = Dna4::new(vec!('A', 'C', 'G', 'T'));
    let seq2 = Dna4::new(vec!('A', 'C', 'G', 'T'));

    Alignment {
        seq1: &seq1,
        seq2: &seq2,
        score: 42,
    };
}
