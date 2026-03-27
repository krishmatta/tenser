use crate::dimension::Dimension;
use crate::shape::Shape;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

pub trait TensorType: Copy + Clone + Default + Debug + Display
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + PartialOrd
    + PartialEq
{
}

impl TensorType for f32 {}
impl TensorType for f64 {}

pub struct Tensor<T: TensorType, S: Shape> {
    shape: S,
    data: Vec<T>,
    stride: Vec<usize>,
}

pub struct TensorView<'a, T: TensorType, S: Shape> {
    shape: S,
    data: &'a [T],
    stride: Vec<usize>,
}

impl<T: TensorType, S: Shape> Tensor<T, S> {
    fn new(shape: S) -> Self {
        let stride = shape.default_stride();
        let size = shape.size();

        Tensor {
            shape,
            data: vec![T::default(); size],
            stride,
        }
    }

    fn view(&self) -> TensorView<'_, T, S> {
        TensorView {
            shape: self.shape,
            data: &self.data,
            stride: self.stride.clone(),
        }
    }
}

impl<'a, T: TensorType, D1: Dimension> TensorView<'a, T, (D1,)> {
    fn slice(&self, index: usize) -> T {
        self.data[self.stride[0] * index]
    }
}

impl<T: TensorType, D1: Dimension> Tensor<T, (D1,)> {
    fn slice(&self, index: usize) -> T {
        self.view().slice(index)
    }
}

impl<'a, T: TensorType, D1: Dimension, D2: Dimension> TensorView<'a, T, (D1, D2)> {
    fn slice(&self, index: usize) -> TensorView<'a, T, (D2,)> {
        let start = index * self.stride[0];
        let end = start + (self.shape.1.get_value() - 1) * self.stride[1] + 1;

        TensorView {
            shape: (self.shape.1,),
            data: &self.data[start..end],
            stride: vec![self.stride[1]],
        }
    }
}

impl<T: TensorType, D1: Dimension, D2: Dimension> Tensor<T, (D1, D2)> {
    fn slice(&self, index: usize) -> TensorView<'_, T, (D2,)> {
        self.view().slice(index)
    }
}

impl<'a, T: TensorType, D1: Dimension, D2: Dimension, D3: Dimension> TensorView<'a, T, (D1, D2, D3)> {
    fn slice(&self, index: usize) -> TensorView<'a, T, (D2, D3)> {
        let start = index * self.stride[0];
        let end = start + (self.shape.1.get_value() - 1) * self.stride[1] + (self.shape.2.get_value() - 1) * self.stride[2] + 1;

        TensorView {
            shape: (self.shape.1, self.shape.2),
            data: &self.data[start..end],
            stride: vec![self.stride[1], self.stride[2]],
        }
    }
}

impl<T: TensorType, D1: Dimension, D2: Dimension, D3: Dimension> Tensor<T, (D1, D2, D3)> {
    fn slice(&self, index: usize) -> TensorView<'_, T, (D2, D3)> {
        self.view().slice(index)
    }
}

impl<'a, T: TensorType, D1: Dimension, D2: Dimension, D3: Dimension, D4: Dimension> TensorView<'a, T, (D1, D2, D3, D4)> {
    fn slice(&self, index: usize) -> TensorView<'a, T, (D2, D3, D4)> {
        let start = index * self.stride[0];
        let end = start + (self.shape.1.get_value() - 1) * self.stride[1] + (self.shape.2.get_value() - 1) * self.stride[2] + (self.shape.3.get_value() - 1) * self.stride[3] + 1;

        TensorView {
            shape: (self.shape.1, self.shape.2, self.shape.3),
            data: &self.data[start..end],
            stride: vec![self.stride[1], self.stride[2], self.stride[3]],
        }
    }
}

impl<T: TensorType, D1: Dimension, D2: Dimension, D3: Dimension, D4: Dimension> Tensor<T, (D1, D2, D3, D4)> {
    fn slice(&self, index: usize) -> TensorView<'_, T, (D2, D3, D4)> {
        self.view().slice(index)
    }
}
