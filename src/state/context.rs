use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::{plugins::Plugin, types::signatures, LCMS_VERSION};

use super::chunks::{
    alarm_codes::AlarmCodesChunk,
    error_handler::{ErrorCode, LogErrorChunk},
};

type Result<T> = std::result::Result<T, String>;

#[derive(Clone, Debug)]
pub struct Context {
    pub(crate) user_data: Option<Box<[u8]>>,
    pub(crate) error_handler: Box<LogErrorChunk>,
    pub(crate) alarm_codes: Box<AlarmCodesChunk>,
}

pub static GLOBAL_CONTEXT: Lazy<Mutex<Context>> = Lazy::new(|| Mutex::new(Context::new(None)));

impl Context {
    pub fn new(data: Option<Box<[u8]>>) -> Self {
        let result = Context {
            user_data: data,
            error_handler: Default::default(),
            alarm_codes: Default::default(),
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
        let eh = self.error_handler.handler;
        eh(self, code, text.into())
    }
}
