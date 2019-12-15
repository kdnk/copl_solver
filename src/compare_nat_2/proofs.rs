use super::super::nat::Nat;

#[derive(PartialEq, Debug, Clone)]
pub enum ProofCompareNat2 {
    LZero,
    LSuccSucc(Box<ProofCompareNat2>),
}

impl ProofCompareNat2 {
    pub fn find(n1: &Nat, n2: &Nat) -> Option<Self> {
        match (n1, n2) {
            (Nat::Z, Nat::S(_)) => Some(Self::LZero),
            (Nat::S(nat1), Nat::S(nat2)) => {
                let proof = Self::find(nat1, nat2)?;
                Some(Self::LSuccSucc(Box::new(proof)))
            }
            (_, _) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_1() {
        let n0_1 = Nat::Z;
        let n0_2 = Nat::Z;
        let proof = ProofCompareNat2::find(&n0_1, &n0_2);
        assert_eq!(proof, None);
    }

    #[test]
    fn test_find_2() {
        let n1 = Nat::s(Nat::Z);
        let n2 = Nat::s(Nat::s(Nat::Z));
        let proof = ProofCompareNat2::find(&n1, &n2);
        assert_eq!(
            proof,
            Some(ProofCompareNat2::LSuccSucc(Box::new(
                ProofCompareNat2::LZero
            )))
        );

        let n1 = Nat::s(Nat::Z);
        let n4 = Nat::s(Nat::s(Nat::s(Nat::s(Nat::Z))));
        let proof = ProofCompareNat2::find(&n1, &n4);
        assert_eq!(
            proof,
            Some(ProofCompareNat2::LSuccSucc(Box::new(
                ProofCompareNat2::LZero
            )))
        );
    }

    #[test]
    fn test_find_3() {
        let n2 = Nat::s(Nat::s(Nat::Z));
        let n3 = Nat::s(Nat::s(Nat::s(Nat::Z)));
        let proof = ProofCompareNat2::find(&n2, &n3);
        assert_eq!(
            proof,
            Some(ProofCompareNat2::LSuccSucc(Box::new(
                ProofCompareNat2::LSuccSucc(Box::new(ProofCompareNat2::LZero))
            )))
        );
    }
}
