use crate::bio::alphabet::Alphabet;
use crate::bio::sequence::Seq;

use super::alphabet::dna::Nuc4;
pub trait DistanceMetric<A> where A: Eq {

    fn cmp(first: &A, second: &A) -> i32;

    fn indel() -> i32;

}

pub struct StandardMetric;

impl DistanceMetric<Nuc4> for StandardMetric {
    fn cmp(first: &Nuc4, second: &Nuc4) -> i32 {
        if *first == *second {
            return 1;
        } else {
            return -1;
        }
    }

    fn indel() -> i32 {
        10
    }
}


pub struct Alignment<'a, A: Alphabet> {
    seq1: &'a Seq<A>,
    seq2: &'a Seq<A>,
    score: u32,
}

impl<'a, A: Alphabet> Alignment<'a, A> {
    pub fn new(first: &Seq<A>, second: &Seq<A>) -> Alignment<'a, A> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::bio::alphabet::dna::Dna4;
    use crate::bio::alphabet::dna::Nuc4::{A, C, G, T};
    use crate::bio::alignment::StandardMetric;

    use super::{Alignment, DistanceMetric};

    #[test]
    fn it_is_possible_to_create_an_alignment() {
        let seq1 = Dna4::new(vec!['A', 'C', 'G', 'T']);
        let seq2 = Dna4::new(vec!['A', 'C', 'G', 'T']);

        Alignment {
            seq1: &seq1,
            seq2: &seq2,
            score: 42,
        };
    }

    #[test]
    fn it_is_possible_to_create_a_distance_metric() {
        assert_eq!(StandardMetric::indel(), 10);

        assert_eq!(StandardMetric::cmp(&A, &A), 1);
        assert_eq!(StandardMetric::cmp(&C, &C), 1);
        assert_eq!(StandardMetric::cmp(&G, &G), 1);
        assert_eq!(StandardMetric::cmp(&T, &T), 1);

        assert_eq!(StandardMetric::cmp(&A, &C), -1);
        assert_eq!(StandardMetric::cmp(&C, &G), -1);
        assert_eq!(StandardMetric::cmp(&G, &T), -1);
        assert_eq!(StandardMetric::cmp(&T, &A), -1);
    }
}
