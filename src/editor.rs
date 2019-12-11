use crate::enemy;
use crate::Msg;
use enemy::Enemy;
use kagura::prelude::*;

pub fn render(enemy: &Enemy) -> Html<Msg> {
    Html::div(
        Attributes::new()
            .class("pure-form")
            .id("content")
            .string("data-type", "editor"),
        Events::new(),
        vec![render_name(&enemy.name)],
    )
}

fn render_name(name: &String) -> Html<Msg> {
    Html::input(
        Attributes::new().string("data-type", "name"),
        Events::new(),
        vec![],
    )
}
