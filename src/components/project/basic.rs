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
use crate::project::{Project, ProjectSection};
use crate::util::Renderable;


#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub section: ProjectSection
}

#[function_component(Basic)]
pub fn basic(props: &Props) -> Html {
  let section = props.section.clone();
  html! {
    <div class=classes!(if section.extra.contains_key("align") && section.extra.get("align").unwrap() == "right" { "text-right" } else { "" })>
      {if let Some(subtitle) = section.subtitle { html! {
        <label class={"text-md text-gray-400 font-medium lowercase"}>{ subtitle }</label>
      }} else {
        html! {<></>}
      }}
      <h1 class={"text-4xl lowercase mb-4 text-gray-700"}>{ section.title }</h1>
      <div class=classes!(if section.extra.contains_key("align") && section.extra.get("align").unwrap() == "right" { "flex justify-end" } else { "" })>
        { section.body.render() }
      </div>
    </div>
  }
}