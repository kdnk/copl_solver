use super::super::nat::Nat;

#[derive(PartialEq, Debug, Clone)]
pub enum ProofCompareNat1 {
    LSucc,
    LTrans(Box<ProofCompareNat1>, Box<ProofCompareNat1>, Nat),
}

impl ProofCompareNat1 {
    pub fn find(n1: &Nat, n3: &Nat) -> Option<Self> {
        match (n1, n3) {
            (nat1, Nat::S(nat3)) if nat1 == &**nat3 => {
                let proof = Some(Self::LSucc);
                proof
            }
            (nat1, nat3) => {
                let (proof2, nat2) = Self::find_n2(nat3)?;
                let proof1 = Self::find(nat1, &nat2)?;
                Some(Self::LTrans(Box::new(proof1), Box::new(proof2), nat2))
            }
        }
    }

    pub fn find_n2(n3: &Nat) -> Option<(Self, Nat)> {
        match n3 {
            Nat::Z => None,
            Nat::S(nat2) => {
                let proof = Self::LSucc;
                Some((proof, *nat2.clone()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // //  test for find_2
    // // -----------------------------------------------

    #[test]
    fn test_find_n2_1() {
        let z_1 = Nat::Z;
        let z_2 = Nat::Z;
        let result = ProofCompareNat1::find_n2(&z_2);
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_n2_2() {
        let n3 = Nat::s(Nat::s(Nat::s(Nat::Z)));
        let result = ProofCompareNat1::find_n2(&n3);

        assert_ne!(result, None);
        let (proof, _nat) = result.unwrap();
        assert_eq!(proof, ProofCompareNat1::LSucc);
    }

    #[test]
    fn test_find_n2_3() {
        let n4 = Nat::s(Nat::s(Nat::s(Nat::s(Nat::Z))));
        let result = ProofCompareNat1::find_n2(&n4);

        assert_ne!(result, None);
        let (proof, _nat) = result.unwrap();
        assert_eq!(proof, ProofCompareNat1::LSucc);
    }

    //  test for find
    // -----------------------------------------------

    #[test]
    fn test_find_1() {
        let n1 = Nat::s(Nat::Z);
        let n2 = Nat::s(Nat::s(Nat::Z));
        let proof = ProofCompareNat1::find(&n1, &n2);

        assert_eq!(proof, Some(ProofCompareNat1::LSucc));
    }

    #[test]
    fn test_find_2() {
        let n0_1 = Nat::Z;
        let n0_2 = Nat::Z;
        let proof = ProofCompareNat1::find(&n0_1, &n0_2);

        assert_eq!(proof, None);
    }

    #[test]
    fn test_find_3() {
        let n2 = Nat::s(Nat::s(Nat::Z));
        let n4 = Nat::s(Nat::s(Nat::s(Nat::s(Nat::Z))));
        let proof = ProofCompareNat1::find(&n2, &n4);

        let n3 = Nat::s(Nat::s(Nat::s(Nat::Z)));
        assert_eq!(
            proof,
            Some(ProofCompareNat1::LTrans(
                Box::new(ProofCompareNat1::LSucc),
                Box::new(ProofCompareNat1::LSucc),
                n3
            ))
        );
    }

    #[test]
    fn test_find_4() {
        let n2 = Nat::s(Nat::s(Nat::Z));
        let n5 = Nat::s(Nat::s(Nat::s(Nat::s(Nat::s(Nat::Z)))));

        let proof = ProofCompareNat1::find(&n2, &n5);

        let n4 = Nat::s(Nat::s(Nat::s(Nat::s(Nat::Z))));
        let n3 = Nat::s(Nat::s(Nat::s(Nat::Z)));
        assert_eq!(
            proof,
            Some(ProofCompareNat1::LTrans(
                Box::new(ProofCompareNat1::LTrans(
                    Box::new(ProofCompareNat1::LSucc),
                    Box::new(ProofCompareNat1::LSucc),
                    n3,
                )),
                Box::new(ProofCompareNat1::LSucc),
                n4
            )),
        );
    }
}
