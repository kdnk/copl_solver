use super::Nat;

#[derive(PartialEq, Debug, Clone)]
pub enum ProofPlusIs {
    PZero,
    PSucc(Box<ProofPlusIs>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum ProofTimeIs {
    TZero,
    TSucc(Box<ProofTimeIs>, Box<ProofPlusIs>, Nat),
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
            (Nat::Z, _) if b == c => Some(ProofPlusIs::PZero),
            (Nat::S(nat1), Nat::S(nat3)) => match Self::find(nat1, b, nat3) {
                None => None,
                Some(proof0) => Some(ProofPlusIs::PSucc(Box::new(proof0))), // <- なんでここを box にしないといけない？
            },
            _ => None,
        }
    }

    pub fn find_n4(a: &Nat, b: &Nat) -> Option<(Self, Nat)> {
        match a {
            Nat::Z => Some((ProofPlusIs::PZero, b.clone())),
            Nat::S(nat1) => {
                let (proof, nat1) = Self::find_n4(nat1, b)?;
                Some((ProofPlusIs::PSucc(Box::new(proof)), Nat::s(nat1)))
            }
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
            (Nat::Z, _, Nat::Z) => Some(ProofTimeIs::TZero),
            // exp: S(S(Z)) times S(Z) is S(S(Z))
            (Nat::S(nat1), nat2, nat4) => {
                // proof1: S(Z) times S(Z) is S(Z)
                // proof2: S(Z) plus S(Z) is S(S(Z))
                let (proof1, nat3) = Self::find_n3(nat1, nat2)?;
                let proof2 = ProofPlusIs::find(nat2, &nat3, nat4)?;

                Some(ProofTimeIs::TSucc(Box::new(proof1), Box::new(proof2), nat3))
            }
            _ => None,
        }
    }

    pub fn find_n3(a: &Nat, b: &Nat) -> Option<(Self, Nat)> {
        match a {
            Nat::Z => Some((ProofTimeIs::TZero, Nat::Z)),
            // exp: S(S(Z)) times S(Z) is S(S(Z))
            Nat::S(nat1) => {
                // proof1: S(Z) times S(Z) is S(Z)
                // proof2: S(Z) plus S(Z) is S(S(Z))
                let (proof1, nat3) = Self::find_n3(nat1, b)?;
                let (proof2, nat4) = ProofPlusIs::find_n4(b, &nat3)?;

                let found_proof = ProofTimeIs::TSucc(Box::new(proof1), Box::new(proof2), nat3);
                Some((found_proof, nat4))
            }
        }
    }
}
