use std::cmp::max;

use crate::bio::alphabet::Alphabet;
use crate::bio::sequence::Seq;
use crate::bio::util::Table;

use super::distance::Distance;

#[derive(Debug)]
pub struct Global<'a, A: Alphabet, D: Distance<A::Elems>> {
    seq1: &'a Seq<A>,
    seq2: &'a Seq<A>,
    metric: &'a D,
    score: i32,
}

impl<'a, A, D> Global<'a, A, D>
where
    A: Alphabet,
    D: Distance<A::Elems>,
{
    fn new(seq1: &'a Seq<A>, seq2: &'a Seq<A>, metric: &'a D, score: i32) -> Self {
        return Global { seq1, seq2, metric, score }
    }

    pub fn align(seq1: &'a Seq<A>, seq2: &'a Seq<A>, metric: &'a D) -> Global<'a, A, D> {
        let m = seq1.length() + 1;
        let n = seq2.length() + 1;
        let mut matrix = Table::new(m, n);

        for i in 0..m {
            matrix.set(i, 0, metric.indel() * i as i32)
        }

        for j in 0..n {
            matrix.set(0, j, metric.indel() * j as i32)
        }

        for i in 1..m {
            for j in 1..n {
                let mat = matrix.get(i - 1, j - 1) + metric.cmp(&seq1.at(i - 1), &seq2.at(j - 1));
                let del = matrix.get(i - 1, j) + metric.indel();
                let ins = matrix.get(i, j - 1) + metric.indel();

                matrix.set(i, j, max(ins, max(mat, del)))
            }
        }

        Global::new( seq1, seq2, metric, *matrix.get(m - 1, n - 1))
    }
}

#[cfg(test)]
mod tests {
    use crate::bio::alphabet::dna::Dna4;
    use crate::bio::alignment::distance::Basic;

    use super::Global;

    #[test]
    fn it_is_possible_to_create_an_alignment() {
        let seq1 = Dna4::new(vec!['A', 'C', 'G', 'T']);
        let seq2 = Dna4::new(vec!['A', 'C', 'G', 'T']);
        let distance = Basic::new(1, -1, 10);

        Global {
            seq1: &seq1,
            seq2: &seq2,
            metric: &distance,
            score: 42,
        };
    }

    #[test]
    fn create_alignment() {
        let distance = Basic::new(1, -1, -1);
        let seq1 = Dna4::new(vec!['G', 'A', 'T', 'T', 'A', 'C', 'A']);
        let seq2 = Dna4::new(vec!['G', 'C', 'A', 'T', 'G', 'C', 'G']);

        let align = Global::align(
            &seq1,
            &seq2,
            &distance
        );

        assert_eq!(align.score, 0)
    }
}