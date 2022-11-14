use crate::bio::alphabet::dna::Nuc4;

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

impl Basic {
    pub fn new(same: i32, different: i32, indel: i32) -> Self {
        return Basic {
            same,
            different,
            indel,
        };
    }
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

#[cfg(test)]
mod tests {
    use crate::bio::alignment::distance::{Basic, Distance};
    use crate::bio::alphabet::dna::Nuc4::{A, C, G, T};

    #[test]
    fn it_is_possible_to_create_a_distance_metric() {
        let distance = Basic::new(1, -1, 10);
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
