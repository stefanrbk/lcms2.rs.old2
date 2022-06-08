use crate::{plugins::Plugin, types::signatures, LCMS_VERSION};

use super::error_handler::{ErrorCode, LogErrorHandler, default_log_error_handler_function, LogErrorHandlerFunction};

type Result<T> = std::result::Result<T, String>;

#[derive(Clone, Debug)]
pub struct Context {
    user_data: Option<Box<[u8]>>,
    error_handler: Box<LogErrorHandler>,
}

impl Context {
    pub(crate) fn create_global() -> Self {
        Context { user_data: None, error_handler: alloc_log_error_handler(None) }
    }
    pub fn new(data: Option<Box<[u8]>>) -> Self {
        let mut result = Context {
            user_data: match data {
                Some(data) => Some(data),
                None => None,
            },
            error_handler: alloc_log_error_handler(None),
        };

        result
    }

    pub fn init_plugins(&mut self, plugin: &Plugin) -> Result<()> {
        let signal_error = |code: ErrorCode, text: String| -> Result<()> {
            self.signal_error(code, text.clone());
            Err(text)
        };

        let mut plugin = plugin;
        while plugin.next.is_some() {
            if plugin.magic != signatures::plugin_type::MAGIC {
                return signal_error(ErrorCode::UnknownExtension, "Unrecognized plugin".to_string());
            }
            if plugin.expected_version > LCMS_VERSION {
                return signal_error(
                    ErrorCode::UnknownExtension,
                    format!(
                        "plugin needs Little CMS {}, current version is {}",
                        plugin.expected_version, LCMS_VERSION
                    ),
                );
            }

            plugin = plugin.next.as_ref().unwrap();
        }

        Ok(())
    }

    pub fn get_user_data(&self) -> Option<&Box<[u8]>> {
        self.user_data.as_ref()
    }
    pub fn get_log_error_handler(&self) -> &Box<LogErrorHandler> {
        &self.error_handler
    }

    pub fn set_log_error_handler(&mut self, func: Option<LogErrorHandlerFunction>) {
        self.error_handler.handler = func;
    }
    
    pub fn signal_error(&self, code: ErrorCode, text: String) {
        match self.error_handler.handler {
            Some(handler) => handler(code, text),
            None => default_log_error_handler_function(code, text),
        };
    }
}

fn alloc_log_error_handler(src: Option<&Context>) -> Box<LogErrorHandler> {
    Box::new(match src {
        Some(src) => (*src.error_handler).clone(),
        None => LogErrorHandler::new(None),
    })
}
