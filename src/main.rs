//-------------------------------------------------------------------------
//                        imports
//-------------------------------------------------------------------------
use ndarray::prelude::*;
use ndarray_linalg::types::Scalar;
use ndarray::*;
use ndarray_linalg::*;
use ndarray_linalg::{OperationNorm};

use num::{Float, NumCast};
use core::fmt::Debug;
use std::ops::AddAssign;
// use std::cmp::Ord;
use std::cmp;
use std::f64::{consts, EPSILON};

//-------------------------------------------------------------------------
//                        code
//-------------------------------------------------------------------------
// NOTE(elsuizo:2019-07-23): esta funcion serviria para cualquier dimension del array!!!
pub fn inf_norm<D: Dimension, T: Float + Debug>(array: &Array<T, D>) -> T {
    let zero = T::zero();
    array.iter().cloned().fold(zero, T::max)
}

// NOTE(elsuizo:2019-07-23): no hace falta ya que esta implementado
pub fn opnorm1<T: Float + AddAssign>(array: &Array2<T>) -> T {
    let shape = array.shape();
    let m = shape[0];
    let n = shape[1];
    let mut nrm: T = T::zero();
    for j in 0..n {
        let mut nrmj = T::zero();
        for i in 0..m {
            nrmj += array[[i, j]].abs();
        }
        nrm = T::max(nrm, nrmj);
    }
    return nrm;
}

pub fn is_square(array: &Array2<f64>) -> bool {
    let shape = array.shape();
    if shape[0] == shape[1] {
        return true;
    } else {
        return false;
    }
}

pub fn compare_floats(num1: f64, num2: f64) -> bool {
    f64::abs(num1 - num2) <= f64::epsilon()
}


pub fn expm(array: &Array2<f64>) -> () {
    if is_square(array) {
        // NOTE(elsuizo:2019-07-23): aca iria gebal que es para balancear la matriz y asi sea mas
        // exacto el resultado
        // TODO(elsuizo:2019-07-23): esto no se ve bien asi
        let n = array.shape()[0];

        let norm_array = OperationNorm::opnorm_one(array).unwrap();
        let I: Array2<f64> = identity(n);
        let mut C = Array1::<f64>::zeros(10);
        if compare_floats(norm_array, 2.1) {
            if compare_floats(norm_array, 0.95) {
                C = arr1(&[17643225600.0, 8821612800.0, 2075673600.0, 302702400.0, 30270240.0, 2162160.0, 110880.0, 3960.0, 90.0, 1.0]);
            } else {
                if compare_floats(norm_array, 0.25) {
                    C = arr1(&[17297280.0, 8648640.0, 1995840.0, 277200., 25200.0, 1512.0, 56.0, 1.0]);
                } else {
                    if compare_floats(norm_array, 0.015) {
                        C = arr1(&[30240.0, 15120.0, 3360.0, 420.0, 30.0, 1.0]);
                    } else {
                        C = arr1(&[120.0, 60.0, 12.0, 1.0]);
                    }
                }
            }
        }
        let array_square = array.dot(array);
        let mut P = I.clone();
        let mut U = C[1] * &P;
        let mut V = C[0] * &P;

        for k in 0..(C.len() / 2) - 1 {
            let k2 = 2 * k;
            P = P.dot(&array_square);
            U = U + (C[k2 + 2] * &P);
            V = V + (C[k2 + 1] * &P);
            println!("U{:}", U);
        }
        U = array.dot(&U);
        let X = V + U;
        let solution = (V - U).solve_into(X).unwrap();
    }
}

pub fn identity(n: usize) -> Array2<f64> {
    let mut result = Array2::<f64>::zeros((n, n));
    for i in 0..n {
        for j in 0..n {
            if i == j {
                result[[i, j]] = 1.0;
            }
        }
    }

    return result;
}

fn main() {

    // NOTE(elsuizo:2019-07-23): para estos dos arrays el resultado es el mismo que en Julia
    let mut a: Array2<f64> = arr2(&[[1.0, 2.0, 3.0],
                                    [4.0, 5.0, 6.0],
                                    [7.0, 8.0, 9.0]]);

    let mut b: Array2<f64> = arr2(&[[-1.0, 1.0, 1.0],
                                    [1.0, 1.0, 1.0],
                                    [1.0, 1.0, 1.0]]);
    let inf_n = inf_norm(&a);
    let norm = opnorm1(&b);
    let n = OperationNorm::opnorm_one(&a);
    let v = arr1(&[1.0, 1.0, 1.0]);
    // NOTE(elsuizo:2019-07-23): Vec of Nones
    let a_clone = a.clone();
    println!("norm: {:?}", n);
    println!("len of v: {:}", v.len());
    expm(&a);
}
