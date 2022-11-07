use crate::bio::alphabet::Alphabet;
use crate::bio::sequence::Seq;

use super::alphabet::dna::Nuc4;
pub trait Distance<A>
where
    A: Eq,
{
    fn cmp(&self, first: &A, second: &A) -> i32;

    fn indel(&self) -> i32;
}

pub struct Basic {
    same: i32,
    different: i32,
    indel: i32
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

pub struct Alignment<'a, A: Alphabet, D: Distance<A::Elems>> {
    seq1: &'a Seq<A>,
    seq2: &'a Seq<A>,
    metric: &'a D,
    score: u32,
}

impl<'a, A, D> Alignment<'a, A, D>
where
    A: Alphabet,
    D: Distance<A::Elems>,
{
    pub fn new(first: &Seq<A>, second: &Seq<A>, metric: D) -> Alignment<'a, A, D> {
        todo!();
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
        let distance = Basic { same:1, different: -1, indel: 10};

        Alignment {
            seq1: &seq1,
            seq2: &seq2,
            metric: &distance,
            score: 42,
        };
    }

    #[test]
    fn it_is_possible_to_create_a_distance_metric() {
        let distance = Basic { same:1, different: -1, indel: 10};
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
}
