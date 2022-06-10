use std::sync::Arc;

use crate::types::Signature;

use super::{InterpFnFactory, ParametricCurveEvaluator, formatter::FormatterFactory};

pub struct Plugin {
    pub magic: Signature,
    pub expected_version: u32,
    pub r#type: Signature,
    pub next: Option<Arc<Plugin>>,
    pub data: PluginType,
}

pub const MAX_TYPES_IN_LCMS_PLUGIN: usize = 20;

pub enum PluginType {
    Interpolation {
        interpolation_factory: InterpFnFactory,
    },
    ParametricCurve {
        num_functions: u32,
        function_types: [u32; MAX_TYPES_IN_LCMS_PLUGIN],
        parameter_count: [u32; MAX_TYPES_IN_LCMS_PLUGIN],
        evaluator: ParametricCurveEvaluator,
    },
    Formatter {
        formatters_factory: FormatterFactory,
    },
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
        matches!(self, Self::Interpolation { .. })
    }

    /// Returns `true` if the plugin type is [`ParametricCurve`].
    ///
    /// [`ParametricCurve`]: PluginType::ParametricCurve
    #[must_use]
    pub fn is_parametric_curves(&self) -> bool {
        matches!(self, Self::ParametricCurve { .. })
    }

    /// Returns `true` if the plugin type is [`Formatter`].
    ///
    /// [`Formatter`]: PluginType::Formatter
    #[must_use]
    pub fn is_formatter(&self) -> bool {
        matches!(self, Self::Formatter { .. })
    }
}
