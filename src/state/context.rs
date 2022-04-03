use super::error_handler::LogErrorHandler;

#[derive(Clone)]
pub struct Context {
    pub user_data: Option<Box<[u8]>>,
    pub error_handler: Box<LogErrorHandler>,
}

impl Context {
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
}

fn alloc_log_error_handler(src: Option<&Context>) -> Box<LogErrorHandler> {
    Box::new(match src {
        Some(src) => (*src.error_handler).clone(),
        None => LogErrorHandler::new(None),
    })
}
