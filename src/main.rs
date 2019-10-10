
#[macro_use]
extern crate yew;


use yew::{html, Component, ComponentLink, Html, ShouldRender, Renderable};


struct Model {

    input: String,
    todos: Vec<String>,
}

enum Msg {

    Add,
    Update(String),
    Remove(usize),
    RemoveAll,
    Nothing,
}


impl Component for Model {

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _ : ComponentLink<Self>) -> Self {
        Model { input: String::from(""), todos: vec![] }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {

        match msg {
            Msg::Add => {true},
            Msg::Update(item) => {true},
            Msg::Remove(item) => {true},
            Msg::RemoveAll => {true},
            Msg::Nothing => {true},
        }
    }

    
}

impl Renderable<Model> for Model {

    fn view(&self) -> Html<Self> {
        html! {
            <div>{"hello"}</div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
