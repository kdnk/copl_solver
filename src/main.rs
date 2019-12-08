#[derive(PartialEq, Debug)]
enum Nat {
    Z,
    S(Box<Nat>),
}

#[derive(PartialEq, Debug)]
enum ProofPlusIs {
    PZero,
    PSucc(Box<ProofPlusIs>),
}

impl ProofPlusIs {
    pub fn check(&self, a: &Nat, b: &Nat, c: &Nat) -> bool {
        match self {
            ProofPlusIs::PZero => a == &Nat::Z && b == c,
            ProofPlusIs::PSucc(proof1) => match (a, c) {
                (Nat::S(nat0), Nat::S(nat1)) => proof1.check(nat0, b, nat1),
                _ => false,
            },
        }
    }

    pub fn find(a: &Nat, b: &Nat, c: &Nat) -> Option<Self> {
        match (a, c) {
            (Nat::Z, _) => Some(ProofPlusIs::PZero),
            // S(Z) plus S(Z) is S(S(Z))
            (Nat::S(nat1), Nat::S(nat2)) => match Self::find(nat1, b, nat2) {
                // Z plus S(Z) is S(Z)
                None => None,
                Some(proof0) => Some(ProofPlusIs::PSucc(Box::new(proof0))), // <- なんでここを box にしないといけない？
            },
            _ => None,
        }
    }
}

fn main() {}

#[test]
fn succ() {
    let n1 = Nat::S(Box::new(Nat::Z));
    let n2 = Nat::S(Box::new(Nat::Z));
    let n3 = Nat::S(Box::new(Nat::S(Box::new(Nat::Z))));
    let proof = ProofPlusIs::find(&n1, &n2, &n3);

    assert_eq!(
        proof,
        Some(ProofPlusIs::PSucc(Box::new(ProofPlusIs::PZero)))
    );

    let checked = proof.unwrap().check(&n1, &n2, &n3);
    assert_eq!(checked, true);
}

#[test]
fn zero() {
    let n1 = Nat::Z;
    let n2 = Nat::S(Box::new(Nat::Z));
    let n3 = Nat::S(Box::new(Nat::Z));
    let proof = ProofPlusIs::find(&n1, &n2, &n3);

    assert_eq!(proof, Some(ProofPlusIs::PZero));

    let checked = proof.unwrap().check(&n1, &n2, &n3);
    assert_eq!(checked, true);
}
