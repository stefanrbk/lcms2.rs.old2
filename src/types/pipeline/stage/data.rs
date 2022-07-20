use crate::{types::ToneCurve, plugins::InterpParams};

#[derive(Clone)]
pub enum StageData{
    ToneCurve(StageToneCurveData),
    Matrix(StageMatrixData),
    CLUT(StageClutData),
    None,
}

impl StageData {
    /// Returns `true` if the stage data is [`ToneCurve`].
    ///
    /// [`ToneCurve`]: StageData::ToneCurve
    #[must_use]
    pub fn is_tone_curve(&self) -> bool {
        matches!(self, Self::ToneCurve(..))
    }

    pub fn as_tone_curve(&self) -> Option<&StageToneCurveData> {
        if let Self::ToneCurve(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn as_tone_curve_mut(&mut self) -> Option<&mut StageToneCurveData> {
        if let Self::ToneCurve(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the stage data is [`Matrix`].
    ///
    /// [`Matrix`]: StageData::Matrix
    #[must_use]
    pub fn is_matrix(&self) -> bool {
        matches!(self, Self::Matrix(..))
    }

    pub fn as_matrix(&self) -> Option<&StageMatrixData> {
        if let Self::Matrix(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn as_matrix_mut(&mut self) -> Option<&mut StageMatrixData> {
        if let Self::Matrix(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the stage data is [`CLUT`].
    ///
    /// [`CLUT`]: StageData::CLUT
    #[must_use]
    pub fn is_clut(&self) -> bool {
        matches!(self, Self::CLUT(..))
    }

    pub fn as_clut(&self) -> Option<&StageClutData> {
        if let Self::CLUT(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn as_clut_mut(&mut self) -> Option<&mut StageClutData> {
        if let Self::CLUT(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the stage data is [`None`].
    ///
    /// [`None`]: StageData::None
    #[must_use]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

pub type StageToneCurveData = Box<[ToneCurve]>;

#[derive(Default, Clone)]
pub struct StageMatrixData {
    pub(crate) double: Box<[f64]>,
    pub(crate) offset: Box<[f64]>,
}
#[derive(Clone)]
pub struct StageClutData {
    pub(crate) tab: Tab,
    pub(crate) params: Box<[InterpParams]>,
}
#[derive(Clone)]
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
