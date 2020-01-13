# Tensorflow Tensors in Rust and Python

This repo is an example of how to create a Tensor and populate it with data. There is a comparable `numpy` array and implementation of the array in Python

### The Rust Code

The full code in src/main.rs is below. This shows how to init a Tensorflow Tensor then pass it a flattened Vector of the data.

When we make the Tensor we need to specify the correct size and type. In this case we use floats, and a 1,3,3,2 array.

```rust
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
```

### The Numpy array

We can make basicly the same array in Python. This is not a Tensorflow Tensor but in Python you can pass a numpy array to the Tensorflow API's and it will automatically convert this to the Tensor you expect. Here we can use the Python example to inform how to manually build the Tensor in Rust.

```python
import numpy as np


arr = np.array([[
    [
        [0.0, 1.0], [2.0, 3.0], [4.0, 5.0]
    ],
    [
        [6.0, 7.0], [8.0, 9.0], [10.0, 11.0]
    ],
    [
        [12.0, 13.0], [14.0, 15.0], [16.0, 17.0]
    ],
]])

# array([[[[  0.,   1.],
#          [  2.,   3.],
#          [  4.,   5.]], 

#         [[  6.,   7.],
#          [  8.,   9.],
#          [ 10.,  11.]],

#         [[ 12.,  13.],
#          [ 14.,  15.],
#          [ 16.,  17.]]]])
```


Hope this is helpful for those who want to use Tensorflow in Rust and are coming from Tensorflow in Python. The src folder container code that can be run and compiled with the following commands

```bash
cargo run --release

# Tensor {
#     inner: TensorDataCRepr {
#         inner: 0x00007fa1f4e5aa60,
#         data_count: 18,
#         phantom: PhantomData,
#     },
#     dims: [
#         1,
#         3,
#         3,
#         2,
#     ],
# }
```