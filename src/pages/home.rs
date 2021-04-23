use yew::prelude::*;
use yew_functional::function_component;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub settings: crate::settings::Settings
}

#[function_component(Home)]
pub fn home(props: &Props) -> Html {
  html! {
    <>
      <div>
        <h1 class=classes!("lowercase", "font-medium")>{ &props.settings.name }</h1>
      </div>
    </>
  }
}
