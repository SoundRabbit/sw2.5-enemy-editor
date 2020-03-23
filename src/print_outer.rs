use crate::enemy;
use crate::enemy::Enemy;
use crate::Dialog;
use crate::Msg;
use kagura::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn render(enemy: &Enemy) -> Html<Msg> {
    Html::div(
        Attributes::new()
            .id("content")
            .string("data-type", "print-outer"),
        Events::new(),
        vec![
            render_paper(&enemy),
            Html::button(
                Attributes::new()
                    .class("pure-button")
                    .class("pure-button-primary")
                    .string("data-non-print", "true"),
                Events::new().on_click(|_| Msg::PrintOut),
                vec![Html::text("印刷")],
            ),
            Html::button(
                Attributes::new()
                    .class("pure-button")
                    .class("pure-button-primary")
                    .string("data-non-print", "true"),
                Events::new().on_click(|_| {
                    Msg::WriteOutElementAsImage(
                        web_sys::window()
                            .unwrap()
                            .document()
                            .unwrap()
                            .get_element_by_id("paper")
                            .unwrap(),
                    )
                }),
                vec![Html::text("画像として保存")],
            ),
        ],
    )
}

fn render_paper(enemy: &Enemy) -> Html<Msg> {
    Html::div(
        Attributes::new().id("paper"),
        Events::new(),
        vec![
            render_title_bar(&enemy.name, &enemy.level, &enemy.kind),
            render_props(&enemy),
            render_parts(&enemy.parts),
            render_special_ability(&enemy.special_ability),
        ],
    )
}

fn render_title_bar(
    name: impl Into<String>,
    level: impl Into<String>,
    kind: impl Into<String>,
) -> Html<Msg> {
    Html::div(
        Attributes::new().id("title-bar"),
        Events::new(),
        vec![
            Html::span(Attributes::new(), Events::new(), vec![Html::text(level)]),
            Html::span(Attributes::new(), Events::new(), vec![Html::text(name)]),
            Html::span(Attributes::new(), Events::new(), vec![Html::text(kind)]),
        ],
    )
}

fn render_props(enemy: &Enemy) -> Html<Msg> {
    Html::div(
        Attributes::new().id("props"),
        Events::new(),
        vec![
            Html::div(
                Attributes::new(),
                Events::new(),
                vec![
                    render_prop("知能", &enemy.intelligence),
                    render_prop("知覚", &enemy.sensation),
                    render_prop("反応", &enemy.reaction),
                ],
            ),
            Html::div(
                Attributes::new(),
                Events::new(),
                vec![
                    render_prop("言語", &enemy.language),
                    render_prop("生息地", &enemy.habitat),
                ],
            ),
            Html::div(
                Attributes::new(),
                Events::new(),
                vec![
                    render_prop(
                        "知名度／弱点値",
                        String::new() + &enemy.popularity.0 + "／" + &enemy.popularity.1,
                    ),
                    render_prop("弱点", &enemy.weak_point),
                ],
            ),
            Html::div(
                Attributes::new(),
                Events::new(),
                vec![
                    render_prop("先制値", &enemy.preemption),
                    render_prop("移動速度", &enemy.speed),
                    render_prop("生命抵抗力", &enemy.life_resistance),
                    render_prop("精神抵抗力", &enemy.mental_resistance),
                ],
            ),
        ],
    )
}

fn render_prop(name: impl Into<String>, value: impl Into<String>) -> Html<Msg> {
    Html::div(
        Attributes::new(),
        Events::new(),
        vec![
            Html::span(
                Attributes::new(),
                Events::new(),
                vec![Html::text(name.into() + "：")],
            ),
            Html::span(Attributes::new(), Events::new(), vec![Html::text(value)]),
        ],
    )
}

fn render_parts(parts: &Vec<enemy::Part>) -> Html<Msg> {
    let mut cells = vec![
        render_part_prop_heading("攻撃方法（部位）"),
        render_part_prop_heading("命中力"),
        render_part_prop_heading("打撃点"),
        render_part_prop_heading("回避力"),
        render_part_prop_heading("防護点"),
        render_part_prop_heading("ＨＰ"),
        render_part_prop_heading("ＭＰ"),
    ];

    for part in parts {
        if part.name == "" {
            cells.push(render_part_prop(&part.way_to_attack));
        } else {
            let value = String::new() + &part.way_to_attack + "（" + &part.name + "）";
            cells.push(render_part_prop(value));
        }
        cells.push(render_part_prop(&part.accuracy));
        cells.push(render_part_prop(&part.damage));
        cells.push(render_part_prop(&part.evasion));
        cells.push(render_part_prop(&part.defense));
        cells.push(render_part_prop(&part.hp));
        cells.push(render_part_prop(&part.mp));
    }

    Html::div(Attributes::new().id("parts"), Events::new(), cells)
}

fn render_part_prop_heading(value: impl Into<String>) -> Html<Msg> {
    Html::span(
        Attributes::new().class("heading"),
        Events::new(),
        vec![Html::text(value)],
    )
}

fn render_part_prop(value: impl Into<String>) -> Html<Msg> {
    Html::span(Attributes::new(), Events::new(), vec![Html::text(value)])
}

fn render_special_ability(value: &String) -> Html<Msg> {
    let text: Vec<&str> = value.split('\n').collect();
    let mut content = Vec::new();
    for p in text {
        for c in p.chars() {
            let mut buf = String::from("");
            buf.push(c);
            content.push(Html::span(
                Attributes::new(),
                Events::new(),
                vec![Html::text(buf)],
            ));
        }
        content.push(Html::br(Attributes::new(), Events::new(), vec![]));
    }
    Html::div(
        Attributes::new().id("special-ability"),
        Events::new(),
        vec![
            Html::span(
                Attributes::new(),
                Events::new(),
                vec![Html::text("特殊能力")],
            ),
            Html::div(
                Attributes::new().string("data-type", "special-ability-context"),
                Events::new(),
                content,
            ),
        ],
    )
}
