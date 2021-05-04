use yew::prelude::*;
use serde::Deserialize;
use yew_services::{fetch::{FetchService, FetchTask, Request, Response}, timeout::{TimeoutService, TimeoutTask}};
use yew::format::{Nothing, Text};
use std::time::Duration;
use yew_functional::function_component;
use crate::router::{AppAnchor, AppRoute};

pub struct List {
  link: ComponentLink<Self>,
  props: ListProps
}

#[derive(Properties, Clone, PartialEq)]
pub struct ListProps {
  pub settings: crate::settings::Settings,
  pub sketches: Vec<String>
}

impl Component for List {
  type Message = ();
  type Properties = ListProps;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self { link, props }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    false
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    let sketches = &self.props.sketches;
   
    html!{
      <div>
        <h1 class=classes!("lowercase", "mt-4", "font-medium", "text-lg")>{ "Sketches" }</h1>
        <small class=classes!("lowercase", "text-gray-500")>{ &self.props.settings.sketches_subtitle }</small>
        <ul class=classes!("mt-2", "grid", "grid-cols-2", "md:grid-cols-4", "gap-4")>
        { sketches.iter().map(|sketch| html!{ <Stub sketch=sketch /> }).collect::<Html>() }
        </ul>
      </div>
    } 
  }

}

#[derive(Properties, Clone, PartialEq)]
pub struct StubProps {
  sketch: String
}

#[function_component(Stub)]
pub fn stub(props: &StubProps) -> Html {
  let name = props.sketch.clone();
  html! {
    <li> 
      <AppAnchor route=AppRoute::Sketch(name.to_owned()) classes="hover:bg-black hover:text-white">
        { name }
      </AppAnchor>
    </li>
  }
}