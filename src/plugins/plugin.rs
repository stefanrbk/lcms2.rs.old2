use std::sync::Arc;

use crate::types::Signature;

use super::InterpFnFactory;

pub struct Plugin {
    pub magic: Signature,
    pub expected_version: u32,
    pub r#type: Signature,
    pub next: Option<Arc<Plugin>>,
    pub data: PluginType,
}

pub const MAX_TYPES_IN_LCMS_PLUGIN: usize = 20;

pub enum PluginType {
    Interpolation(InterpFnFactory),
    ParametricCurve,
    Formatter,
    TagType,
    Tag,
    RenderingIntent,
    MultiProcessElement,
    Optimization,
    Transform,
}

impl PluginType {
    /// Returns `true` if the plugin type is [`Interpolation`].
    ///
    /// [`Interpolation`]: PluginType::Interpolation
    #[must_use]
    pub fn is_interpolation(&self) -> bool {
        matches!(self, Self::Interpolation(..))
    }

    pub fn as_interpolation(&self) -> Option<&InterpFnFactory> {
        if let Self::Interpolation(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_interpolation(self) -> Result<InterpFnFactory, Self> {
        if let Self::Interpolation(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}
