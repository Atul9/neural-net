extern crate byteorder;
extern crate ndarray;
extern crate rand;
extern crate reqwest;
#[macro_use] extern crate simple_error;

use std::error::Error;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Clone)]
pub struct TrainableVariable {
    pub name: String,
    pub value: Rc<Cell<f32>>,
}

pub trait Layer {
    fn init(&self, namespace: &str) -> Box<LayerInstance>;

    fn input_shape(&self) -> ndarray::IxDyn;

    fn output_shape(&self) -> ndarray::IxDyn;
}

pub trait LayerInstance {
    fn eval(&self, input: ndarray::ArrayViewD<f32>) -> ndarray::ArrayD<f32> {
        self.expression(input.mapv(algebra::c).view()).mapv(|e| e.eval())
    }

    fn expression(&self, input: ndarray::ArrayViewD<algebra::Expr>) -> ndarray::ArrayD<algebra::Expr>;

    fn trainable_variables(&self) -> &[TrainableVariable] {
        &[]
    }
}

pub trait Dataset {
    fn len(&self) -> usize;

    fn input(&mut self, i: usize) -> Result<ndarray::ArrayViewD<f32>, Box<Error>>;

    fn target(&mut self, i: usize) -> Result<ndarray::ArrayViewD<f32>, Box<Error>>;
}

pub mod algebra;
pub mod activations;
pub mod datasets;
pub mod initializers;
pub mod layers;
pub mod models;
pub mod util;
