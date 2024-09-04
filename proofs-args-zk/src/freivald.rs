#![allow(non_snake_case)]

use ark_ff::Field;
use ndarray::{Array1, Array2};

use super::solomon_codes::ReedSolomon;


pub struct Freivald<T> {
    A: Array2<T>,
    B: Array2<T>,
    x: Array1<T>,
    ABx: Array1<T>,
}

impl<T: Field> Freivald<T> {

    pub fn new(A: Array2<T>, B: Array2<T>) -> Self {
        assert!(A.is_square(), "Input must be square matrixes");
        assert!(B.is_square(), "Input must be square matrixes");
        let n = A.nrows();
        assert_eq!(n, B.ncols(), "Input matrixes have to be of the same size");

        // TODO: move x and ABx out of instantiation with multithreading
        let r = ReedSolomon::draw_random(); 
        let mut r_power = r;
        let mut x = vec![T::ONE, r];
        for _ in 0..n-2 {
            r_power *= r;
            x.push(r_power);
        }
        let x = ndarray::arr1(&x);
        let ABx = Freivald::precompute(&A, &B, &x);
        dbg!(&ABx);

        Freivald { A, B, x, ABx }
    }

    fn precompute(A: & Array2<T>, B: &Array2<T> , x: &Array1<T>) -> Array1<T> {
        let Bx = B.dot(x);
        println!("{:?}", &Bx);
        A.dot(&Bx)
    }

    pub fn get_x(&self) -> &Array1<T> {
        &self.x
    }

    pub fn verify(&self, Cx: &Array1<T>) -> bool {
        self.ABx.eq(Cx)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::fields::{Fp64, MontBackend, MontConfig};

    #[derive(MontConfig)]
    #[modulus = "17"]
    #[generator = "3"]
    pub struct FqConfig;
    pub type F = Fp64<MontBackend<FqConfig, 1>>;

    #[test]
    fn test_custom_field() {

        let A = ndarray::arr2(&[[F::from(1), F::from(2)],
                                [F::from(3), F::from(4)]]);
        let B = ndarray::arr2(&[[F::from(1), F::from(2)],
                                [F::from(3), F::from(4)]]);
        let C = A.dot(&B);

        let algo = Freivald::new(A, B);
        let x = algo.get_x();
        let Cx = C.dot(x);

        assert!(algo.verify(&Cx));
    }
}
