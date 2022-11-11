use std::cmp::max;

use crate::bio::alphabet::dna::Nuc4;
use crate::bio::alphabet::Alphabet;
use crate::bio::sequence::Seq;
use crate::bio::util::Table;

pub trait Distance<A>
where
    A: Eq,
{
    fn cmp(&self, first: &A, second: &A) -> i32;

    fn indel(&self) -> i32;
}

#[derive(Debug)]
pub struct Basic {
    same: i32,
    different: i32,
    indel: i32,
}

impl Distance<Nuc4> for Basic {
    fn cmp(&self, first: &Nuc4, second: &Nuc4) -> i32 {
        if *first == *second {
            return self.same;
        } else {
            return self.different;
        }
    }

    fn indel(&self) -> i32 {
        self.indel
    }
}

#[derive(Debug)]
pub struct Alignment<'a, A: Alphabet, D: Distance<A::Elems>> {
    seq1: &'a Seq<A>,
    seq2: &'a Seq<A>,
    metric: &'a D,
    score: i32,
}

impl<'a, A, D> Alignment<'a, A, D>
where
    A: Alphabet,
    D: Distance<A::Elems>,
{
    pub fn new(seq1: &'a Seq<A>, seq2: &'a Seq<A>, metric: &'a D) -> Alignment<'a, A, D> {
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
                let mat = matrix.get(i - 1, j - 1) + metric.cmp(&seq1.at(i-1), &seq2.at(j-1));
                let del = matrix.get(i - 1, j) + metric.indel();
                let ins = matrix.get(i, j - 1) + metric.indel();

                matrix.set(i, j, max(ins, max(mat, del)))
            }
        }

        Alignment {
            seq1,
            seq2,
            metric,
            score: *matrix.get(m - 1, n - 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bio::alignment::Basic;
    use crate::bio::alphabet::dna::Dna4;
    use crate::bio::alphabet::dna::Nuc4::{A, C, G, T};

    use super::{Alignment, Distance};

    #[test]
    fn it_is_possible_to_create_an_alignment() {
        let seq1 = Dna4::new(vec!['A', 'C', 'G', 'T']);
        let seq2 = Dna4::new(vec!['A', 'C', 'G', 'T']);
        let distance = Basic {
            same: 1,
            different: -1,
            indel: 10,
        };

        Alignment {
            seq1: &seq1,
            seq2: &seq2,
            metric: &distance,
            score: 42,
        };
    }

    #[test]
    fn it_is_possible_to_create_a_distance_metric() {
        let distance = Basic {
            same: 1,
            different: -1,
            indel: 10,
        };
        assert_eq!(distance.indel(), 10);

        assert_eq!(distance.cmp(&A, &A), 1);
        assert_eq!(distance.cmp(&C, &C), 1);
        assert_eq!(distance.cmp(&G, &G), 1);
        assert_eq!(distance.cmp(&T, &T), 1);

        assert_eq!(distance.cmp(&A, &C), -1);
        assert_eq!(distance.cmp(&C, &G), -1);
        assert_eq!(distance.cmp(&G, &T), -1);
        assert_eq!(distance.cmp(&T, &A), -1);
    }

    #[test]
    fn create_alignment() {
        let seq1 = Dna4::new(vec!['G', 'A', 'T', 'T', 'A', 'C', 'A']);
        let seq2 = Dna4::new(vec!['G', 'C', 'A', 'T', 'G', 'C', 'G']);

        let align = Alignment::new(
            &seq1,
            &seq2,
            &Basic {
                same: 1,
                different: -1,
                indel: -1,
            },
        );

        assert_eq!(align.score, 0)
    }
}
