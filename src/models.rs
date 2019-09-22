use std::error::Error;

use super::{Layer, Shape};

pub struct Sequential {
    output_shape: Shape,
}

impl Sequential {
    pub fn new(input_shape: Shape) -> Sequential {
        Sequential{
            output_shape: input_shape,
        }
    }

    pub fn output_shape(&self) -> Shape {
        self.output_shape
    }

    pub fn add_layer<T>(&mut self, layer: T) -> Result<(), Box<Error>>
        where T: Layer
    {
        if self.output_shape != layer.input_shape() {
            bail!("invalid input shape for layer. got {}, expected {}", layer.input_shape(), self.output_shape)
        }

        // TODO
        self.output_shape = layer.output_shape();
        Ok(())
    }

    pub fn compile(self) -> CompiledSequential {
        // TODO
        CompiledSequential{}
    }
}

pub struct CompiledSequential {}

impl CompiledSequential {
    pub fn fit(&mut self) {
        // TODO
    }
}
