use crate::plugins::ParametricCurvesCollection;

#[derive(Clone, Debug, Default)]
pub struct CurvesPluginChunk {
    pub parametric_curves: ParametricCurvesCollection,
}
