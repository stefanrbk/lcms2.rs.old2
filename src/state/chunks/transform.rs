use crate::plugins::TransformCollection;

#[derive(Clone, Debug, Default)]
pub struct TransformPluginChunk {
    pub transform_collection: TransformCollection,
}
