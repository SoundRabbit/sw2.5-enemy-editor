use crate::enemy;
use crate::Msg;
use enemy::Enemy;
use kagura::prelude::*;

pub fn render(enemy: &Enemy) -> Html<Msg> {
    let mut content = vec![
        render_name(&enemy.name),
        render_level_kind(&enemy.level, &enemy.kind),
        render_separator("基本情報"),
        render_props(&enemy),
        render_separator("部位"),
    ];
    {
        let mut part_num = 1;
        for part in &enemy.parts {
            content.push(render_part(part));
            content.push(render_remove_part_button(part_num));
            part_num = part_num + 1;
        }
    }
    content.push(render_append_part_button());
    content.push(render_separator("特殊能力"));
    content.push(render_special_ability(&enemy.special_ability));
    Html::div(
        Attributes::new()
            .class("pure-form")
            .id("content")
            .string("data-type", "editor"),
        Events::new(),
        content,
    )
}

fn render_separator(text: impl Into<String>) -> Html<Msg> {
    Html::h3(Attributes::new(), Events::new(), vec![Html::text(text)])
}

fn render_name(name: &String) -> Html<Msg> {
    Html::input(
        Attributes::new()
            .string("data-type", "name")
            .value(name)
            .placeholder("魔物の名前"),
        Events::new().on_input(|name| Msg::InputNameOfEnemy(name)),
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

fn render_part(part: &enemy::Part) -> Html<Msg> {
    Html::div(
        Attributes::new(),
        Events::new(),
        vec![
            Html::div(
                Attributes::new().class("pure-control-group"),
                Events::new(),
                vec![
                    Html::label(
                        Attributes::new(),
                        Events::new(),
                        vec![Html::text("攻撃方法")],
                    ),
                    Html::input(Attributes::new().value(""), Events::new(), vec![]),
                ],
            ),
            Html::div(
                Attributes::new().class("pure-control-group"),
                Events::new(),
                vec![
                    Html::label(Attributes::new(), Events::new(), vec![Html::text("命中")]),
                    Html::input(Attributes::new().value(""), Events::new(), vec![]),
                ],
            ),
            Html::div(
                Attributes::new().class("pure-control-group"),
                Events::new(),
                vec![
                    Html::label(Attributes::new(), Events::new(), vec![Html::text("打撃点")]),
                    Html::input(Attributes::new().value(""), Events::new(), vec![]),
                ],
            ),
            Html::div(
                Attributes::new().class("pure-control-group"),
                Events::new(),
                vec![
                    Html::label(Attributes::new(), Events::new(), vec![Html::text("回避力")]),
                    Html::input(Attributes::new().value(""), Events::new(), vec![]),
                ],
            ),
            Html::div(
                Attributes::new().class("pure-control-group"),
                Events::new(),
                vec![
                    Html::label(Attributes::new(), Events::new(), vec![Html::text("防護点")]),
                    Html::input(Attributes::new().value(""), Events::new(), vec![]),
                ],
            ),
            Html::div(
                Attributes::new().class("pure-control-group"),
                Events::new(),
                vec![
                    Html::label(Attributes::new(), Events::new(), vec![Html::text("HP")]),
                    Html::input(Attributes::new().value(""), Events::new(), vec![]),
                ],
            ),
            Html::div(
                Attributes::new().class("pure-control-group"),
                Events::new(),
                vec![
                    Html::label(Attributes::new(), Events::new(), vec![Html::text("MP")]),
                    Html::input(Attributes::new().value(""), Events::new(), vec![]),
                ],
            ),
        ],
    )
}

fn render_append_part_button() -> Html<Msg> {
    Html::button(
        Attributes::new()
            .class("pure-button")
            .class("pure-button-primary"),
        Events::new().on_click(|_| Msg::AppendPartToEnemy),
        vec![Html::text("部位を追加")],
    )
}

fn render_remove_part_button(part_num: u32) -> Html<Msg> {
    let position = part_num - 1;
    Html::button(
        Attributes::new()
            .class("pure-button")
            .class("button-secondary")
            .class("remove-part-button"),
        Events::new().on_click(move |_| Msg::RemovePartFromEnemy(position)),
        vec![Html::text(
            String::from("部位") + &part_num.to_string() + "を削除",
        )],
    )
}

fn render_special_ability(special_ability: &String) -> Html<Msg> {
    Html::textarea(Attributes::new(), Events::new(), vec![])
}
