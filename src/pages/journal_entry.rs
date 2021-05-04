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

pub struct JournalEntry {
  link: ComponentLink<Self>,
  body: Option<String>,
  props: Props,
  _fetch: FetchTask
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub date: String,
  pub path: String,
  pub snowflake: i64
}

pub enum Msg {
  Apply(String)
}

impl Component for JournalEntry {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let request = Request::get(format!("{}?{}", props.path, props.snowflake))
    .body(Nothing)
    .expect("Failed to build request.");

    let _fetch = FetchService::fetch(request, link.callback( |response: Response<Text>| {
      let lines: Vec<&str> = response.body().as_ref().unwrap().split("\n").collect();

      let mut space_count = 0;
      let mut count = 0;
      for (pos, line) in lines.iter().enumerate() {
        if *line == "---" {
          if space_count == 1 {
            count = pos+1;
            break;
          } else {
            space_count += 1;
          }
        }
      }

      let body = lines[count .. lines.len()].join("\n");
      Msg::Apply(body.to_owned())
    })).unwrap();

    Self { link, props, body: None, _fetch }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::Apply(body) => {
        self.body = Some(markdown::to_html(&body));
        true
      }

      _ => {
        false
      }
    }
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    if let Some(body) = &self.body {
      let div = web_sys::window()
      .unwrap()
      .document()
      .unwrap()
      .create_element("article")
      .unwrap();

      div.set_inner_html(body);
      div.set_class_name("prose");

      let node = Node::from(div);
      let vnode = VNode::VRef(node);

      let script = web_sys::window()
      .unwrap()
      .document()
      .unwrap()
      .create_element("script")
      .unwrap();

      script.set_inner_html("Prism.highlightAll()");

      let script_node = Node::from(script);
      let script_vnode = VNode::VRef(script_node);

      html! {
        <div>
          <header class=classes!("flex", "items-center", "justify-between", "mb-4")>
            <h1 class=classes!("text-lg", "font-medium", "text-gray-400")> { &self.props.date } </h1>
            <AppAnchor classes="hover:bg-black hover:text-white" route=AppRoute::Home> { "\u{2190} return" }</AppAnchor>
          </header>
          <section>
            <>{ vnode }</>
            <> {script_vnode} </>
          </section>
          <footer>
            <AppAnchor classes="hover:bg-black hover:text-white mt-6 mb-16 inline-block" route=AppRoute::Home> { "\u{2190} return" }</AppAnchor>
          </footer>
        </div>
      }
    } else {
      html! {
        <h1> { "..." } </h1>
      }
    }
  }

}