use web_sys::Node;
use yew::{prelude::*, virtual_dom::VNode};
use serde::Deserialize;
use yew_services::{fetch::{FetchService, FetchTask, Request, Response}, timeout::{TimeoutService, TimeoutTask}, ConsoleService};
use yew::format::{Nothing, Text};
use std::time::Duration;
use yew_functional::function_component;
use crate::router::{AppAnchor, AppRoute};
use std::collections::HashMap;
use yew_router::{agent::{RouteAgent, RouteRequest}, route::Route};

pub struct Projects {
  props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub settings: crate::settings::Settings,
  pub projects: Vec<crate::project::Project>,
}

impl Component for Projects {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self { props }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    false
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    if props.projects.len() != self.props.projects.len() {
      self.props.projects = props.projects;
      return true;
    }

    false
  }

  fn view(&self) -> Html {
    let projects = &self.props.projects;
    let settings = &self.props.settings;

    html! { 
      <div>
        <header class=classes!("flex", "w-full", "items-center", "justify-between")>
          <div>
            <h1 class=classes!("text-lg")> { "projects" } </h1>
            <h2 class=classes!("text-gray-500")> { settings.clone().projects_subtitle } </h2>
          </div>
          <AppAnchor classes="hover:bg-black hover:text-white" route=AppRoute::Home> { "\u{2190} return" }</AppAnchor>
        </header>
        <ul class=classes!("mt-8", "flex", "flex-col", "w-full")>
          { projects.iter().map(|project| html!{ <Stub project=project /> }).collect::<Html>() }
        </ul>
      </div>
    }
  }

}


#[derive(Properties, Clone, PartialEq)]
pub struct StubProps {
  project: crate::project::Project
}

#[function_component(Stub)]
pub fn stub(props: &StubProps) -> Html {
  let project = props.project.clone();
  let name = project.name.to_lowercase();
  html! {
    <li class=classes!("w-full", "border", "border-black", "p-8", "flex", "lowercase", "bg-black", "text-white", "flex-wrap", "mb-8")>
      <div class=classes!("flex", "items-center", "justify-between", "w-full")>
          <h1 class=classes!("text-2xl", "font-medium")> { &name } </h1>
          <ul class=classes!("flex")>
            { project.tags.iter().map(|tag| html! { 
              <li class=classes!("text-gray-600", "ml-2")> { format!("#{}", tag) } </li> 
            }).collect::<Html>() }
          </ul>
      </div>
      <div class=classes!("flex-1", "pr-16")>
        <p class=classes!("text-gray-300", "mt-2")> { project.summary } </p>
      </div>
      { if project.link.is_none() {
        html! { <AppAnchor classes="bg-black text-white hover:bg-white hover:text-black self-end px-4 py-2 border border-white" route=AppRoute::Project(name)> { "Read about it \u{2192}" }</AppAnchor> }
      } else {
        html! { <a class=classes!("bg-black", "text-white", "hover:bg-white", "hover:text-black", "self-end", "px-4", "py-2", "border", "border-white") href={project.link.unwrap()}> { "Check it out \u{2192}" }</a> }
      }}
    </li>
  }
}