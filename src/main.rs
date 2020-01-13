extern crate tensorflow;

use tensorflow::Tensor;

fn main() {
    let values = vec![
        0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        17.0,
    ];
    let tensor_train: Tensor<f32> = <Tensor<f32>>::new(&[1, 3, 3, 2])
        .with_values(&values)
        .unwrap();

    println!("{:#?}", tensor_train);
}
