use headless_chrome::protocol::dom::Node;
use headless_chrome::{Element, Tab};
use log::debug;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum TranslatorError {
    #[error("Unknown error")]
    Unknown,
    #[error("Invalid Language: from=`{0}`, to=`{1}`")]
    InvalidLanguage(String, String),
    #[error("Operation on page failed because of `{0}`")]
    OperationFailure(String),
}

pub trait Translator {
    fn get_url() -> &'static str;
    fn new(tab: Arc<Tab>) -> Self;
    fn set_language(&self, from: Option<&str>, to: Option<&str>) -> Result<(), TranslatorError>;
    fn translate(&self, text: String) -> Result<String, TranslatorError>;
}

#[allow(dead_code)]
pub const INTERVAL: Duration = Duration::from_millis(100);

#[allow(dead_code)]
pub fn wait_until_focusable(element: &Element, focusable: bool, interval: Duration) {
    while element.focus().is_ok() != focusable {
        sleep(interval);
    }
}

#[allow(dead_code)]
pub fn find_elements<'a>(
    tab: &'a Tab,
    selector: &str,
    operation: &str,
) -> Result<Vec<Element<'a>>, TranslatorError> {
    debug!("find_elements(): {}", operation);
    tab.find_elements(selector).map_err(|_| {
        TranslatorError::OperationFailure(format!(
            "Cannot find elements by selector `{}` when `{}`",
            selector, operation
        ))
    })
}

#[allow(dead_code)]
pub fn find_element<'a>(
    tab: &'a Tab,
    selector: &str,
    operation: &str,
) -> Result<Element<'a>, TranslatorError> {
    debug!("find_element(): {}", operation);
    tab.find_element(selector).map_err(|_| {
        TranslatorError::OperationFailure(format!(
            "Cannot find element by selector `{}` when `{}`",
            selector, operation
        ))
    })
}

#[allow(dead_code)]
pub fn wait_for_element<'a>(
    tab: &'a Tab,
    selector: &str,
    operation: &str,
) -> Result<Element<'a>, TranslatorError> {
    debug!("wait_for_element(): {}", operation);
    tab.wait_for_element(selector).map_err(|_| {
        TranslatorError::OperationFailure(format!(
            "Cannot wait for element by selector `{}` when `{}`",
            selector, operation
        ))
    })
}

#[allow(dead_code)]
pub fn type_str<'a>(tab: &'a Tab, key: &str, operation: &str) -> Result<(), TranslatorError> {
    debug!("type_str(): {}", operation);
    tab.type_str(key).map(|_| ()).map_err(|e| {
        TranslatorError::OperationFailure(format!(
            "Cannot type `{}` str when `{}`, e={:?}",
            key, operation, e
        ))
    })
}

#[allow(dead_code)]
pub fn press_key<'a>(tab: &'a Tab, key: &str, operation: &str) -> Result<(), TranslatorError> {
    debug!("press_key(): {}", operation);
    tab.press_key(key).map(|_| ()).map_err(|_| {
        TranslatorError::OperationFailure(format!(
            "Cannot find press key `{}` when `{}`",
            key, operation
        ))
    })
}

#[allow(dead_code)]
pub fn click<'a>(element: &'a Element, operation: &str) -> Result<(), TranslatorError> {
    debug!("click(): {}", operation);
    element.click().map(|_| ()).map_err(|_| {
        TranslatorError::OperationFailure(format!("Failed to click when `{}`", operation))
    })
}

#[allow(dead_code)]
pub fn type_into<'a>(
    element: &'a Element,
    text: &str,
    operation: &str,
) -> Result<(), TranslatorError> {
    debug!("type_into(): {}", operation);
    element.type_into(text).map(|_| ()).map_err(|_| {
        TranslatorError::OperationFailure(format!(
            "Cannot type `{}` into element when `{}`",
            text, operation
        ))
    })
}

#[allow(dead_code)]
pub fn get_description(element: &Element, operation: &str) -> Result<Node, TranslatorError> {
    debug!("get_description(): {}", operation);
    element.get_description().map_err(|_| {
        TranslatorError::OperationFailure(format!("Failed to get description when `{}`", operation))
    })
}
