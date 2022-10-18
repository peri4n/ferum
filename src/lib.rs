#[cfg(test)]
mod bio {
    use std::ops::Index;

    struct Seq {
        residue: Vec<char>,
    }

    impl Index<usize> for Seq {
        type Output = char;

        fn index(&self, index: usize) -> &char {
            &self.residue[index]
        }
    }

    impl Seq {
        fn length(&self) -> usize {
            return self.residue.len();
        }
    }

    #[test]
    fn computes_the_length() {
        let seq = Seq { residue: vec!('A', 'C', 'G', 'T') };

        assert_eq!(seq.length(), 4);
    }

    #[test]
    fn is_accessible_by_index() {
        let seq = Seq { residue: vec!('A', 'C', 'G', 'T') };

        assert_eq!(seq[0], 'A');
        assert_eq!(seq[1], 'C');
        assert_eq!(seq[2], 'G');
        assert_eq!(seq[3], 'T');
    }
}
