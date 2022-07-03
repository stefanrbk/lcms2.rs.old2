use std::{fmt::Debug, sync::Mutex};

use once_cell::sync::Lazy;

use crate::{
    plugins::{
        FormatterFactoryList, IntentsList, InterpFnFactory, OptimizationCollection,
        ParametricCurvesCollection, Plugin, TagList, TagTypeList, TransformCollection,
    },
    types::{signatures, MAX_CHANNELS},
    LCMS_VERSION,
};

use super::{
    default_interpolatior_factory, default_log_error_handler_function, ErrorCode,
    LogErrorHandlerFunction,
};

type Result<T> = std::result::Result<T, String>;

#[derive(Clone)]
pub struct Context {
    pub(crate) user_data: Option<Box<[u8]>>,
    pub(crate) error_handler: Box<LogErrorHandlerFunction>,
    pub(crate) alarm_codes: Box<[u16; MAX_CHANNELS]>,
    pub(crate) adaption_state: Box<f64>,
    pub(crate) interpolation_plugin: Box<InterpFnFactory>,
    pub(crate) curves_plugin: Box<ParametricCurvesCollection>,
    pub(crate) formatters_plugin: Box<FormatterFactoryList>,
    pub(crate) tag_types_plugin: Box<TagTypeList>,
    pub(crate) tags_plugin: Box<TagList>,
    pub(crate) intents_plugin: Box<IntentsList>,
    pub(crate) mpe_types_plugin: Box<TagTypeList>,
    pub(crate) optimization_plugin: Box<OptimizationCollection>,
    pub(crate) transform_plugin: Box<TransformCollection>,
}

pub static GLOBAL_CONTEXT: Lazy<Mutex<Context>> = Lazy::new(|| Mutex::new(Context::new(None)));

impl Context {
    pub fn new(data: Option<Box<[u8]>>) -> Self {
        let result = Context {
            user_data: data,
            error_handler: Box::new(default_log_error_handler_function),
            alarm_codes: Box::new([
                0x7F00, 0x7F00, 0x7F00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]),
            adaption_state: Box::new(1.0),
            interpolation_plugin: Box::new(default_interpolatior_factory),
            curves_plugin: Default::default(),
            formatters_plugin: Default::default(),
            tag_types_plugin: Default::default(),
            tags_plugin: Default::default(),
            intents_plugin: Default::default(),
            mpe_types_plugin: Default::default(),
            optimization_plugin: Default::default(),
            transform_plugin: Default::default(),
        };

        result
    }

    pub fn init_plugin(&mut self, plugin: &Plugin) -> Result<()> {
        let signal_error = |ctx: &mut Self, code: ErrorCode, text: String| -> Result<()> {
            ctx.signal_error(code, text.clone());
            Err(text)
        };

        let mut plugin = plugin;
        while plugin.next.is_some() {
            if plugin.magic != signatures::plugin_type::MAGIC {
                return signal_error(
                    self,
                    ErrorCode::UnknownExtension,
                    "Unrecognized plugin".to_string(),
                );
            }
            if plugin.expected_version > LCMS_VERSION {
                return signal_error(
                    self,
                    ErrorCode::UnknownExtension,
                    format!(
                        "plugin needs Little CMS {}, current version is {}",
                        plugin.expected_version, LCMS_VERSION
                    ),
                );
            }

            match plugin.r#type {
                signatures::plugin_type::INTERPOLATION => (),
                signatures::plugin_type::TAG_TYPE => (),
                signatures::plugin_type::TAG => (),
                signatures::plugin_type::FORMATTERS => (),
                signatures::plugin_type::RENDERING_INTENT => (),
                signatures::plugin_type::PARAMETRIC_CURVE => (),
                signatures::plugin_type::MULTI_PROCESS_ELEMENT => (),
                signatures::plugin_type::OPTIMIZATION => (),
                signatures::plugin_type::TRANSFORM => (),
                _ => {
                    return signal_error(
                        self,
                        ErrorCode::UnknownExtension,
                        format!("Unrecognized plugin type '{:?}'", plugin.r#type),
                    )
                }
            };

            plugin = plugin.next.as_ref().unwrap();
        }

        Ok(())
    }

    pub fn get_user_data(&self) -> Option<&Box<[u8]>> {
        self.user_data.as_ref()
    }

    pub fn signal_error(&mut self, code: ErrorCode, text: impl Into<String>) {
        let eh = (*self.error_handler).clone();
        eh(self, code, text.into())
    }
}

impl Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("user_data", &self.user_data)
            .field("error_handler", &"[Function Ptr]")
            .field("alarm_codes", &self.alarm_codes)
            .field("adaption_state", &self.adaption_state)
            .field("interpolation_plugin", &self.interpolation_plugin)
            .field("curves_plugin", &self.curves_plugin)
            .field("formatters_plugin", &self.formatters_plugin)
            .field("tag_types_plugin", &self.tag_types_plugin)
            .field("tags_plugin", &self.tags_plugin)
            .field("intents_plugin", &self.intents_plugin)
            .field("mpe_types_plugin", &self.mpe_types_plugin)
            .field("optimization_plugin", &self.optimization_plugin)
            .field("transform_plugin", &self.transform_plugin)
            .finish()
    }
}
