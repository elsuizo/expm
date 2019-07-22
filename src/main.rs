use ndarray::prelude::*;
use ndarray_linalg::types::Scalar;
use ndarray::*;
use ndarray_linalg::*;

use num::Float;
use core::fmt::Debug;

pub fn inf_norm<D: Dimension, T: Float + Debug>(array: &Array<T, D>) -> T {
    let zero = T::zero();
    array.iter().cloned().fold(zero, T::max)
}

fn main() {

    let mut a: Array3<f64> = random((3, 3, 3));
    // let a: Array3<f64> = arr3(&[[1.0, 1.7], [1.0, 3.0], [67.0, 2.8]]);
    let inf_n = inf_norm(&a);
    println!("norm: {:}", inf_n);
    println!("a: {:}", a);
}
