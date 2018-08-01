use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlParagraphElement;

#[wasm_bindgen(module = "./tests/wasm/element.js")]
extern {
    fn new_paragraph() -> HtmlParagraphElement;
}

#[wasm_bindgen_test]
fn test_paragraph_element() {
    let paragraph = new_paragraph();
    paragraph.set_align("right");
    assert_eq!(paragraph.align(), "right", "Paragraph should be aligned 'right'.");
}