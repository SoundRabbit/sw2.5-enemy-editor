extern crate kagura;
extern crate serde;
extern crate wasm_bindgen;
extern crate web_sys;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use kagura::prelude::*;
use wasm_bindgen::prelude::*;

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

pub enum Msg {}

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
    Cmd::none()
}

fn render(state: &State) -> Html<Msg> {
    Html::div(
        Attributes::new().id("app"),
        Events::new(),
        vec![render_menu(), editor::render(&state.enemy)],
    )
}

fn render_menu() -> Html<Msg> {
    Html::menu(Attributes::new(), Events::new(), vec![])
}
