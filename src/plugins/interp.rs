use std::any::Any;

use crate::{state::{Context, ErrorCode}, SafeClonableAny};

pub const MAX_INPUT_DIMENTIONS: usize = 15;

#[derive(Clone)]
pub enum InterpFunction {
    InterpFn16(fn(input: &[u16], output: &mut [u16], p: &InterpParams) -> Option<Box<dyn Any>>),
    InterpFnFloat(fn(input: &[f32], output: &mut [f32], p: &InterpParams) -> Option<Box<dyn Any>>),
}
impl Default for InterpFunction {
    fn default() -> Self {
        Self::InterpFn16(|_, _, _| { None })
    }
}

pub type InterpFnFactory =
    fn(input_channels: u32, output_channels: u32, flags: u32) -> Option<InterpFunction>;

#[derive(Clone, Default)]
pub struct InterpParams {
    pub(crate) flags: u32,
    pub(crate) inputs: u32,
    pub(crate) outputs: u32,
    pub(crate) num_samples: [u32; MAX_INPUT_DIMENTIONS],
    pub(crate) domain: [u32; MAX_INPUT_DIMENTIONS],
    pub(crate) optimization: [u32; MAX_INPUT_DIMENTIONS],
    pub(crate) table: Option<Box<dyn SafeClonableAny>>,
    pub(crate) interpolation: InterpFunction,
}

impl InterpParams {
    pub(crate) fn compute(
        context: &mut Context,
        num_samples: u32,
        input_chan: u32,
        output_chan: u32,
        table: &dyn SafeClonableAny,
        flags: u32,
    ) -> Option<Self> {
        let samples = [num_samples; MAX_INPUT_DIMENTIONS];

        Self::compute_ex(context, &samples, input_chan, output_chan, table, flags)
    }
    pub(crate) fn compute_ex(
        context: &mut Context,
        num_samples: &[u32],
        input_chan: u32,
        output_chan: u32,
        table: &dyn SafeClonableAny,
        flags: u32,
    ) -> Option<Self> {
        let input_chan = input_chan as usize;

        // Check for maximum inputs
        if input_chan > MAX_INPUT_DIMENTIONS {
            let error = format!("Too many input channels ({} channels, max={}", input_chan, MAX_INPUT_DIMENTIONS);
            context.signal_error(ErrorCode::Range, error);
            return None;
        }

        let mut result_samples = [0u32; MAX_INPUT_DIMENTIONS];
        let mut result_domain = [0u32; MAX_INPUT_DIMENTIONS];
        let mut result_opta = [0u32; MAX_INPUT_DIMENTIONS];

        for i in 0..input_chan {
            result_samples[i] = num_samples[i];
            result_domain[i] = num_samples[i] - 1;
        }

        // Compute factors to apply to each component to index the grid array
        result_opta[0] = output_chan;
        for i in 1..input_chan {
            result_opta[i] = result_opta[i-1] * num_samples[input_chan - i];
        }

        

        None
    }
    pub(crate) fn set_interpolation_routine(&mut self, context: &mut Context) -> Option<()> {
        // Invoke factory, possibly in the plug-in
        if let Some(interp) = (context.interpolation_plugin)(self.inputs, self.outputs, self.flags) {
            self.interpolation = interp;
            return Some(());
        }

        // If unsupported by the plug-in, go for the default
        if let Some(interp) = Self::default_interpolatior_factory(self.inputs, self.outputs, self.flags) {
            self.interpolation = interp;
            return Some(());
        }

        None
    }

    pub fn default_interpolatior_factory(_num_input_channels: u32, _num_output_channels: u32, _flags: u32) -> Option<InterpFunction> {
        None
    }

    
}
