extern crate kagura;
extern crate serde;
extern crate wasm_bindgen;
extern crate web_sys;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use kagura::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod editor;
mod enemy;

use enemy::Enemy;

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run(new(), "app");
}

struct State {
    enemy: Enemy,
}

pub enum Msg {
    InputNameOfEnemy(String),
    InputLevelOfEnemy(String),
    InputKindOfEnemy(String),
    InputIntelligenceOfEnemy(String),
    InputSensationOfEnemy(String),
    InputReactionOfEnemy(String),
    InputLanguageOfEnemy(String),
    InputHabitatOfEnemy(String),
    InputPopularityOfEnemy0(String),
    InputPopularityOfEnemy1(String),
    InputWeakPointOfEnemy(String),
    InputPreemptionOfEnemy(String),
    InputSpeedOfEnemy(String),
    InputLifeResistanceOfEnemy(String),
    InputMentalResistanceOfEnemy(String),
    InputSpecialAbilityOfEnemy(String),
    AppendPartToEnemy,
    RemovePartFromEnemy(usize),
    InputWayToAttackOfPartOfEnemy(usize, String),
    InputAccuracyOfPartOfEnemy(usize, String),
    InputDamageOfPartOfEnemy(usize, String),
    InputEvasionOfPartOfEnemy(usize, String),
    InputDefenceOfPartOfEnemy(usize, String),
    InputHpOfPartOfEnemy(usize, String),
    InputMpOfPartOfEnemy(usize, String),
    Save,
}

struct Sub();

fn new() -> Component<Msg, State, Sub> {
    Component::new(init, update, render)
}

fn init() -> (State, Cmd<Msg, Sub>) {
    let state = State {
        enemy: Enemy::new(),
    };
    (state, Cmd::none())
}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match msg {
        Msg::InputNameOfEnemy(name) => {
            state.enemy.name = name;
            Cmd::none()
        }
        Msg::InputLevelOfEnemy(level) => {
            state.enemy.level = level;
            Cmd::none()
        }
        Msg::InputKindOfEnemy(kind) => {
            state.enemy.kind = kind;
            Cmd::none()
        }
        Msg::InputIntelligenceOfEnemy(intelligence) => {
            state.enemy.intelligence = intelligence;
            Cmd::none()
        }
        Msg::InputSensationOfEnemy(sensation) => {
            state.enemy.sensation = sensation;
            Cmd::none()
        }
        Msg::InputReactionOfEnemy(reaction) => {
            state.enemy.reaction = reaction;
            Cmd::none()
        }
        Msg::InputLanguageOfEnemy(language) => {
            state.enemy.language = language;
            Cmd::none()
        }
        Msg::InputHabitatOfEnemy(habitat) => {
            state.enemy.habitat = habitat;
            Cmd::none()
        }
        Msg::InputPopularityOfEnemy0(popularity) => {
            state.enemy.popularity.0 = popularity;
            Cmd::none()
        }
        Msg::InputPopularityOfEnemy1(popularity) => {
            state.enemy.popularity.1 = popularity;
            Cmd::none()
        }
        Msg::InputWeakPointOfEnemy(weak_point) => {
            state.enemy.weak_point = weak_point;
            Cmd::none()
        }
        Msg::InputPreemptionOfEnemy(preemption) => {
            state.enemy.preemption = preemption;
            Cmd::none()
        }
        Msg::InputSpeedOfEnemy(speed) => {
            state.enemy.speed = speed;
            Cmd::none()
        }
        Msg::InputLifeResistanceOfEnemy(life_resistance) => {
            state.enemy.life_resistance = life_resistance;
            Cmd::none()
        }
        Msg::InputMentalResistanceOfEnemy(mental_resistance) => {
            state.enemy.mental_resistance = mental_resistance;
            Cmd::none()
        }
        Msg::InputSpecialAbilityOfEnemy(special_ability) => {
            state.enemy.special_ability = special_ability;
            Cmd::none()
        }
        Msg::AppendPartToEnemy => {
            state.enemy.parts.push(enemy::Part::new());
            Cmd::none()
        }
        Msg::RemovePartFromEnemy(position) => {
            state.enemy.parts.remove(position);
            Cmd::none()
        }
        Msg::InputWayToAttackOfPartOfEnemy(position, way_to_attack) => {
            if let Some(part) = state.enemy.parts.get_mut(position) {
                part.way_to_attack = way_to_attack;
            }
            Cmd::none()
        }
        Msg::InputAccuracyOfPartOfEnemy(position, accuracy) => {
            if let Some(part) = state.enemy.parts.get_mut(position) {
                part.accuracy = accuracy;
            }
            Cmd::none()
        }
        Msg::InputDamageOfPartOfEnemy(position, damage) => {
            if let Some(part) = state.enemy.parts.get_mut(position) {
                part.damage = damage;
            }
            Cmd::none()
        }
        Msg::InputEvasionOfPartOfEnemy(position, evasion) => {
            if let Some(part) = state.enemy.parts.get_mut(position) {
                part.evasion = evasion;
            }
            Cmd::none()
        }
        Msg::InputDefenceOfPartOfEnemy(position, defense) => {
            if let Some(part) = state.enemy.parts.get_mut(position) {
                part.defense = defense;
            }
            Cmd::none()
        }
        Msg::InputHpOfPartOfEnemy(position, hp) => {
            if let Some(part) = state.enemy.parts.get_mut(position) {
                part.hp = hp;
            }
            Cmd::none()
        }
        Msg::InputMpOfPartOfEnemy(position, mp) => {
            if let Some(part) = state.enemy.parts.get_mut(position) {
                part.mp = mp;
            }
            Cmd::none()
        }
        Msg::Save => {
            let save_data = serde_json::to_string_pretty(&state.enemy).unwrap();
            let blob = web_sys::Blob::new_with_str_sequence_and_options(
                &JsValue::from_serde(&[save_data]).unwrap(),
                web_sys::BlobPropertyBag::new().type_("application/json"),
            )
            .unwrap();
            let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
            let document = web_sys::window().unwrap().document().unwrap();
            let a = document.create_element("a").unwrap();
            a.set_attribute("href", &url);
            a.set_attribute("download", &(String::new() + &state.enemy.name));
            a.set_attribute("style", "display: none");
            document.body().unwrap().append_child(&a);
            a.dyn_ref::<web_sys::HtmlElement>().unwrap().click();
            document.body().unwrap().remove_child(&a);
            Cmd::none()
        }
    }
}

fn render(state: &State) -> Html<Msg> {
    Html::div(
        Attributes::new().id("app"),
        Events::new(),
        vec![render_menu(), editor::render(&state.enemy)],
    )
}

fn render_menu() -> Html<Msg> {
    Html::menu(
        Attributes::new(),
        Events::new(),
        vec![
            Html::span(
                Attributes::new()
                    .class("pure-button")
                    .class("item")
                    .string("data-selected", "true"),
                Events::new(),
                vec![Html::text("編集")],
            ),
            Html::span(
                Attributes::new().class("pure-button").class("item"),
                Events::new(),
                vec![Html::text("プレビュー")],
            ),
            Html::span(
                Attributes::new().class("pure-button").class("item"),
                Events::new().on_click(|_| Msg::Save),
                vec![Html::text("保存")],
            ),
            Html::span(
                Attributes::new().class("pure-button").class("item"),
                Events::new(),
                vec![Html::text("読み込み")],
            ),
            Html::span(
                Attributes::new().class("pure-button").class("item"),
                Events::new(),
                vec![Html::text("書き出し")],
            ),
        ],
    )
}
