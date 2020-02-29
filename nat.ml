type nat =
  | Z
  | S of nat

type proof_plus_is =
  | PZero
  | PSucc of proof_plus_is

type proof_times_is =
  | TZero
  | TSucc of proof_times_is * proof_plus_is * nat

let rec check_plus a b c proof = match proof with
  | PZero -> a = Z && b = c
  | PSucc proof_0 -> begin match a, c with
    | S a', S c' -> check_plus a' b c' proof_0
    | _, _ -> false
  end

let rec check_times a b c proof = match proof with
  | TZero -> a = Z && c = Z
  | TSucc (proof_0, proof_1, nat_0) -> begin match a, b, c with
    | S a', b, c -> check_times a' b nat_0 proof_0 && check_plus b nat_0 c proof_1
    | _, _, _ -> false
  end

let rec find_plus_proof = fun a b c -> match a, c with
  | Z, _ -> if b = c then Some(PZero) else None
  | S a', S c' -> begin match find_plus_proof a' b c' with
    | None -> None
    | Some proof_0 -> Some(PSucc proof_0)
  end
  | _, _ -> None

(* option option int *)
(* string | null | null *)


(* Z plus S(Z) is S(Z) *)
let proof = find_plus_proof Z (S Z) Z;;
(* printf proof *)
