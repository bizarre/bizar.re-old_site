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

pub struct Sketch {
  link: ComponentLink<Self>,
  body: Option<String>,
  script: Option<String>,
  props: Props,
  _fetches: Vec<FetchTask>,
  _timeout: TimeoutTask
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub sketch: String
}

pub enum Msg {
  NotFound,
  Load,
  ApplyBody(String),
  ApplyScript(String),
  Error
}

impl Component for Sketch {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let request = Request::get("/.sketches")
    .body(Nothing)
    .expect("Failed to build request.");

    let cloned = props.clone();

    let _fetch = FetchService::fetch(request, link.callback(move |response: Response<Text>| {
      let sketches: Vec<String> = serde_json::from_str(response.body().as_ref().unwrap()).unwrap();
      for sketch in sketches {
        if sketch == cloned.sketch {
          return Msg::Load
        }
      } 

      Msg::NotFound
    })).unwrap();

    let _timeout = TimeoutService::spawn(Duration::new(5, 0), link.callback(|_res| {
      Msg::Error
    }));

    let _fetches = vec![_fetch];

    Self { link, props, body: None, script: None, _fetches, _timeout }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::NotFound => {
        RouteAgent::dispatcher().send(RouteRequest::ChangeRoute(Route { route: "/page-not-found".to_owned(), state: () }));
        false
      }

      Msg::Load => {
        let body_request = Request::get(format!("/content/sketches/{}/README.md", self.props.sketch))
        .body(Nothing)
        .expect("Failed to build request.");

        let script_request = Request::get(format!("/content/sketches/{}/script.js", self.props.sketch))
        .body(Nothing)
        .expect("Failed to build request.");

        let fetch_body = FetchService::fetch(body_request, self.link.callback(|response: Response<Text>| {
          Msg::ApplyBody(response.body().as_ref().unwrap().to_owned())
        })).unwrap();

        let fetch_script = FetchService::fetch(script_request, self.link.callback(|response: Response<Text>| {
          Msg::ApplyScript(response.body().as_ref().unwrap().to_owned())
        })).unwrap();

        self._fetches.push(fetch_body);
        self._fetches.push(fetch_script);
    
        false
      }

      Msg::ApplyBody(body) => {
        self.body = Some(markdown::to_html(&body));
        true
      }

      Msg::ApplyScript(script) => {
        self.script = Some(script);
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

        let iframe = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("iframe")
        .unwrap();

        iframe.set_attribute("src", &format!("/content/sketches/{}/sketch.html", self.props.sketch));
        iframe.set_attribute("width", "500px");
        iframe.set_attribute("height", "500px");

        let iframe_node = Node::from(iframe);
        let iframe_vnode = VNode::VRef(iframe_node);

        html! {
          <div>
            <header class=classes!("flex", "items-center", "justify-between", "mb-4")>
              <h1 class=classes!("text-lg", "font-medium", "text-gray-400")> { &self.props.sketch } </h1>
              <AppAnchor classes="hover:bg-black hover:text-white" route=AppRoute::Home> { "\u{2190} return" }</AppAnchor>
            </header>
            <section>
              <> { vnode } </>
              <> { script_vnode } </>
              <> { iframe_vnode } </>
            </section>
            <footer class=classes!("flex", "justify-between", "items-center", "mt-6", "mb-16")>
              // todo grab the github link from settings.yaml
              <a class=classes!("hover:bg-black", "hover:text-white", "lowercase") href={ &format!("https://github.com/bizarre/bizarre/blob/master/content/sketches/{}/sketch.js", &self.props.sketch) }> { "View source on GitHub" } </a>
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