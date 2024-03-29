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

pub fn is_square<T: Float>(array: &Array2<T>) -> bool {
    let shape = array.shape();
    if shape[0] == shape[1] {
        return true;
    } else {
        return false;
    }
}

pub fn compare_floats<T: Float>(num1: T, num2: T) -> bool {
    Float::abs(num1 - num2) <= Float::epsilon()
}

pub fn expm<T: Float + PadeAprox + Scalar>(array: &Array2<T>) -> () {
    if is_square(array) {
        // NOTE(elsuizo:2019-07-23): aca iria gebal que es para balancear la matriz y asi sea mas
        // exacto el resultado
        // TODO(elsuizo:2019-07-23): esto no se ve bien asi
        let n = array.shape()[0];

        let norm_array = OperationNorm::opnorm_one(array).unwrap();
        let I: Array2<T> = identity(n);
        let value1 = NumCast::from(2.1).unwrap();
        let value2 = NumCast::from(0.95).unwrap();
        let value3 = NumCast::from(0.25).unwrap();
        let value4 = NumCast::from(0.015).unwrap();
        let mut C = Array1::<T>::zeros(10);
        if compare_floats(norm_array, value1) {
            if compare_floats(norm_array, value2) {
                C = T::get_vector1();
            } else {
                if compare_floats(norm_array, value3) {
                    C = T::get_vector2();
                } else {
                    if compare_floats(norm_array, value4) {
                        C = T::get_vector3();
                    } else {
                        C = T::get_vector4();
                    }
                }
            }
        }
        let array_square = array.dot(array);
        let P = I.clone();
    }
}

pub fn identity<T: Float>(n: usize) -> Array2<T> {
    let one = T::one();
    let mut result = Array2::<T>::zeros((n, n));
    for i in 0..n {
        for j in 0..n {
            if i == j {
                result[[i, j]] = one;
            }
        }
    }

    return result;
}

fn main() {

    // NOTE(elsuizo:2019-07-23): para estos dos arrays el resultado es el mismo que en Julia
    let mut a: Array2<f32> = arr2(&[[1.0, 2.0, 3.0],
                                    [4.0, 5.0, 6.0],
                                    [7.0, 8.0, 9.0]]);

    let mut b: Array2<f32> = arr2(&[[-1.0, 1.0, 1.0],
                                    [1.0, 1.0, 1.0],
                                    [1.0, 1.0, 1.0]]);
    let inf_n = inf_norm(&a);
    let norm = opnorm1(&b);
    let v = vec![1, 1, 1];
    let n = OperationNorm::opnorm_one(&a);
    // NOTE(elsuizo:2019-07-23): Vec of Nones
    let a_clone = a.clone();
    println!("norm: {:?}", a_clone);
    // expm(&a);
}
