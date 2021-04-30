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
        <div>
          <h1 class=classes!("lowercase", "font-medium", "text-lg")>{ &props.settings.name }</h1>
          <div class=classes!("mt-4", "flex", "flex-col", "md:flex-row")>
          { if let Some(status) = &props.settings.status {
            html! {
              <ul class=classes!("md:mr-10")>
                { status.iter().map(|line| html!{
                  <div class=classes!("cursor-default")> { line }</div>
                }).collect::<Html>() }
              </ul>
            }
          } else { 
            html!{}
          }}
            <div class=classes!("mt-4", "md:mt-0")>
              <p class=classes!("text-gray-500")>{ &props.settings.bio }</p>
            </div>
          </div>
        </div>
        <div class=classes!("grid", "grid-cols-1", "gap-0", "md:grid-cols-2", "md:gap-8", "mt-4")>       
          <crate::components::journal::List />
          <crate::components::sketches::List />
        </div>
      </div>
    </>
  }
}
