use crate::translator::*;
use headless_chrome::Tab;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread::sleep;
use strfmt::strfmt;

pub struct GoogleTranslator {
    tab: Arc<Tab>,
}

const GOOGLE_TRANSLATOR_URL: &str = "https://translate.google.com";
const SELECTOR_BUTTONS_LANG: &str = r#"button.szLmtb"#;
const SELECTOR_BUTTON_CLEAR_INPUT: &str = r#"button.GA2I6e"#;
const SELECTOR_SOURCE_LANG: &str =
    r#".hRFt4b .OoYv6d .MeCBDd .vSUSRc :nth-child(3) .qSb8Pe[data-language-code="{source}"]"#;
const SELECTOR_TARGET_LANG: &str =
    r#".hRFt4b .ykTHSe .MeCBDd .vSUSRc :nth-child(2) .qSb8Pe[data-language-code="{target}"]"#;
const SELECTOR_SOURCE_AUTO: &str =
    r#".hRFt4b .OoYv6d .MeCBDd .vSUSRc :nth-child(1) .qSb8Pe[data-language-code="auto"]"#;
const SELECTOR_INPUT: &str = r#"div.QFw9Te > textarea"#;
const SELECTOR_OUTPUT: &str = r#"div.eyKpYb .J0lOec:first-child"#;
const SELECTOR_OUTPUT_LINES: &str = r#"div.eyKpYb .J0lOec:first-child > span > span.JLqJ4b > span"#;
#[allow(dead_code)]
const SELECTOR_BUTTON_DISMISS_PROMOTE: &str = r#"button.M6CB1c"#;

const LANGUAGE_AUTO: &str = "auto";
const LANGUAGE_SUPPORTED_SOURCE: [&str; 3] = [LANGUAGE_AUTO, "en", "zh-CN"];
const LANGUAGE_SUPPORTED_TARGET: [&str; 2] = ["en", "zh-CN"];

impl GoogleTranslator {
    fn validate_language(from: Option<&str>, to: Option<&str>) -> Result<(), TranslatorError> {
        match (from, to) {
            (None, None) => return Err(TranslatorError::InvalidLanguage("".into(), "".into())),
            (Some(source), None) => {
                if !LANGUAGE_SUPPORTED_SOURCE.contains(&source) {
                    return Err(TranslatorError::InvalidLanguage(source.into(), "".into()));
                }
            }
            (None, Some(target)) => {
                if !LANGUAGE_SUPPORTED_TARGET.contains(&target) {
                    return Err(TranslatorError::InvalidLanguage("".into(), target.into()));
                }
            }
            (Some(source), Some(target)) => {
                if !LANGUAGE_SUPPORTED_SOURCE.contains(&source)
                    || !LANGUAGE_SUPPORTED_TARGET.contains(&target)
                {
                    return Err(TranslatorError::InvalidLanguage(
                        source.into(),
                        target.into(),
                    ));
                }
            }
        }
        Ok(())
    }
}

impl Translator for GoogleTranslator {
    fn get_url() -> &'static str {
        GOOGLE_TRANSLATOR_URL
    }

    fn new(tab: Arc<Tab>) -> Self {
        GoogleTranslator { tab }
    }

    fn set_language(&self, from: Option<&str>, to: Option<&str>) -> Result<(), TranslatorError> {
        // Validate src/target
        GoogleTranslator::validate_language(from, to)?;

        // Dismiss chrome promote dialog if needed
        if let Ok(iframe) = find_element(&self.tab, "iframe", "iframe") {
            iframe.remove_node().map_err(|_| {
                TranslatorError::OperationFailure(format!("Cannot remove iframe element"))
            })?;
        }

        // Prepare elements
        let buttons = find_elements(
            &self.tab,
            SELECTOR_BUTTONS_LANG,
            "finding expand language buttons",
        )?;
        let button_source = &buttons[0];
        let button_target = &buttons[1];

        // Prepare vars for selectors

        // Set source language
        if let Some(source) = from {
            // Prepare selector
            let mut vars = HashMap::<String, String>::new();
            vars.insert("source".into(), source.into());
            let selector = match source {
                LANGUAGE_AUTO => SELECTOR_SOURCE_AUTO.to_string(),
                _ => strfmt(SELECTOR_SOURCE_LANG, &vars).map_err(|_| {
                    TranslatorError::OperationFailure(
                        "Failed to generate selector for source language".into(),
                    )
                })?,
            }; // Note: selector is different for LANGUAGE_AUTO case
            let clickable_item =
                find_element(&self.tab, &selector, "finding source language selection")?;
            // Click button_source, select item_source, wait for card disappear
            wait_until_focusable(button_source, true, INTERVAL);
            click(button_source, "expanding source language card")?;
            wait_until_focusable(&clickable_item, true, INTERVAL);
            sleep(INTERVAL);
            click(&clickable_item, "selecting source language")?;
            wait_until_focusable(&clickable_item, false, INTERVAL);
            if to.is_some() {
                sleep(INTERVAL); // Delay some time before click target
            }
        }

        if let Some(target) = to {
            // Prepare selector
            let mut vars = HashMap::<String, String>::new();
            vars.insert("target".into(), target.into());
            let selector = strfmt(SELECTOR_TARGET_LANG, &vars).map_err(|_| {
                TranslatorError::OperationFailure(
                    "Failed to generate selector for target language".into(),
                )
            })?;
            let clickable_item =
                find_element(&self.tab, &selector, "finding target language selection")?;
            sleep(INTERVAL);
            wait_until_focusable(button_target, true, INTERVAL);
            click(button_target, "expanding target language card")?;
            wait_until_focusable(&clickable_item, true, INTERVAL);
            sleep(INTERVAL);
            click(&clickable_item, "selecting target language")?;
            wait_until_focusable(&clickable_item, false, INTERVAL);
        }

        // Prepare for some selectors
        // vars.insert("source".into(), source.into());
        // vars.insert("target".into(), target.into());
        // let selector_source = match source {
        //     "auto" => SELECTOR_SOURCE_AUTO.to_string(),
        //     _ => strfmt(SELECTOR_SOURCE_LANG, &vars).map_err(|_| {
        //         TranslatorError::OperationFailure(
        //             "Failed to generate selector for source language".into(),
        //         )
        //     })?,
        // }; // Note: selector is different for "auto" case
        // let selector_target = strfmt(SELECTOR_TARGET_LANG, &vars).map_err(|_| {
        //     TranslatorError::OperationFailure(
        //         "Failed to generate selector for target language".into(),
        //     )
        // })?;

        // let item_source = find_element(
        //     &self.tab,
        //     &selector_source,
        //     "finding source language selection",
        // )?;
        // let item_target = find_element(
        //     &self.tab,
        //     &selector_target,
        //     "finding target language selection",
        // )?;

        // Click button_source, select item_source, wait for card disappear
        // wait_until_focusable(button_source, true, INTERVAL);
        // click(button_source, "expanding source language card")?;
        // wait_until_focusable(&item_source, true, INTERVAL);
        // sleep(INTERVAL);
        // click(&item_source, "selecting source language")?;
        // wait_until_focusable(&item_source, false, INTERVAL);

        // Click button_target, select item_target, wait for card disappear
        // sleep(INTERVAL);
        // wait_until_focusable(button_target, true, INTERVAL);
        // click(button_target, "expanding target language card")?;
        // wait_until_focusable(&item_target, true, INTERVAL);
        // sleep(INTERVAL);
        // click(&item_target, "selecting target language")?;
        // wait_until_focusable(&item_target, false, INTERVAL);

        Ok(())
    }

    fn translate(&self, text: String) -> Result<String, TranslatorError> {
        // Input source text
        let source_input = find_element(&self.tab, SELECTOR_INPUT, "finding source input")?;
        let mut input_lines = text.lines();
        wait_until_focusable(&source_input, true, INTERVAL);
        loop {
            if let Some(line) = input_lines.next() {
                match line {
                    "" => press_key(&self.tab, "Enter", "Handling empty line")?,
                    _ => type_str(&self.tab, line, "Inputing text")?,
                };
                press_key(&self.tab, "Enter", "Handling next line")?;
                continue;
            }
            break;
        }

        // Get output
        let _ = wait_for_element(&self.tab, SELECTOR_OUTPUT, "Getting output span")?;
        let output_lines = find_elements(
            &self.tab,
            SELECTOR_OUTPUT_LINES,
            "finding output lines span",
        )?;
        let mut output: String = String::new();
        for line in output_lines {
            let node = get_description(&line, "Getting output line")?;
            if let Some(contents) = node.children {
                if contents.is_empty() {
                    output.push('\n');
                } else {
                    output.push_str(&contents[0].node_value);
                }
            }
        }

        // Clear input
        let button_clear = find_element(
            &self.tab,
            SELECTOR_BUTTON_CLEAR_INPUT,
            "Finding clear button",
        )?;
        wait_until_focusable(&button_clear, true, INTERVAL);
        click(&button_clear, "Clearing input text")?;
        wait_until_focusable(&button_clear, false, INTERVAL);

        Ok(output)
    }
}
