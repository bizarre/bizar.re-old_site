use yew::prelude::*;
use yew_functional::function_component;
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub route: Option<String>
}

#[function_component(NotFound)]
pub fn not_found(props: &Props) -> Html {
  html! {
    <h1> { props.route.as_ref().unwrap() } </h1>
  }
}
