use ndarray::prelude::*;
use ndarray_linalg::types::Scalar;
use ndarray::*;
use ndarray_linalg::*;

use num::Float;
use core::fmt::Debug;
use std::ops::AddAssign;
// use std::cmp::Ord;
use std::cmp;

pub fn inf_norm<D: Dimension, T: Float + Debug>(array: &Array<T, D>) -> T {
    let zero = T::zero();
    array.iter().cloned().fold(zero, T::max)
}

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

fn main() {

    let mut a: Array2<f32> = arr2(&[[1.0, 2.0, 3.0],
                                    [4.0, 5.0, 6.0],
                                    [7.0, 8.0, 9.0]]);
    // let a: Array3<f64> = arr3(&[[1.0, 1.7], [1.0, 3.0], [67.0, 2.8]]);
    let inf_n = inf_norm(&a);
    // println!("norm: {:}", inf_n);
    // println!("a: {:}", a);
    let norm = opnorm1(&a);
    println!("norm: {:}", norm);
}
