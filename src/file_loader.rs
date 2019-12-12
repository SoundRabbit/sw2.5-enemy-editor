use crate::Msg;
use kagura::prelude::*;

pub fn render() -> Html<Msg> {
    Html::div(
        Attributes::new()
            .class("pure-form")
            .id("content")
            .string("data-type", "file-loader"),
        Events::new(),
        vec![],
    )
}
