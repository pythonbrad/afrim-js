//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use serde_wasm_bindgen::{self};
use std::collections::HashMap;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_process() {
    use afrim_js::Preprocessor;

    let mut data = HashMap::new();
    data.insert("a1".to_owned(), "à".to_owned());
    data.insert("e2".to_owned(), "é".to_owned());
    let map = serde_wasm_bindgen::to_value(&data).unwrap();

    // Process
    let mut preprocessor = Preprocessor::new(&map, 32).unwrap();
    preprocessor
        .process("a".to_owned(), "keydown".to_owned())
        .unwrap();
    preprocessor
        .process("Backspace".to_owned(), "keydown".to_owned())
        .unwrap();
    assert_eq!(preprocessor.get_input(), "".to_owned());

    preprocessor
        .process("a".to_owned(), "keydown".to_owned())
        .unwrap();
    preprocessor
        .process("1".to_owned(), "keydown".to_owned())
        .unwrap();
    assert_eq!(preprocessor.get_input(), "a1".to_owned());

    // Get commands
    assert_eq!(preprocessor.pop_stack(), "!pause".to_string());
    assert_eq!(preprocessor.pop_stack(), "".to_string());
    assert_eq!(preprocessor.pop_stack(), "!resume".to_string());
    assert_eq!(preprocessor.pop_stack(), "!pause".to_string());
    assert_eq!(preprocessor.pop_stack(), "!backspace".to_string());
    assert_eq!(preprocessor.pop_stack(), "!backspace".to_string());
    assert_eq!(preprocessor.pop_stack(), "à".to_string());
    assert_eq!(preprocessor.pop_stack(), "!resume".to_string());
}

#[wasm_bindgen_test]
fn test_translate() {
    use afrim_js::Translator;

    let mut dictionary = HashMap::new();
    dictionary.insert("hello".to_owned(), vec!["hi".to_owned()]);
    dictionary.insert("hallo".to_owned(), vec!["hola".to_owned()]);
    let dictionary = serde_wasm_bindgen::to_value(&dictionary).unwrap();

    // Translate
    let translator = Translator::new(&dictionary, false).unwrap();
    let translations: Vec<(String, String, Vec<String>, bool)> =
        serde_wasm_bindgen::from_value(translator.translate("hello")).unwrap();

    #[cfg(not(feature = "strsim"))]
    assert_eq!(
        translations,
        vec![(
            "hello".to_owned(),
            "".to_owned(),
            vec!["hi".to_owned()],
            false
        )]
    );

    #[cfg(feature = "strsim")]
    assert_eq!(
        translations,
        vec![
            (
                "hello".to_owned(),
                "".to_owned(),
                vec!["hi".to_owned()],
                false
            ),
            (
                "hallo".to_owned(),
                "".to_owned(),
                vec!["hola".to_owned()],
                false
            )
        ]
    );
}
