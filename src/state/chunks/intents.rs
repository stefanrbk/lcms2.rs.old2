use crate::plugins::IntentsList;

#[derive(Clone, Debug, Default)]
pub struct IntentsPluginChunk {
    pub intents: IntentsList,
}
