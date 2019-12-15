use super::super::nat::Nat;

#[derive(PartialEq, Debug, Clone)]
pub enum ProofCompareNat3 {
    LSucc,
    LSuccR(Box<ProofCompareNat3>),
}

impl ProofCompareNat3 {
    pub fn find(n1: &Nat, n2: &Nat) -> Option<Self> {
        match (n1, n2) {
            (nat1, Nat::S(nat2)) if Box::new(nat1.clone()) == nat2.clone() => Some(Self::LSucc),
            (nat1, Nat::S(nat2)) => {
                let proof = Self::find(nat1, nat2)?;
                Some(Self::LSuccR(Box::new(proof)))
            }
            (_, Nat::Z) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_1() {
        let n1 = Nat::s(Nat::Z);
        let n2 = Nat::s(Nat::s(Nat::Z));
        let proof = ProofCompareNat3::find(&n1, &n2);

        assert_eq!(proof, Some(ProofCompareNat3::LSucc));
    }

    #[test]
    fn test_find_2() {
        let n0_1 = Nat::Z;
        let n0_2 = Nat::Z;
        let proof = ProofCompareNat3::find(&n0_1, &n0_2);

        assert_eq!(proof, None);
    }

    #[test]
    fn test_find_3() {
        let n2 = Nat::s(Nat::s(Nat::Z));
        let n5 = Nat::s(Nat::s(Nat::s(Nat::s(Nat::s(Nat::Z)))));
        let proof = ProofCompareNat3::find(&n2, &n5);

        assert_eq!(
            proof,
            Some(ProofCompareNat3::LSuccR(Box::new(
                ProofCompareNat3::LSuccR(Box::new(ProofCompareNat3::LSucc))
            )))
        );
    }
}
