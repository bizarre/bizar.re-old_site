mod settings;
mod pages;
mod router;
mod components;

use yew::prelude::*;
use yew_router::{route::Route, switch::Permissive};
use settings::Settings;
use yew_services::{fetch::{FetchService, FetchTask, Request, Response}, timeout::{TimeoutService, TimeoutTask}};
use yew::format::{Nothing, Text};
use std::time::Duration;
use serde::Deserialize;

use router::{AppRouter, AppRoute};

use wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct BuildInfo {
  git_remote: Option<String>,
  git_commit_id: String,
  git_author_name: String,
  git_author_email: String,
  git_commit_summary: String,
  git_commit_time: i64
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub settings: Option<Settings>,
  pub build_info: Option<BuildInfo>
}

enum Msg {
  LoadSettings(Settings),
  LoadBuildInfo(BuildInfo),
  Error
}
struct Model {
  _link: ComponentLink<Self>,
  _fetches: Vec<FetchTask>,
  _timeout: TimeoutTask,
  props: Props,
  error: bool
}

impl Default for Props {
  fn default() -> Self {
    Self { settings: None, build_info: None }
  }
}

impl Component for Model {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let current_time_millis = js_sys::Date::new_0().get_time();

    let settings_request = Request::get(&format!("/settings.yaml?{}", current_time_millis))
    .body(Nothing)
    .expect("Failed to build request.");

    let build_info_request = Request::get(&format!("/.build_info?{}", current_time_millis))
    .body(Nothing)
    .expect("Failed to build request.");

    let fetches = vec![FetchService::fetch(settings_request, link.callback(|response: Response<Text>| {
      Msg::LoadSettings(Settings::new(response.body().as_ref().unwrap()))
    })).unwrap(), FetchService::fetch(build_info_request, link.callback(|response: Response<Text>| {
      Msg::LoadBuildInfo(serde_json::from_str(response.body().as_ref().unwrap()).unwrap())
    })).unwrap()];

    let _timeout = TimeoutService::spawn(Duration::new(5, 0), link.callback(|_res| {
      Msg::Error
    }));

    Self { props, _link: link, _fetches: fetches, _timeout, error: false }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::LoadSettings(settings) => {
        self.props.settings = Some(settings);
        true
      },

      Msg::LoadBuildInfo(build_info) => {
        self.props.build_info = Some(build_info);
        true
      }

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
    let settings = self.props.settings.clone();
    let build_info = self.props.build_info.clone();

    if build_info.is_none() {
      return html! { 
        <h1>{ "..." }</h1>
      };
    }

    let info = build_info.unwrap();
    let date = js_sys::Date::new_0();
    date.set_time((info.git_commit_time * 1000) as f64);
    let snowflake = info.git_commit_time;
   
    html! {
      <main class=classes!("h-screen", "w-screen", "flex", "p-4", "md:w-2/3", "md:m-auto", "md:px-0", "md:pt-40", "flex-wrap")>
      { if settings.is_some() { 
          html! {
          <AppRouter
              render=AppRouter::render(move |route: AppRoute| {
                match route {
                  AppRoute::Home => {
                    html! { <pages::Home settings=settings.clone().unwrap() snowflake=snowflake /> }
                  }
            
                  AppRoute::PageNotFound(Permissive(route)) => {
                    html! { <pages::NotFound route=route /> }
                  }

                  AppRoute::About => {
                    html! { <pages::About /> }
                  }

                  AppRoute::JournalEntry(date) => {
                    html! { <pages::JournalEntry date=date snowflake=snowflake /> }
                  }

                  AppRoute::Sketch(sketch) => {
                    html! { <pages::Sketch sketch=sketch snowflake=snowflake /> }
                  }
                }
              })
              redirect=AppRouter::redirect(|route: Route| {
                AppRoute::PageNotFound(Permissive(Some(route.route)))
            })
          />
      }} else {
        { if self.error {
          html! { 
            <h1 class=classes!("text-red-500")>{ "couldnt load settings" }</h1>
          }
        } else {
          html! { 
            <h1>{ "..." }</h1>
          }
        }}
      }}
      // this should probs be moved into its own component.
      // very ugly rn
        <footer class=classes!("w-full", "cursor-default", "pb-8")>
          <small class=classes!("text-gray-300", "block")>
            { "built w/ \u{2764} in rust via " }
            <a class=classes!("hover:bg-black", "hover:text-white", "underline") href={"https://github.com/yewstack/yew"}>{ "yew" }</a>
          </small>
          <small class=classes!("text-gray-300", "block")>
            { "last commit " }
            <a class=classes!("hover:bg-black", "hover:text-white", "underline") href={format!{"{}/commit/{}", info.git_remote.unwrap_or("https://github.com/bizarre/bizarre".to_owned()), info.git_commit_id}}>{ format!("[{}]", info.git_commit_id.chars().take(7).collect::<String>()) }</a>
            { " by " }
            <strong>{ format!("{} ({})", info.git_author_name, info.git_author_email) }</strong>
            { " on " }
            <span>{ date.to_date_string().as_string().unwrap() }</span>
            { " at " }
            <span>{ date.to_locale_time_string("en-US").as_string().unwrap() }</span>
            { ": " }
            <strong>{ format!("\"{}\"", info.git_commit_summary) }</strong>
            // { format!("last commit [{}] by {} ({}) on {} at {}: '{}'", info.git_commit_id.chars().take(7).collect::<String>(), info.git_author_name, info.git_author_email, date.to_date_string().as_string().unwrap(), date.to_locale_time_string("en-US").as_string().unwrap(), info.git_commit_summary)}
          </small>
        </footer>
      </main>
    }
  }

}

fn main() {
  yew::start_app::<Model>();
}
