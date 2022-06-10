use std::sync::Arc;

use crate::types::Signature;

pub struct Plugin {
    pub magic: Signature,
    pub expected_version: u32,
    pub r#type: Signature,
    pub next: Option<Arc<Plugin>>,
    pub data: PluginType,
}

pub const MAX_TYPES_IN_LCMS_PLUGIN: usize = 20;

pub enum PluginType {
    Interpolation,
    ParametricCurve,
    Formatter,
    TagType,
    Tag,
    RenderingIntent,
    MultiProcessElement,
    Optimization,
    Transform,
}
