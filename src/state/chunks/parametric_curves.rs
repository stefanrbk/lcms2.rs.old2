use crate::plugins::ParametricCurvesCollection;

#[derive(Clone, Debug)]
pub struct CurvesPluginChunk {
    pub parametric_curves: ParametricCurvesCollection,
}
