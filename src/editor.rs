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
        vec![
            render_name(&enemy.name),
            render_level_kind(&enemy.level, &enemy.kind),
            render_props(&enemy),
        ],
    )
}

fn render_name(name: &String) -> Html<Msg> {
    Html::input(
        Attributes::new().string("data-type", "name"),
        Events::new(),
        vec![],
    )
}

fn render_level_kind(level: &String, kind: &String) -> Html<Msg> {
    Html::div(
        Attributes::new(),
        Events::new(),
        vec![
            Html::div(
                Attributes::new().class("pure-control-group"),
                Events::new(),
                vec![
                    Html::label(Attributes::new(), Events::new(), vec![Html::text("レベル")]),
                    Html::input(Attributes::new().value(level), Events::new(), vec![]),
                ],
            ),
            Html::div(
                Attributes::new().class("pure-control-group"),
                Events::new(),
                vec![
                    Html::label(Attributes::new(), Events::new(), vec![Html::text("分類")]),
                    Html::input(Attributes::new().value(kind), Events::new(), vec![]),
                ],
            ),
        ],
    )
}

fn render_props(enemy: &Enemy) -> Html<Msg> {
    Html::div(
        Attributes::new(),
        Events::new(),
        vec![
            Html::div(
                Attributes::new().class("pure-control-group"),
                Events::new(),
                vec![
                    Html::label(Attributes::new(), Events::new(), vec![Html::text("知能")]),
                    Html::input(Attributes::new().value(""), Events::new(), vec![]),
                ],
            ),
            Html::div(
                Attributes::new().class("pure-control-group"),
                Events::new(),
                vec![
                    Html::label(Attributes::new(), Events::new(), vec![Html::text("知覚")]),
                    Html::input(Attributes::new().value(""), Events::new(), vec![]),
                ],
            ),
            Html::div(
                Attributes::new().class("pure-control-group"),
                Events::new(),
                vec![
                    Html::label(Attributes::new(), Events::new(), vec![Html::text("反応")]),
                    Html::input(Attributes::new().value(""), Events::new(), vec![]),
                ],
            ),
            Html::div(
                Attributes::new().class("pure-control-group"),
                Events::new(),
                vec![
                    Html::label(Attributes::new(), Events::new(), vec![Html::text("言語")]),
                    Html::input(Attributes::new().value(""), Events::new(), vec![]),
                ],
            ),
            Html::div(
                Attributes::new().class("pure-control-group"),
                Events::new(),
                vec![
                    Html::label(Attributes::new(), Events::new(), vec![Html::text("生息地")]),
                    Html::input(Attributes::new().value(""), Events::new(), vec![]),
                ],
            ),
            Html::div(
                Attributes::new().class("pure-control-group"),
                Events::new(),
                vec![
                    Html::label(Attributes::new(), Events::new(), vec![Html::text("知名度")]),
                    Html::input(Attributes::new().value(""), Events::new(), vec![]),
                ],
            ),
            Html::div(
                Attributes::new().class("pure-control-group"),
                Events::new(),
                vec![
                    Html::label(Attributes::new(), Events::new(), vec![Html::text("弱点")]),
                    Html::input(Attributes::new().value(""), Events::new(), vec![]),
                ],
            ),
            Html::div(
                Attributes::new().class("pure-control-group"),
                Events::new(),
                vec![
                    Html::label(Attributes::new(), Events::new(), vec![Html::text("先制値")]),
                    Html::input(Attributes::new().value(""), Events::new(), vec![]),
                ],
            ),
            Html::div(
                Attributes::new().class("pure-control-group"),
                Events::new(),
                vec![
                    Html::label(
                        Attributes::new(),
                        Events::new(),
                        vec![Html::text("移動速度")],
                    ),
                    Html::input(Attributes::new().value(""), Events::new(), vec![]),
                ],
            ),
            Html::div(
                Attributes::new().class("pure-control-group"),
                Events::new(),
                vec![
                    Html::label(
                        Attributes::new(),
                        Events::new(),
                        vec![Html::text("生命抵抗力")],
                    ),
                    Html::input(Attributes::new().value(""), Events::new(), vec![]),
                ],
            ),
            Html::div(
                Attributes::new().class("pure-control-group"),
                Events::new(),
                vec![
                    Html::label(
                        Attributes::new(),
                        Events::new(),
                        vec![Html::text("精神抵抗力")],
                    ),
                    Html::input(Attributes::new().value(""), Events::new(), vec![]),
                ],
            ),
        ],
    )
}
