extern crate kagura;
extern crate serde;
extern crate wasm_bindgen;
extern crate web_sys;
#[macro_use]
extern crate serde_derive;
extern crate toml;

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
    AppendPartToEnemy,
    RemovePartFromEnemy(u32),
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
        Msg::AppendPartToEnemy => {
            state.enemy.parts.push(enemy::Part::new());
            Cmd::none()
        }
        Msg::RemovePartFromEnemy(position) => {
            state.enemy.parts.remove(position as usize);
            Cmd::none()
        }
        Msg::Save => {
            let save_data = toml::to_string(&state.enemy).unwrap();
            let blob = web_sys::Blob::new_with_str_sequence_and_options(
                &JsValue::from_serde(&[save_data]).unwrap(),
                web_sys::BlobPropertyBag::new().type_("application/toml"),
            )
            .unwrap();
            let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
            let document = web_sys::window().unwrap().document().unwrap();
            let a = document.create_element("a").unwrap();
            a.set_attribute("href", &url);
            a.set_attribute("download", &(String::new() + &state.enemy.name + ".toml"));
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
