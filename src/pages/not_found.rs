use yew::prelude::*;
use yew_functional::function_component;
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub route: Option<String>
}

#[function_component(NotFound)]
pub fn not_found(_props: &Props) -> Html {
  html! {
    <h1> { "404" } </h1>
  }
}
