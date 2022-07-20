use std::any::Any;

use crate::math::{f32_slice_to_u16_slice, u16_slice_to_f32_slice};

use self::stage::{Stage, MAX_STAGE_CHANNELS};

use super::Signature;

pub enum StageLoc {
    AtBegin,
    AtEnd,
}

pub enum PipelineEvalFn {
    U16(fn(r#in: &[u16], out: &mut [u16], data: &Box<dyn Any>)),
    Float(fn(r#in: &[f32], out: &mut [f32], data: &Box<dyn Any>)),
}
pub struct Pipeline {
    pub(crate) elements: Vec<Stage>,
    pub(crate) input_channels: u32,
    pub(crate) output_channels: u32,
    pub(crate) data: Option<Box<dyn Any>>,
    pub(crate) eval: PipelineEvalFn,
    pub(crate) save_as_8_bits: bool,
}
// &mut Context must be passed in for all functions involving Pipeline

impl Pipeline {
    pub fn new(input_channels: u32, output_channels: u32) -> Self {
        let mut result = Self {
            elements: Vec::new(),
            input_channels,
            output_channels,
            data: None,
            eval: PipelineEvalFn::Float(Self::lut_eval_float),
            save_as_8_bits: false,
        };
        result.bless_lut();

        result
    }
    pub fn bless_lut(&mut self) {
        if self.elements.is_empty() {
            return;
        }

        let first = self.elements.first().unwrap();
        let last = self.elements.last().unwrap();

        self.input_channels = first.input_channels;
        self.output_channels = last.output_channels;
    }
    pub fn lut_eval_16(r#in: &[u16], out: &mut [u16], data: &Box<dyn Any>) {
        let lut = data.downcast_ref::<Self>().unwrap();
        let mut storage = [[0f32; MAX_STAGE_CHANNELS]; 2];
        let mut phase = 0usize;

        u16_slice_to_f32_slice(r#in, &mut storage[phase][0..r#in.len()]);

        for mpe in lut.elements.iter() {
            let next_phase = phase ^ 1;
            let (s, next_s) = if next_phase == 0 {
                let (second, first) = storage.split_at_mut(1);
                (
                    &first[0][0..MAX_STAGE_CHANNELS],
                    &mut second[0][0..MAX_STAGE_CHANNELS],
                )
            } else {
                let (first, second) = storage.split_at_mut(1);
                (
                    &first[0][0..MAX_STAGE_CHANNELS],
                    &mut second[0][0..MAX_STAGE_CHANNELS],
                )
            };
            //(mpe.eval)(s, next_s, mpe);
            phase = next_phase;
        }

        f32_slice_to_u16_slice(
            &storage[phase][0..out.len()],
            &mut out[0..lut.output_channels as usize],
        );
    }
    pub fn lut_eval_float(r#in: &[f32], out: &mut [f32], data: &Box<dyn Any>) {
        let lut = data.downcast_ref::<Self>().unwrap();
        let mut storage = [[0f32; MAX_STAGE_CHANNELS]; 2];
        let mut phase = 0usize;

        storage[phase][0..r#in.len()].copy_from_slice(r#in);

        for mpe in lut.elements.iter() {
            let next_phase = phase ^ 1;
            let (s, next_s) = if next_phase == 0 {
                let (second, first) = storage.split_at_mut(1);
                (
                    &first[0][0..MAX_STAGE_CHANNELS],
                    &mut second[0][0..MAX_STAGE_CHANNELS],
                )
            } else {
                let (first, second) = storage.split_at_mut(1);
                (
                    &first[0][0..MAX_STAGE_CHANNELS],
                    &mut second[0][0..MAX_STAGE_CHANNELS],
                )
            };
            //(mpe.eval)(s, next_s, mpe);
            phase = next_phase;
        }

        let len = out.len();
        out[0..lut.output_channels as usize].copy_from_slice(&storage[phase][0..len]);
    }
    pub fn insert_stage(&mut self, loc: StageLoc, mpe: Stage) {
        match loc {
            StageLoc::AtBegin => self.elements.insert(0, mpe),
            StageLoc::AtEnd => self.elements.push(mpe),
        }

        self.bless_lut();
    }

    pub(crate) fn check_and_retrieve_stages(
        &self,
        n: i32,
        args: &[(Signature, &mut Option<Stage>)],
    ) -> Option<()> {
        todo!()
    }
}

pub mod stage;
