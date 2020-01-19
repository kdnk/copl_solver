use super::super::nat::proofs::*;
use super::super::nat::Nat;

pub enum ProofEvalNatExp {
    EConst,
    EPlus,
    ETimes,
    ProofPlusIs::PZero,
    ProofPlusIs::PSucc(Box<ProofPlusIs>),
    ProofTimeIs::TZero,
    ProofTimeIs::TSucc(Box<ProofTimeIs>, Box<ProofPlusIs>, Nat),
}
