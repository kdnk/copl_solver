pub mod proofs;

#[derive(PartialEq, Debug, Clone)]
pub enum Nat {
    Z,
    S(Box<Nat>),
}

impl Nat {
    pub fn s(nat: Self) -> Self {
        Nat::S(Box::new(nat))
    }
}

#[cfg(test)]
mod tests {
    use super::proofs::{ProofPlusIs, ProofTimeIs};
    use super::*;

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

    #[test]
    fn t_succ() {
        let n2_1 = Nat::s(Nat::s(Nat::Z));
        let n1 = Nat::s(Nat::Z);
        let n2_2 = Nat::s(Nat::s(Nat::Z));
        let proof = ProofTimeIs::find(&n2_1, &n1, &n2_2);

        assert_eq!(
            proof,
            Some(ProofTimeIs::TSucc(
                Box::new(ProofTimeIs::TSucc(
                    Box::new(ProofTimeIs::TZero),
                    Box::new(ProofPlusIs::PSucc(Box::new(ProofPlusIs::PZero))),
                    Nat::Z,
                )),
                Box::new(ProofPlusIs::PSucc(Box::new(ProofPlusIs::PZero))),
                Nat::s(Nat::Z)
            ))
        );

        let checked = proof.unwrap().check(&n2_1, &n1, &n2_2);
        assert_eq!(checked, true);
    }

    #[test]
    fn t_zero() {
        let n1 = Nat::Z;
        let n2 = Nat::S(Box::new(Nat::S(Box::new(Nat::Z))));
        let n3 = Nat::Z;
        let proof = ProofTimeIs::find(&n1, &n2, &n3);

        assert_eq!(proof, Some(ProofTimeIs::TZero));

        let checked = proof.unwrap().check(&n1, &n2, &n3);
        assert_eq!(checked, true);
    }
}
