use yew::prelude::*;
use yew_functional::function_component;

#[function_component(Home)]
pub fn home() -> Html {
  html! {
    <h1> {"Hello"} </h1>
  }
}
