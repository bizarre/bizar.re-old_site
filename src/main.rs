mod settings;
mod pages;
mod router;
mod components;
mod project;
mod util;

use yew::prelude::*;
use yew_router::{route::Route, switch::Permissive};
use settings::Settings;
use yew_services::{fetch::{FetchService, FetchTask, Request, Response}, timeout::{TimeoutService, TimeoutTask}};
use yew::format::{Nothing, Text};
use std::time::Duration;
use serde::Deserialize;
use yew_router::{agent::{RouteAgent, RouteRequest}};
use std::collections::HashMap;
use project::Project;

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

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct AppData {
  pub projects: Vec<String>,
  pub shots: Vec<String>,
  pub build_info: BuildInfo,
  pub sketches: Vec<String>,
  pub journal_entries: Vec<HashMap<String, String>>
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub settings: Option<Settings>,
  pub app_data: Option<AppData>,
  pub projects: Vec<Project>
}

enum Msg {
  LoadSettings(Settings),
  LoadAppData(AppData),
  AddProject(Project),
  Error
}
struct Model {
  link: ComponentLink<Self>,
  fetches: Vec<FetchTask>,
  _timeout: TimeoutTask,
  props: Props,
  error: bool
}

impl Default for Props {
  fn default() -> Self {
    Self { settings: None, app_data: None, projects: vec![] }
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

    let data_request = Request::get(&format!("/site.json?{}", current_time_millis))
    .body(Nothing)
    .expect("Failed to build request.");

    let fetches = vec![FetchService::fetch(settings_request, link.callback(|response: Response<Text>| {
      Msg::LoadSettings(Settings::new(response.body().as_ref().unwrap()))
    })).unwrap(), FetchService::fetch(data_request, link.callback(|response: Response<Text>| {
      Msg::LoadAppData(serde_json::from_str(response.body().as_ref().unwrap()).unwrap())
    })).unwrap()];

    let _timeout = TimeoutService::spawn(Duration::new(5, 0), link.callback(|_res| {
      Msg::Error
    }));

    Self { props, link, fetches, _timeout, error: false }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::LoadSettings(settings) => {
        self.props.settings = Some(settings);
        true
      }

      Msg::LoadAppData(app_data) => {
        for project in &app_data.projects {
          let request = Request::get(&format!("/content/projects/{}/project.json?{}", project, &app_data.build_info.git_commit_time))
          .body(Nothing)
          .expect("Failed to build request.");

          self.fetches.push(FetchService::fetch(request, self.link.callback(|response: Response<Text>| {
            Msg::AddProject(serde_json::from_str(response.body().as_ref().unwrap()).unwrap())
          })).unwrap());
        }

        self.props.app_data = Some(app_data);

        true
      }

      Msg::AddProject(project) => {
        self.props.projects.push(project);
        self.props.projects.sort_by(|a, b| a.id.cmp(&b.id));

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
    let app_data = self.props.app_data.clone();
    let projects = self.props.projects.clone();

    if app_data.is_none() {
      return html! { 
        <h1>{ "..." }</h1>
      };
    }
    
    let app_data = app_data.unwrap();
    let info = app_data.clone().build_info;
    let date = js_sys::Date::new_0();
    date.set_time((info.git_commit_time * 1000) as f64);
    let snowflake = info.git_commit_time;
    let shots = &app_data.shots;
   
    html! {
      <main class=classes!("h-screen", "w-screen", "flex", "flex-col", "p-4", "md:w-2/3", "md:m-auto", "md:px-0", "md:pt-40")>
      { if settings.is_some() { 
          html! {
          <AppRouter
              render=AppRouter::render(move |route: AppRoute| {
                match route {
                  AppRoute::Home => {
                    html! { <pages::Home settings=settings.clone().unwrap() app_data=app_data.clone() /> }
                  }

                  AppRoute::Projects => {
                    html! { <pages::Projects settings=settings.clone().unwrap() projects=projects.clone() /> }
                  }

                  AppRoute::Project(name) => {
                    let mut project = None;
                    for other in projects.clone() {
                      if other.name.to_lowercase() == name.to_lowercase() {
                        project = Some(other)
                      }
                    }

                    if project.is_none() {
                      if projects.len() > 0 {
                        RouteAgent::dispatcher().send(RouteRequest::ChangeRoute(Route { route: "/page-not-found".to_owned(), state: () }));
                      }

                      return html! {<></>}
                    }

                    html! { <pages::Project settings=settings.clone().unwrap() project=project.unwrap() /> }
                  }
            
                  AppRoute::PageNotFound(Permissive(route)) => {
                    html! { <pages::NotFound route=route /> }
                  }

                  AppRoute::About => {
                    html! { <pages::About snowflake=snowflake /> }
                  }

                  AppRoute::JournalEntry(date) => {
                    let mut entry = None;
                    for other in app_data.clone().journal_entries {
                      if other.get("date").unwrap().to_string() == date {
                        entry = Some(other);
                        break;
                      }
                    }

                    if entry.is_none() {
                      RouteAgent::dispatcher().send(RouteRequest::ChangeRoute(Route { route: "/page-not-found".to_owned(), state: () }));
                      return html! {<></>}
                    }

                    html! { <pages::JournalEntry date=date snowflake=snowflake path=entry.unwrap().get("_path").unwrap().to_owned() /> }
                  }

                  AppRoute::Sketch(sketch) => {
                    if (app_data.sketches.contains(&sketch)) {
                      return html! { <pages::Sketch sketch=sketch snowflake=snowflake /> }
                    }

                    RouteAgent::dispatcher().send(RouteRequest::ChangeRoute(Route { route: "/page-not-found".to_owned(), state: () }));
                    html! {<></>}
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
        <footer class=classes!("w-full", "cursor-default", "pb-8", "flex-1", "flex", "items-end")>
          <div>
            <small class=classes!("text-gray-400", "block")>
              { "built w/ \u{2764} in rust via " }
              <a class=classes!("hover:bg-black", "hover:text-white", "underline") href={"https://github.com/yewstack/yew"}>{ "yew" }</a>
            </small>
            <small class=classes!("text-gray-400", "block")>
              { "latest commit " }
              <a class=classes!("hover:bg-black", "hover:text-white", "underline") href={format!{"{}/commit/{}", info.git_remote.unwrap_or("https://github.com/bizarre/bizarre".to_owned()), info.git_commit_id}}>{ format!("[{}]", info.git_commit_id.chars().take(7).collect::<String>()) }</a>
              { " by " }
              <strong>{ format!("{} ({})", info.git_author_name, info.git_author_email) }</strong>
              { " on " }
              <span>{ date.to_date_string().as_string().unwrap() }</span>
              { " at " }
              <span>{ date.to_locale_time_string("en-US").as_string().unwrap() }</span>
              { ": " }
              <strong>{ format!("\"{}\"", info.git_commit_summary) }</strong>
            </small>
          </div>
        </footer>
      </main>
    }
  }

}

fn main() {
  yew::start_app::<Model>();
}
