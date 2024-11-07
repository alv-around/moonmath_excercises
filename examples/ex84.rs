use ark_ec::pairing::Pairing;
use ark_std::UniformRand;

use ark_bn254::{Bn254, G1Projective as G1, G2Projective as G2};

fn main() {

    // The pairing engine is parameterized by the scalar field of the curve.
    let mut rng = ark_std::test_rng();
    let a = G1::rand(&mut rng);
    let b = G2::rand(&mut rng);

    // We can compute the pairing of two points on the curve, either monolithically...
    let e1 = Bn254::pairing(a, b);

    // ... or in two steps. First, we compute the Miller loop...
    let ml_result = Bn254::miller_loop(a, b);
    // ... and then the final exponentiation.
    let e2 = Bn254::final_exponentiation(ml_result).unwrap();
    assert_eq!(e1, e2);

}
