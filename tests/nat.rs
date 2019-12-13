use copl_solver::nat::proofs::*;
use copl_solver::nat::*;

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
