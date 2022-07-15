use std::any::Any;

use crate::{
    math::{f32_slice_to_u16_slice, u16_slice_to_f32_slice},
    plugins::InterpParams,
};

use super::{signatures::stage, Signature, ToneCurve};

pub const MAX_STAGE_CHANNELS: usize = 128;

pub type StageEvalFn = fn(r#in: &[f32], out: &mut [f32], mpe: &Stage);
pub struct Stage {
    pub(crate) r#type: Signature,
    pub(crate) implements: Signature,
    pub(crate) input_channels: u32,
    pub(crate) output_channels: u32,
    pub(crate) eval: StageEvalFn,
    pub(crate) data: Option<Box<dyn Any>>,
}
pub enum StageLoc {
    AtBegin,
    AtEnd,
}

pub struct StageToneCurveData {
    pub(crate) curves: Vec<ToneCurve>,
}
#[derive(Default)]
pub struct StageMatrixData {
    pub(crate) double: Box<[f64]>,
    pub(crate) offset: Box<[f64]>,
}
pub enum Tab {
    U16(Box<[u16]>),
    F32(Box<[f32]>),
}

impl Tab {
    pub fn as_u16(&self) -> Option<&Box<[u16]>> {
        if let Self::U16(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn as_u16_mut(&mut self) -> Option<&mut Box<[u16]>> {
        if let Self::U16(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_f32(&self) -> Option<&Box<[f32]>> {
        if let Self::F32(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_f32_mut(&mut self) -> Option<&mut Box<[f32]>> {
        if let Self::F32(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the tab is [`U16`].
    ///
    /// [`U16`]: Tab::U16
    #[must_use]
    pub fn is_u16(&self) -> bool {
        matches!(self, Self::U16(..))
    }

    /// Returns `true` if the tab is [`F32`].
    ///
    /// [`F32`]: Tab::F32
    #[must_use]
    pub fn is_f32(&self) -> bool {
        matches!(self, Self::F32(..))
    }
}
pub struct StageClutData {
    pub(crate) tab: Tab,
    pub(crate) params: Box<[InterpParams]>,
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

impl Stage {
    pub fn new(
        r#type: Signature,
        input_channels: u32,
        output_channels: u32,
        eval_ptr: StageEvalFn,
        data: Option<Box<dyn Any>>,
    ) -> Self {
        Self {
            r#type,
            implements: r#type,
            input_channels,
            output_channels,
            eval: eval_ptr,
            data,
        }
    }
    pub fn alloc_tone_curves(_num_channels: u32, _curves: &[ToneCurve]) -> Option<Self> {
        todo!()
    }
    pub fn alloc_clut_16bit(
        _num_grid_points: u32,
        _input_channels: u32,
        _output_channels: u32,
        _table: &mut [u16],
    ) -> Option<Self> {
        todo!()
    }
    pub fn alloc_matrix(rows: u32, cols: u32, matrix: &[f64], offset: &[f64]) -> Option<Self> {
        let n = u32::wrapping_mul(rows, cols);

        // check for overflow
        if n == 0 {
            return None;
        }
        if n >= u32::MAX / cols {
            return None;
        }
        if n >= u32::MAX / rows {
            return None;
        }
        if n < rows || n < cols {
            return None;
        }

        let mut new_mpe = Self::new(
            stage::MATRIX_ELEM_TYPE,
            cols,
            rows,
            Self::evaluate_matrix,
            None,
        );

        let mut new_elem = StageMatrixData::default();
        let v = vec![0.0; n as usize];
        new_elem.double = v.into_boxed_slice();

        new_elem.double[0..n as usize].copy_from_slice(&matrix[0..n as usize]);

        if offset.len() != 0 {
            let v = vec![0.0; rows as usize];
            new_elem.offset = v.into_boxed_slice();
            new_elem.offset[0..rows as usize].copy_from_slice(&offset[0..rows as usize]);
        }

        new_mpe.data = Some(Box::new(new_elem));

        Some(new_mpe)
    }
    fn evaluate_matrix(_in: &[f32], _out: &mut [f32], _mpe: &Stage) {
        todo!()
    }

    pub(crate) fn alloc_clut_16bit_granular(
        clut_points: &[u32],
        input_channels: usize,
        output_channels: usize,
        table: &[u16],
    ) -> Option<Self> {
        todo!()
    }
    pub fn get_curve_set(&self) -> Option<&Vec<ToneCurve>> {
        match &self.data {
            Some(data) => data.as_ref().downcast_ref::<Vec<ToneCurve>>(),
            None => None,
        }
    }
    pub fn get_curve_set_mut(&mut self) -> Option<&mut Vec<ToneCurve>> {
        match &mut self.data {
            Some(data) => data.as_mut().downcast_mut::<Vec<ToneCurve>>(),
            None => None,
        }
    }
}

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
            (mpe.eval)(s, next_s, mpe);
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
            (mpe.eval)(s, next_s, mpe);
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
}
