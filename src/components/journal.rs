use chrono::prelude::*;
use yew::prelude::*;
use serde::Deserialize;
use yew_services::{fetch::{FetchService, FetchTask, Request, Response}, timeout::{TimeoutService, TimeoutTask}};
use yew::format::{Nothing, Text};
use std::time::Duration;
use yew_functional::function_component;
use crate::router::{AppAnchor, AppRoute};

#[derive(Deserialize, Clone, PartialEq)]
pub struct JournalEntry {
  pub date: String,
  #[serde(rename = "_path")]
  pub path: String
}

pub struct List {
  link: ComponentLink<Self>,
  error: bool,
  entries: Vec<JournalEntry>,
  _fetch: FetchTask,
  _timeout: TimeoutTask
}

pub enum Msg {
  Init(Vec<JournalEntry>),
  Error
}

impl Component for List {
  type Message = Msg;
  type Properties = ();

  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let request = Request::get("/.journal.json")
    .body(Nothing)
    .expect("Failed to build request.");

    let _fetch = FetchService::fetch(request, link.callback(|response: Response<Text>| {
      Msg::Init(serde_json::from_str(response.body().as_ref().unwrap()).unwrap())
    })).unwrap();

    let _timeout = TimeoutService::spawn(Duration::new(5, 0), link.callback(|_res| {
      Msg::Error
    }));

    Self { link, error: false, entries: Vec::new(), _fetch, _timeout }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::Init(entries) => {
        self.entries = entries;
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
    let entries = &self.entries;
   
    html!{
      <>
        <h1 class=classes!("lowercase", "mt-4", "font-medium", "text-lg")>{ "Journal" }</h1>
        <ul class=classes!("mt-2", "grid", "grid-cols-2", "md:grid-cols-4", "gap-4")>
          { entries.iter().map(|entry| html!{ <Stub entry=entry /> }).collect::<Html>() }
        </ul>
      </>
    }
  }

}

#[derive(Properties, Clone, PartialEq)]
pub struct StubProps {
  entry: JournalEntry
}

#[function_component(Stub)]
pub fn stub(props: &StubProps) -> Html {
  let entry = props.entry.clone();
  html! {
    <li> 
      <AppAnchor route=AppRoute::JournalEntry(entry.date.to_owned()) classes="hover:bg-black hover:text-white">
        { entry.date }
      </AppAnchor>
    </li>
  }
}