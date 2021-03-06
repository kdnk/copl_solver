use super::Nat;

#[derive(PartialEq, Debug, Clone)]
pub enum ProofPlusIs {
    PZero,
    PSucc(Box<ProofPlusIs>),
}

impl ProofPlusIs {
    pub fn check(&self, a: &Nat, b: &Nat, c: &Nat) -> bool {
        match self {
            Self::PZero => a == &Nat::Z && b == c,
            Self::PSucc(proof1) => match (a, c) {
                (Nat::S(nat0), Nat::S(nat1)) => proof1.check(nat0, b, nat1),
                _ => false,
            },
        }
    }

    pub fn find(a: &Nat, b: &Nat, c: &Nat) -> Option<Self> {
        match (a, c) {
            (Nat::Z, _) if b == c => Some(Self::PZero),
            (Nat::S(nat1), Nat::S(nat3)) => match Self::find(nat1, b, nat3) {
                None => None,
                Some(proof0) => Some(Self::PSucc(Box::new(proof0))),
            },
            _ => None,
        }
    }

    pub fn find_n4(a: &Nat, b: &Nat) -> Option<(Self, Nat)> {
        match a {
            Nat::Z => Some((Self::PZero, b.clone())),
            Nat::S(nat1) => {
                let (proof, nat) = Self::find_n4(nat1, b)?;
                Some((Self::PSucc(Box::new(proof)), Nat::s(nat)))
            }
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum ProofTimeIs {
    TZero,
    TSucc(Box<ProofTimeIs>, Box<ProofPlusIs>, Nat),
}

impl ProofTimeIs {
    pub fn check(&self, a: &Nat, b: &Nat, c: &Nat) -> bool {
        match self {
            Self::TZero => a == &Nat::Z && c == &Nat::Z,
            Self::TSucc(proof0, proof1, nat3) => match (a, b, c) {
                (Nat::S(nat1), nat2, nat4) => {
                    proof0.check(nat1, nat2, nat3) && proof1.check(nat2, nat3, nat4)
                }
                _ => false,
            },
        }
    }

    pub fn find(a: &Nat, b: &Nat, c: &Nat) -> Option<Self> {
        match (a, b, c) {
            (Nat::Z, _, Nat::Z) => Some(Self::TZero),
            (Nat::S(nat1), nat2, nat4) => {
                let (proof1, nat3) = Self::find_n3(nat1, nat2)?;
                let proof2 = ProofPlusIs::find(nat2, &nat3, nat4)?;

                Some(Self::TSucc(Box::new(proof1), Box::new(proof2), nat3))
            }
            _ => None,
        }
    }

    pub fn find_n3(a: &Nat, b: &Nat) -> Option<(Self, Nat)> {
        match a {
            Nat::Z => Some((Self::TZero, Nat::Z)),
            Nat::S(nat1) => {
                let (proof1, nat3) = Self::find_n3(nat1, b)?;
                let (proof2, nat4) = ProofPlusIs::find_n4(b, &nat3)?;

                let found_proof = Self::TSucc(Box::new(proof1), Box::new(proof2), nat3);
                Some((found_proof, nat4))
            }
        }
    }
}
