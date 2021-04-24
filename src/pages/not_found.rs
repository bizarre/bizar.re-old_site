use yew::prelude::*;
use yew_functional::function_component;
use crate::router::{AppRoute, AppAnchor};
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub route: Option<String>
}

#[function_component(NotFound)]
pub fn not_found(_props: &Props) -> Html {
  html! {
    <div class=classes!("block")>
      <h1 class=classes!("text-lg", "font-medium")> { "Page not found" } </h1>
      <AppAnchor classes="w-full hover:bg-black hover:text-white" route=AppRoute::Home> { "\u{2190} Back to safety" }</AppAnchor>
    </div>
  }
}
