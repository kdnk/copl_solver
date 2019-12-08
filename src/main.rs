#[derive(PartialEq)]
enum Nat {
    Z,
    S(Box<Nat>),
}

enum ProofPlusIs {
    PZero,
    PSucc(Box<ProofPlusIs>),
}

impl ProofPlusIs {
    pub fn check(&self, a: &Nat, b: &Nat, c: &Nat, proof: &ProofPlusIs) -> bool {
        match proof {
            ProofPlusIs::PZero => a == &Nat::Z && b == c,
            ProofPlusIs::PSucc(proof1) => match (a, c) {
                (Nat::S(nat0), Nat::S(nat1)) => self.check(nat0, b, nat1, proof1),
                _ => false,
            },
        }
    }

    pub fn find(&self, a: &Nat, b: &Nat, c: &Nat) -> Option<ProofPlusIs> {
        match (a, c) {
            (Nat::Z, _) => Some(ProofPlusIs::PZero),
            // S(Z) plus S(Z) is S(S(Z))
            (Nat::S(nat1), Nat::S(nat2)) => match self.find(nat1, b, nat2) {
                // Z plus S(Z) is S(Z)
                None => None,
                Some(proof0) => Some(ProofPlusIs::PSucc(Box::new(proof0))), // <- なんでここを box にしないといけない？
            },
            _ => None,
        }
    }
}

fn main() {}
