use yew::prelude::*;
use yew_functional::function_component;
use crate::router::{AppAnchor, AppRoute};
use crate::components::journal::JournalEntry;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub settings: crate::settings::Settings,
  pub app_data: crate::AppData
}

#[function_component(Home)]
pub fn home(props: &Props) -> Html {
  let entries = props.app_data.journal_entries.iter().map(|entry| { 
    JournalEntry::new(entry.get("date").unwrap().to_string(), entry.get("_path").unwrap().to_string()) 
  }).collect::<Vec<JournalEntry>>();

  let sketches = props.app_data.clone().sketches;

  html! {
    <>
      <div>
        <div class=classes!("flex", "items-center")>
          <div class=classes!("w-20", "mr-5")>
            <img class=classes!("object-scale-down", "border", "border-black") src={"/assets/images/avatar.png"} />
          </div>
          <div class=classes!("flex-1")>
            <div class=classes!("flex", "justify-between", "items-center")>
              <h1 class=classes!("lowercase", "font-medium", "text-lg", "flex", "items-center")>{ &props.settings.name }</h1>
              <AppAnchor classes="hover:bg-black hover:text-white text-md ml-1 opacity-75" route=AppRoute::About> { "/about-me" }</AppAnchor>
            </div>
            <div class=classes!("mt-2", "flex", "flex-col", "md:flex-row")>
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
              <div class=classes!("md:mt-0")>
                <p class=classes!("text-gray-500")>{ &props.settings.bio }</p>
              </div>
            </div>
          </div>
        </div>
        <div class=classes!("flex", "flex-col")>
          <div class=classes!("flex", "w-full", "items-center", "justify-between", "mt-8")>
            { props.app_data.shots.iter().map(|src| html!{
              <div class=classes!("flex-1", "odd:hidden", "p-0.5", "md:odd:inline-block", "md:px-2", "last:pr-0", "first:pl-0", "filter", "transition", "grayscale", "opacity-90", "hover:grayscale-0", "hover:opacity-100", "cursor-pointer")>
                <img class=classes!("object-scale-down") src=src />
              </div>
            }).collect::<Html>() }
          </div>
          <div class=classes!("grid", "grid-cols-1", "gap-0", "md:grid-cols-2", "md:gap-8", "mt-4")>
            <crate::components::journal::List settings=&props.settings.clone() entries=entries />
            <crate::components::sketches::List settings=&props.settings.clone() sketches=sketches />
          </div>
        </div>
      </div>
    </>
  }
}
