use yew::prelude::*;
use serde::Deserialize;
use yew_services::{fetch::{FetchService, FetchTask, Request, Response}, timeout::{TimeoutService, TimeoutTask}};
use yew::format::{Nothing, Text};
use std::time::Duration;
use yew_functional::function_component;
use crate::router::{AppAnchor, AppRoute};

#[derive(Deserialize, Clone, PartialEq)]
pub struct SketchDef(String);

pub struct List {
  link: ComponentLink<Self>,
  error: bool,
  props: ListProps,
  sketches: Vec<SketchDef>,
  _fetch: FetchTask,
  _timeout: TimeoutTask
}

#[derive(Properties, Clone, PartialEq)]
pub struct ListProps {
  pub settings: crate::settings::Settings
}

pub enum Msg {
  Init(Vec<SketchDef>),
  Error
}

impl Component for List {
  type Message = Msg;
  type Properties = ListProps;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let request = Request::get("/.sketches")
    .body(Nothing)
    .expect("Failed to build request.");

    let _fetch = FetchService::fetch(request, link.callback(|response: Response<Text>| {
      Msg::Init(serde_json::from_str(response.body().as_ref().unwrap()).unwrap())
    })).unwrap();

    let _timeout = TimeoutService::spawn(Duration::new(5, 0), link.callback(|_res| {
      Msg::Error
    }));

    Self { link, error: false, sketches: Vec::new(), _fetch, _timeout, props }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::Init(sketches) => {
        self.sketches = sketches;
        true
      },
      Msg::Error => {
        self.error = true;
        true
      }
    }
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    let sketches = &self.sketches;
   
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
  sketch: SketchDef
}

#[function_component(Stub)]
pub fn stub(props: &StubProps) -> Html {
  let sketch = props.sketch.clone();
  let SketchDef(name) = sketch;
  html! {
    <li> 
      <AppAnchor route=AppRoute::Sketch(name.to_owned()) classes="hover:bg-black hover:text-white">
        { name }
      </AppAnchor>
    </li>
  }
}