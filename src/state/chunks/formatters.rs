use crate::plugins::FormatterFactoryList;

#[derive(Clone, Debug)]
pub struct FormattersPluginChunk {
    pub factory_list: FormatterFactoryList,
}

impl FormattersPluginChunk {
    pub fn new(factory_list: FormatterFactoryList) -> Self {
        Self { factory_list }
    }
}

impl Default for FormattersPluginChunk {
    fn default() -> Self {
        Self { factory_list: Vec::new() }
    }
}
