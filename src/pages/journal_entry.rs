use chrono::prelude::*;
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
  _fetch: FetchTask,
  _timeout: TimeoutTask
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub date: String
}

pub enum Msg {
  NotFound,
  Load(String),
  Apply(String),
  Error
}

impl Component for JournalEntry {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let request = Request::get("/.journal.json")
    .body(Nothing)
    .expect("Failed to build request.");

    let cloned = props.clone();

    let _fetch = FetchService::fetch(request, link.callback(move |response: Response<Text>| {
      let entries: Vec<HashMap<String, String>> = serde_json::from_str(response.body().as_ref().unwrap()).unwrap();
      for entry in entries {
        if let Some(date) = entry.get("date") {
          if date == &cloned.date.to_owned() {
            return Msg::Load(entry.get("_path").unwrap().to_owned())
          }
        }
      }

      Msg::NotFound
    })).unwrap();

    let _timeout = TimeoutService::spawn(Duration::new(5, 0), link.callback(|_res| {
      Msg::Error
    }));

    Self { link, props, body: None, _fetch, _timeout }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::NotFound => {
        RouteAgent::dispatcher().send(RouteRequest::ChangeRoute(Route { route: "/page-not-found".to_owned(), state: () }));
        false
      }

      Msg::Load(uri) => {
        let request = Request::get(uri)
        .body(Nothing)
        .expect("Failed to build request.");
    
        let _fetch = FetchService::fetch(request, self.link.callback( |response: Response<Text>| {
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

        self._fetch = _fetch;
    
        false
      }

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
      .create_element("div")
      .unwrap();

      div.set_inner_html(body);

      let node = Node::from(div);
      let vnode = VNode::VRef(node);

      html! {
        <article class=classes!("prose")>
          <>{ vnode }</>
          <AppAnchor classes="w-full hover:bg-black hover:text-white" route=AppRoute::Home> { "\u{2190} Return" }</AppAnchor>
        </article>
      }
    } else {
      html! {
        <h1> { "loading" } </h1>
      }
    }
  }

}