use crate::plugins::TagTypeList;

#[derive(Clone, Debug, Default)]
pub struct TagTypePluginChunk {
    pub tag_types: TagTypeList,
}
