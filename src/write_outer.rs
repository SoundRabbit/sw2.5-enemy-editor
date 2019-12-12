use crate::Msg;
use kagura::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn render() -> Html<Msg> {
    Html::div(
        Attributes::new()
            .class("pure-form")
            .id("content")
            .string("data-type", "write-out"),
        Events::new(),
        vec![],
    )
}
