use crate::Msg;
use kagura::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn render() -> Html<Msg> {
    Html::div(
        Attributes::new()
            .class("pure-form")
            .id("content")
            .string("data-type", "write-outer"),
        Events::new(),
        vec![render_option(
            "Udonarium用に書き出し",
            "Udonarium用のXMLファイルを書き出します。zip圧縮して利用してください。",
            || Msg::WriteOutToUdonarium,
        )],
    )
}

fn render_option(
    name: impl Into<String>,
    description: impl Into<String>,
    handler: impl FnOnce() -> Msg + 'static,
) -> Html<Msg> {
    Html::div(
        Attributes::new(),
        Events::new(),
        vec![
            Html::button(
                Attributes::new()
                    .class("pure-button")
                    .class("pure-button-primary"),
                Events::new().on_click(|_| handler()),
                vec![Html::text(name)],
            ),
            Html::span(
                Attributes::new(),
                Events::new(),
                vec![Html::text(description)],
            ),
        ],
    )
}
