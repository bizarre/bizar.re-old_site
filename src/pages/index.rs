use yew::prelude::*;
use yew_functional::function_component;

#[function_component(Index)]
pub fn index() -> Html {
  html! {
    <h1> {"Hello"} </h1>
  }
}