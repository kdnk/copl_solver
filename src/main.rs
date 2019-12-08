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

#[derive(PartialEq, Debug)]
enum ProofTimeIs {
    TZero,
    TSucc(Box<ProofTimeIs>, Box<ProofPlusIs>, Box<Nat>),
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
            (Nat::S(nat1), Nat::S(nat3)) => match Self::find(nat1, b, nat3) {
                // Z plus S(Z) is S(Z)
                None => None,
                Some(proof0) => Some(ProofPlusIs::PSucc(Box::new(proof0))), // <- なんでここを box にしないといけない？
            },
            _ => None,
        }
    }
}

impl ProofTimeIs {
    pub fn check(&self, a: &Nat, b: &Nat, c: &Nat) -> bool {
        match self {
            ProofTimeIs::TZero => a == &Nat::Z && c == &Nat::Z,
            ProofTimeIs::TSucc(proof0, proof1, nat3) => match (a, b, c) {
                (Nat::S(nat1), nat2, nat4) => {
                    proof0.check(nat1, nat2, nat3) && proof1.check(nat2, nat3, nat4)
                }
                _ => false,
            },
        }
    }

    pub fn find(a: &Nat, b: &Nat, c: &Nat) -> Option<Self> {
        match (a, b, c) {
            (Nat::Z, _, _) => Some(ProofTimeIs::TZero),
            // exp: S(S(Z)) times S(Z) is S(S(Z))
            (Nat::S(nat1), nat2, nat4) => {
                // proof1: S(Z) times S(Z) is S(Z)
                // proof2:  S(Z) plus S(Z) is S(S(Z))
                match (
                    Self::find(nat1, nat2, nat3),
                    ProofPlusIs::find(nat2, nat3, nat4),
                ) {
                    (Some(proof1), Some(proof2)) => {
                        Some(ProofTimeIs::TSucc(Box::new(proof1), Box::new(proof2), nat3))
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

fn main() {}

#[test]
fn p_succ() {
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
fn p_zero() {
    let n1 = Nat::Z;
    let n2 = Nat::S(Box::new(Nat::Z));
    let n3 = Nat::S(Box::new(Nat::Z));
    let proof = ProofPlusIs::find(&n1, &n2, &n3);

    assert_eq!(proof, Some(ProofPlusIs::PZero));

    let checked = proof.unwrap().check(&n1, &n2, &n3);
    assert_eq!(checked, true);
}
