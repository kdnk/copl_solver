use super::super::nat::Nat;

#[derive(PartialEq, Debug, Clone)]

pub enum ProofCompareNat1 {
    LSucc,
    LTrans(Box<ProofCompareNat1>, Box<ProofCompareNat1>),
}

impl ProofCompareNat1 {
    pub fn check(&self, n1: &Nat, n3: &Nat) -> bool {
        match self {
            Self::LSucc => true,
            Self::LTrans(proof1, proof2) => {
                let (_, n2) = Self::find_n2(n1, n3);
                if n2 == None {
                    return false;
                }
                proof1.check(n1, &n2.clone().unwrap()) && proof2.check(&n2.clone().unwrap(), n3)
            }
        }
    }

    pub fn find_n2(n1: &Nat, n3: &Nat) -> (Option<Self>, Option<Nat>) {
        match (n1, n3) {
            (nat1, nat3) if nat1 == nat3 => (None, None),
            (nat1, Nat::S(nat3)) if Box::new(nat1.clone()) == nat3.clone() => {
                (Some(Self::LSucc), Some(nat1.clone()))
            }
            (nat1, nat3) => {
                let nat2 = Nat::S(Box::new(nat1.clone()));
                let (proof1, n3) = Self::find_n2(&nat2, nat3);

                // assert_eq!(proof1, None);
                assert_ne!(n3, None);
                if proof1 == None {
                    return (None, None);
                }

                let found_proof = Self::LTrans(Box::new(Self::LSucc), Box::new(proof1.unwrap()));
                (Some(found_proof), n3)
            }
        }
    }

    pub fn find(n1: &Nat, n3: &Nat) -> Option<Self> {
        match (n1, n3) {
            (nat1, nat3) if nat1 == nat3 => None,
            (nat1, Nat::S(nat3)) if Box::new(nat1) == Box::new(nat3) => Some(Self::LSucc),
            (nat1, nat3) => {
                let (_proof, nat2) = Self::find_n2(nat1, nat3);
                Self::find(nat1, &nat2.unwrap())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_n2_1() {
        let z1 = Nat::Z;
        let z2 = Nat::Z;
        let (proof, nat) = ProofCompareNat1::find_n2(&z1, &z2);
        assert_eq!(proof, None);
        assert_eq!(nat, None);
    }

    #[test]
    fn test_find_n2_2() {
        let n2 = Nat::s(Nat::s(Nat::Z));
        let n3 = Nat::s(Nat::s(Nat::s(Nat::Z)));
        let (proof, nat) = ProofCompareNat1::find_n2(&n2, &n3);
        assert_eq!(proof, Some(ProofCompareNat1::LSucc));
        assert_eq!(nat, Some(n2));
    }

    #[test]
    fn test_find_n2_3() {
        let n2 = Nat::s(Nat::s(Nat::Z));
        let n4 = Nat::s(Nat::s(Nat::s(Nat::s(Nat::Z))));
        let (proof, nat) = ProofCompareNat1::find_n2(&n2, &n4);

        let n3 = Nat::s(Nat::s(Nat::s(Nat::Z)));
        assert_eq!(
            proof,
            Some(ProofCompareNat1::LTrans(
                Box::new(ProofCompareNat1::LSucc),
                Box::new(ProofCompareNat1::LSucc),
            ))
        );
        assert_eq!(nat, Some(n3));
    }
}
