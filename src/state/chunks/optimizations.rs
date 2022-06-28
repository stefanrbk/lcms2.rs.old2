use crate::plugins::OptimizationCollection;

#[derive(Clone, Debug, Default)]
pub struct OptimizationPluginChunk {
    pub optimization_collection: OptimizationCollection,
}
