use std::sync::Arc;

type Result = std::result::Result<(), ()>;

use crate::{types::Signature, state::Context};

use super::{
    formatter::FormatterFactory, IntentFn, InterpFnFactory, OPToptimizeFn,
    ParametricCurveEvaluator, TagDescriptor, TransformFactories, TypeHandler,
};

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
    TagType {
        handler: TypeHandler,
    },
    Tag {
        signature: Signature,
        descriptor: TagDescriptor,
    },
    RenderingIntent {
        intent: Signature,
        link: IntentFn,
        description: String,
    },
    MultiProcessElement {
        handler: TypeHandler,
    },
    Optimization {
        optimizer: OPToptimizeFn,
    },
    Transform {
        factory: TransformFactories,
    },
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

    /// Returns `true` if the plugin type is [`TagType`].
    ///
    /// [`TagType`]: PluginType::TagType
    #[must_use]
    pub fn is_tag_type(&self) -> bool {
        matches!(self, Self::TagType { .. })
    }

    /// Returns `true` if the plugin type is [`Tag`].
    ///
    /// [`Tag`]: PluginType::Tag
    #[must_use]
    pub fn is_tag(&self) -> bool {
        matches!(self, Self::Tag { .. })
    }

    /// Returns `true` if the plugin type is [`RenderingIntent`].
    ///
    /// [`RenderingIntent`]: PluginType::RenderingIntent
    #[must_use]
    pub fn is_rendering_intent(&self) -> bool {
        matches!(self, Self::RenderingIntent { .. })
    }

    /// Returns `true` if the plugin type is [`MultiProcessElement`].
    ///
    /// [`MultiProcessElement`]: PluginType::MultiProcessElement
    #[must_use]
    pub fn is_multi_process_element(&self) -> bool {
        matches!(self, Self::MultiProcessElement { .. })
    }

    /// Returns `true` if the plugin type is [`Optimization`].
    ///
    /// [`Optimization`]: PluginType::Optimization
    #[must_use]
    pub fn is_optimization(&self) -> bool {
        matches!(self, Self::Optimization { .. })
    }

    /// Returns `true` if the plugin type is [`Transform`].
    ///
    /// [`Transform`]: PluginType::Transform
    #[must_use]
    pub fn is_transform(&self) -> bool {
        matches!(self, Self::Transform { .. })
    }

    pub fn register(self, context: &mut Context) -> Result {
        match self {
            PluginType::TagType { handler } => context.tag_types_plugin.add(handler),
            _ => todo!(),
            // PluginType::Interpolation { interpolation_factory } => todo!(),
            // PluginType::ParametricCurve { num_functions, function_types, parameter_count, evaluator } => todo!(),
            // PluginType::Formatter { formatters_factory } => todo!(),
            // PluginType::Tag { signature, descriptor } => todo!(),
            // PluginType::RenderingIntent { intent, link, description } => todo!(),
            // PluginType::MultiProcessElement { handler } => todo!(),
            // PluginType::Optimization { optimizer } => todo!(),
            // PluginType::Transform { factory } => todo!(),
        }
        Ok(())
    }
}
