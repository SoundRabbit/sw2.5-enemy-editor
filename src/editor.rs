use crate::enemy;
use crate::Msg;
use enemy::Enemy;
use kagura::prelude::*;
use wasm_bindgen::JsCast;

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
            content.push(render_part(part_num, part));
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
        Events::new().on_input(|a| Msg::InputNameOfEnemy(a)),
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
                    Html::input(
                        Attributes::new().value(level),
                        Events::new().on_input(|a| Msg::InputLevelOfEnemy(a)),
                        vec![],
                    ),
                ],
            ),
            Html::div(
                Attributes::new().class("pure-control-group"),
                Events::new(),
                vec![
                    Html::label(Attributes::new(), Events::new(), vec![Html::text("分類")]),
                    Html::input(
                        Attributes::new().value(kind),
                        Events::new().on_input(|a| Msg::InputKindOfEnemy(a)),
                        vec![],
                    ),
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
            render_props_1("知能", &enemy.intelligence, |a| {
                Msg::InputIntelligenceOfEnemy(a)
            }),
            render_props_1("知覚", &enemy.sensation, |a| {
                Msg::InputSensationOfEnemy(a)
            }),
            render_props_1("反応", &enemy.reaction, |a| Msg::InputReactionOfEnemy(a)),
            render_props_1("言語", &enemy.language, |a| Msg::InputLanguageOfEnemy(a)),
            render_props_1("生息地", &enemy.habitat, |a| Msg::InputHabitatOfEnemy(a)),
            render_props_2(
                "知名度",
                &enemy.popularity.0,
                &enemy.popularity.1,
                |a| Msg::InputPopularityOfEnemy0(a),
                |a| Msg::InputPopularityOfEnemy1(a),
            ),
            render_props_1("弱点", &enemy.weak_point, |a| {
                Msg::InputWeakPointOfEnemy(a)
            }),
            render_props_1("先制値", &enemy.preemption, |a| {
                Msg::InputPreemptionOfEnemy(a)
            }),
            render_props_1("移動速度", &enemy.speed, |a| Msg::InputSpeedOfEnemy(a)),
            render_props_1("生命抵抗力", &enemy.life_resistance, |a| {
                Msg::InputLifeResistanceOfEnemy(a)
            }),
            render_props_1("精神抵抗力", &enemy.mental_resistance, |a| {
                Msg::InputMentalResistanceOfEnemy(a)
            }),
        ],
    )
}

fn render_props_1(
    name: impl Into<String>,
    value: impl Into<String>,
    handler: impl FnOnce(String) -> Msg + 'static,
) -> Html<Msg> {
    Html::div(
        Attributes::new().class("pure-control-group"),
        Events::new(),
        vec![
            Html::label(Attributes::new(), Events::new(), vec![Html::text(name)]),
            Html::input(
                Attributes::new().value(value),
                Events::new().on_input(handler),
                vec![],
            ),
        ],
    )
}

fn render_props_2(
    name: impl Into<String>,
    value_1: impl Into<String>,
    value_2: impl Into<String>,
    handler_1: impl FnOnce(String) -> Msg + 'static,
    handler_2: impl FnOnce(String) -> Msg + 'static,
) -> Html<Msg> {
    Html::div(
        Attributes::new().class("pure-control-group"),
        Events::new(),
        vec![
            Html::label(Attributes::new(), Events::new(), vec![Html::text(name)]),
            Html::input(
                Attributes::new().value(value_1),
                Events::new().on_input(handler_1),
                vec![],
            ),
            Html::input(
                Attributes::new().value(value_2),
                Events::new().on_input(handler_2),
                vec![],
            ),
        ],
    )
}

fn render_part(part_num: usize, part: &enemy::Part) -> Html<Msg> {
    let position = part_num - 1;
    Html::div(
        Attributes::new(),
        Events::new(),
        vec![
            render_part_props("攻撃方法（部位）", &part.way_to_attack, move |a| {
                Msg::InputWayToAttackOfPartOfEnemy(position, a)
            }),
            render_part_props("命中", &part.accuracy, move |a| {
                Msg::InputAccuracyOfPartOfEnemy(position, a)
            }),
            render_part_props("打撃点", &part.damage, move |a| {
                Msg::InputDamageOfPartOfEnemy(position, a)
            }),
            render_part_props("回避力", &part.evasion, move |a| {
                Msg::InputEvasionOfPartOfEnemy(position, a)
            }),
            render_part_props("防護点", &part.defense, move |a| {
                Msg::InputDefenceOfPartOfEnemy(position, a)
            }),
            render_part_props("HP", &part.hp, move |a| {
                Msg::InputHpOfPartOfEnemy(position, a)
            }),
            render_part_props("MP", &part.mp, move |a| {
                Msg::InputMpOfPartOfEnemy(position, a)
            }),
        ],
    )
}

fn render_part_props(
    name: impl Into<String>,
    value: impl Into<String>,
    handler: impl FnOnce(String) -> Msg + 'static,
) -> Html<Msg> {
    Html::div(
        Attributes::new().class("pure-control-group"),
        Events::new(),
        vec![
            Html::label(Attributes::new(), Events::new(), vec![Html::text(name)]),
            Html::input(
                Attributes::new().value(value),
                Events::new().on_input(handler),
                vec![],
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

fn render_remove_part_button(part_num: usize) -> Html<Msg> {
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
    Html::textarea(
        Attributes::new().value(special_ability),
        Events::new().on("input", |e| {
            let a = e
                .target()
                .unwrap()
                .dyn_into::<web_sys::HtmlTextAreaElement>()
                .unwrap()
                .value();
            Msg::InputSpecialAbilityOfEnemy(a)
        }),
        vec![Html::text(special_ability)],
    )
}
