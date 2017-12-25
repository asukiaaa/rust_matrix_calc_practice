extern crate ndarray;
use ndarray::prelude::*;

fn main() {
    let a = Array::<f64, _>::zeros(3);
    println!("{:?}", a);
    let b = Array::range(0., 9., 1.).into_shape((3, 3)).unwrap();
    println!("{:?}", b);
}
