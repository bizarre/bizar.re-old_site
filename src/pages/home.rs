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
        <h1 class=classes!("lowercase", "font-medium", "text-lg")>{ &props.settings.name }</h1>
        <ul class=classes!("mt-4")>
          { props.settings.status.iter().map(|line| html!{
            <div class=classes!("cursor-default")> { line }</div>
          }).collect::<Html>() }
        </ul>
        <div class=classes!("grid", "grid-cols-1", "gap-0", "md:grid-cols-2", "md:gap-8")>       
          <crate::components::journal::List />
          <crate::components::sketches::List />
        </div>
      </div>
    </>
  }
}
