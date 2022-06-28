pub const MAX_INPUT_DIMENTIONS: usize = 15;

pub enum InterpFunction {
    InterpFn16(fn(input: &[u16], output: &mut [u16], p: InterpParams)),
    InterpFnFloat(fn(input: &[f32], output: &mut [f32], p: InterpParams)),
}

pub type InterpFnFactory =
    fn(input_channels: u32, output_channels: u32, flags: u32) -> InterpFunction;

pub struct InterpParams {
    flags: u32,
    inputs: u32,
    outputs: u32,
    samples: [u32; MAX_INPUT_DIMENTIONS],
    domain: [u32; MAX_INPUT_DIMENTIONS],
    optimization: [u32; MAX_INPUT_DIMENTIONS],
    table: Box<[u8]>,
    interpolation: InterpFunction,
}
