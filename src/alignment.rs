use crate::sequence::Seq;

struct Alignment<'a> {
    seq1: &'a Seq,
    seq2: &'a Seq,
    score: u32,
}

#[test]
fn it_compiles() {
    let seq1 = Seq { residue: vec!('A', 'C', 'G', 'T') };
    let seq2 = Seq { residue: vec!('A', 'C', 'G', 'T') };

    Alignment {
        seq1: &seq1,
        seq2: &seq2,
        score: 42,
    };
}
