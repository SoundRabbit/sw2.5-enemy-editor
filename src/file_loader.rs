use crate::Msg;
use kagura::prelude::*;
use wasm_bindgen::JsCast;

pub fn render() -> Html<Msg> {
    Html::div(
        Attributes::new()
            .class("pure-form")
            .id("content")
            .string("data-type", "file-loader"),
        Events::new(),
        vec![render_drop_area(), render_file_selecter()],
    )
}

fn render_drop_area() -> Html<Msg> {
    Html::div(
        Attributes::new().class("drop-area"),
        Events::new(),
        vec![Html::span(
            Attributes::new(),
            Events::new(),
            vec![Html::text("ここにファイルをドロップ（未対応）")],
        )],
    )
}

fn render_file_selecter() -> Html<Msg> {
    Html::div(
        Attributes::new().class("pure-form").class("file-selecter"),
        Events::new(),
        vec![
            Html::label(
                Attributes::new(),
                Events::new(),
                vec![Html::text("または：")],
            ),
            Html::input(
                Attributes::new().type_("file"),
                Events::new().on("change", on_select_file),
                vec![],
            ),
        ],
    )
}

fn on_select_file(e: web_sys::Event) -> Msg {
    if let Some(target) = e.target() {
        if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
            if let Some(file_list) = input.files() {
                if let Some(file) = file_list.item(0) {
                    return Msg::Load(file);
                }
            }
        }
    }
    Msg::NoOp
}
