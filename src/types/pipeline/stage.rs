use crate::types::{signatures, Signature, ToneCurve};

mod data;

pub use data::{StageClutData, StageData, StageMatrixData, StageToneCurveData, Tab};

pub const MAX_STAGE_CHANNELS: usize = 128;

pub type StageEvalFn = fn(&[f32], &mut [f32], &Stage);
#[derive(Clone)]
pub struct Stage {
    pub(crate) r#type: Signature,
    pub(crate) implements: Signature,
    pub(crate) input_channels: u32,
    pub(crate) output_channels: u32,
    pub(crate) eval: StageEvalFn,
    pub(crate) data: StageData,
}

impl Stage {
    pub(crate) fn unwrap_matrix(&self) -> &StageMatrixData {
        self.data.as_matrix().unwrap()
    }
    pub(crate) fn unwrap_clut(&self) -> &StageClutData {
        self.data.as_clut().unwrap()
    }
    pub(crate) fn unwrap_tone_curve(&self) -> &StageToneCurveData {
        self.data.as_tone_curve().unwrap()
    }
    pub(crate) fn unwrap_matrix_mut(&mut self) -> &mut StageMatrixData {
        self.data.as_matrix_mut().unwrap()
    }
    pub(crate) fn unwrap_clut_mut(&mut self) -> &mut StageClutData {
        self.data.as_clut_mut().unwrap()
    }
    pub(crate) fn unwrap_tone_curve_mut(&mut self) -> &mut StageToneCurveData {
        self.data.as_tone_curve_mut().unwrap()
    }
    pub fn new(
        r#type: Signature,
        input_channels: u32,
        output_channels: u32,
        eval_ptr: StageEvalFn,
        data: StageData,
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
            signatures::stage::MATRIX_ELEM_TYPE,
            cols,
            rows,
            Self::evaluate_matrix,
            StageData::None,
        );

        let v = vec![0.0; n as usize];

        let mut d = v.into_boxed_slice();
        d[0..n as usize].copy_from_slice(&matrix[0..n as usize]);

        let new_elem = StageData::Matrix(if offset.len() != 0 {
            let v = vec![0.0; rows as usize];
            let mut o = v.into_boxed_slice();

            o[0..rows as usize].copy_from_slice(&offset[0..rows as usize]);

            StageMatrixData {
                double: d,
                offset: o,
            }
        } else {
            StageMatrixData {
                double: d,
                offset: Default::default(),
            }
        });

        new_mpe.data = new_elem;

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
}
