use super::alphabet::Finite;

struct BitEnc<A: Finite> {
    storage: Vec<u8>,
    length: usize,
    alphabet: A,
}

impl <A: Finite> BitEnc<A> {
    pub fn new() {
        let bits_to_store = 2;
        todo!()
    }
}
