pub use crate::request::{Catch, Endpoint};
pub use crate::response::{IntoString, Result, ToErrorResponse, ToResponse};
pub use serde_json::json;
pub use wayfinder_macros::{
    catch, connect, delete, get, head, html, options, patch, post, put, request, trace,
};

#[macro_export]
macro_rules! response {
    ($code: literal, $message: literal) => {
        Err(($code, $message.to_string()))
    };
    ($code: expr, $message: literal) => {
        Err(($code as u16, $message.to_string()))
    };
    ($result: expr) => {
        Ok($result)
    };
}

#[macro_export]
macro_rules! group {
    ($($items: expr),* $(,)?) => {
        [$(std::sync::Arc::new($items),)*]
    };
}

pub use crate::group;
pub use crate::response;
// pub use html_to_string_macro::html;
