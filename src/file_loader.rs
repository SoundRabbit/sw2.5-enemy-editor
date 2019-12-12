use crate::Msg;
use kagura::prelude::*;

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
            vec![Html::text("ここにファイルをドロップ")],
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
            Html::input(Attributes::new().type_("file"), Events::new(), vec![]),
        ],
    )
}
