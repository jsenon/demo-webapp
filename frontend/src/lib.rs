#[macro_use]
extern crate yew;

use yew::prelude::*;

pub struct Model {
}

pub enum Msg {
    Click,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model { }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                // TODO: Post to Backend
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <p><a>{"Welcome on demo app !"}</a></p>
                <button onclick=|_| Msg::Click,>{ "Click" }</button>
            </div>
        }
    }
}