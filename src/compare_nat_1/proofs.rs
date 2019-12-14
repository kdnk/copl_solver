use super::super::nat::Nat;

#[derive(PartialEq, Debug, Clone)]
pub enum ProofCompareNat1 {
    LSucc,
    LTrans(Box<ProofCompareNat1>, Box<ProofCompareNat1>),
}

impl ProofCompareNat1 {
    pub fn find(n1: &Nat, n3: &Nat) -> Option<Self> {
        match (n1, n3) {
            (nat1, Nat::S(nat3)) if Box::new(nat1.clone()) == nat3.clone() => Some(Self::LSucc),
            (nat1, nat3) => {
                let (proof1, n2) = Self::find_n2(nat1, nat3)?;
                if n2 == None {
                    return None;
                }
                let proof2 = Self::find(&n2.unwrap(), nat3)?;
                let found_proof = Self::LTrans(Box::new(proof1), Box::new(proof2));
                Some(found_proof)
            }
        }
    }

    pub fn find_n2(n1: &Nat, n3: &Nat) -> Option<(Self, Option<Nat>)> {
        match (n1, n3) {
            (nat1, nat3) if nat1 == nat3 => None,
            (nat1, Nat::S(nat3)) if Box::new(nat1.clone()) == nat3.clone() => {
                Some((Self::LSucc, None))
            }
            (nat1, nat3) => {
                let (proof, nat) = Self::find_n2(&Nat::s(nat1.clone()), nat3)?;
                if nat == None {
                    return None;
                }
                let found_proof = Self::LTrans(Box::new(Self::LSucc), Box::new(proof));
                Some((found_proof, Some(Nat::s(nat1.clone()))))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_n2_1() {
        let z_1 = Nat::Z;
        let z_2 = Nat::Z;
        let result = ProofCompareNat1::find_n2(&z_1, &z_2);
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_n2_2() {
        let n2 = Nat::s(Nat::s(Nat::Z));
        let n3 = Nat::s(Nat::s(Nat::s(Nat::Z)));
        let result = ProofCompareNat1::find_n2(&n2, &n3);

        assert_ne!(result, None);
        let (proof, _nat) = result.unwrap();
        assert_eq!(proof, ProofCompareNat1::LSucc);
    }

    #[test]
    fn test_find_n2_3() {
        let n2 = Nat::s(Nat::s(Nat::Z));
        let n4 = Nat::s(Nat::s(Nat::s(Nat::s(Nat::Z))));
        let result = ProofCompareNat1::find_n2(&n2, &n4);

        assert_ne!(result, None);
        let (proof, nat) = result.unwrap();
        let n3 = Nat::s(Nat::s(Nat::s(Nat::Z)));
        assert_eq!(
            proof,
            ProofCompareNat1::LTrans(
                Box::new(ProofCompareNat1::LSucc),
                Box::new(ProofCompareNat1::LSucc),
            )
        );
        assert_eq!(nat, Some(n3));
    }
}
