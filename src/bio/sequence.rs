use crate::bio::alphabet::Alphabet;
use crate::bio::alphabet::dna::Dna4;

#[derive(Debug)]
pub struct Seq<A: Alphabet> {
    pub alphabet: A,
    pub residue: Vec<char>,
}

#[derive(PartialEq, Debug)]
pub struct Count {
    a: i32,
    c: i32,
    g: i32,
    t: i32,
    unknown: i32,
}

impl <A: Alphabet> Seq<A> {
    pub fn at(&self, index: usize) -> A::Elems {
        A::char(self.residue[index])
    }

    pub fn length(&self) -> usize {
        return self.residue.len();
    }
}

impl Seq<Dna4> {

    pub fn count(&self) -> Count {
        let mut a = 0;
        let mut c = 0;
        let mut g = 0;
        let mut t = 0;
        let mut unknown = 0;
        for x in &self.residue {
            match x {
                'a' | 'A' => a += 1,
                'c' | 'C' => c += 1,
                'g' | 'G' => g += 1,
                't' | 'T' => t += 1,
                _ => unknown += 1
            }
        }
        Count { a, c, g, t, unknown }
    }

}

#[cfg(test)]
mod tests {
    use crate::bio::alphabet::dna::Nuc4::{A, C, G, T};
    use crate::bio::alphabet::dna::Dna4;

    use super::Count;

    #[test]
    fn computes_the_length() {
        let seq = Dna4::new(vec!('A', 'C', 'G', 'T'));

        assert_eq!(seq.length(), 4);
    }

    #[test]
    fn counts_standard_nucleotides() {
        let seq = Dna4::new(vec!('A', 'c', 'G', 't'));

        assert_eq!(seq.count(), Count { a: 1, c: 1, g: 1, t: 1, unknown: 0 })
    }

    #[test]
    fn counts_unknown_nucleotides() {
        let seq = Dna4::new(vec!('A', 'b', 'G', 'X'));

        assert_eq!(seq.count(), Count { a: 1, c: 0, g: 1, t: 0, unknown: 2 })
    }

    #[test]
    fn is_accessible_by_index() {
        let seq = Dna4::new(vec!('A', 'C', 'G', 'T'));

        assert_eq!(seq.at(0), A);
        assert_eq!(seq.at(1), C);
        assert_eq!(seq.at(2), G);
        assert_eq!(seq.at(3), T);
    }
}
