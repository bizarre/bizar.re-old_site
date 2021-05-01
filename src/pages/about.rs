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

pub struct About {
  link: ComponentLink<Self>,
  body: Option<String>,
  _fetch: FetchTask,
  _timeout: TimeoutTask
}

pub enum Msg {
  Apply(String),
  Error
}

impl Component for About {
  type Message = Msg;
  type Properties = ();

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let request = Request::get("/content/about.md")
    .body(Nothing)
    .expect("Failed to build request.");

    let cloned = props.clone();

    let _fetch = FetchService::fetch(request, link.callback(move |response: Response<Text>| {
      Msg::Apply(response.body().as_ref().unwrap().to_owned())
    })).unwrap();

    let _timeout = TimeoutService::spawn(Duration::new(5, 0), link.callback(|_res| {
      Msg::Error
    }));

    Self { link, body: None, _fetch, _timeout }
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
            <h1 class=classes!("text-lg", "font-medium", "text-gray-400")> { "about me" } </h1>
            <AppAnchor classes="hover:bg-black hover:text-white" route=AppRoute::Home> { "\u{2190} return" }</AppAnchor>
          </header>
          <section class=classes!("pb-16")>
            <>{ vnode }</>
            <> {script_vnode} </>
          </section>
        </div>
      }
    } else {
      html! {
        <h1> { "..." } </h1>
      }
    }
  }

}