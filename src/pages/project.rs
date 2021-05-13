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

pub struct Proj {
  pub props: Props,
  pub sections: HashMap<String, ProjectSection>,
  pub fetches: Vec<FetchTask>
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub settings: crate::settings::Settings,
  pub project: Project,
}

pub enum Msg {
  LoadSection(String, String)
}

impl Component for Proj {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let fetches = props.clone().project.sections.into_iter().map(|section| {
      let request = Request::get(format!("/content/projects/{}/{}.md", &props.project.name.to_lowercase(), section))
      .body(Nothing)
      .expect("Failed to build request.");

      FetchService::fetch(request, link.callback(move |response: Response<Text>| {
        Msg::LoadSection(section.to_string(), response.body().as_ref().unwrap().to_string())
      })).unwrap()
    }).collect::<Vec<FetchTask>>();
    
    Self { props, sections: HashMap::new(), fetches }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::LoadSection(name, data) => {
        self.sections.insert(name, ProjectSection::parse(data));

        true
      }
    }
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    let project = &self.props.project;

    let mut loaded = true;
    for section in &project.sections {
      if !self.sections.contains_key(section) {
        loaded = false;
        break;
      }
    }

    html! { 
      <div class={"mb-8"}>
        <header class="mb-4">
          <div class=classes!("flex", "bg-black", "text-white", "p-8")>
            <div class=classes!("flex-1", "pr-16", "lowercase")>
              <h1 class=classes!("text-3xl", "font-bold")>{ &project.name }</h1>
              <h2 class=classes!("text-xl", "text-gray-400")>{ &project.tagline }</h2>
            </div>
            <AppAnchor classes="bg-black text-white hover:bg-white hover:text-black self-center px-4 py-2 border border-white lowercase" route=AppRoute::Projects> { "\u{2190} Go back" }</AppAnchor>
          </div>
          <div class={"flex mt-4 items-center"}>
            <div class={"md:w-6/12 md:pr-4 lowercase"}>
              <div class={"mb-4"}>
                <label class={"font-medium text-gray-500 text-sm"}>{ "Overview" }</label>
                <p>{ &project.overview }</p>
              </div>
              <div class={"mb-4"}>
                <label class={"font-medium text-gray-500 text-sm"}>{ "My Roles" }</label>
                <ul>
                  { project.roles.iter().map(|role| html! { <li>{ role }</li> }).collect::<Html>() }
                </ul>
              </div>
              { if project.team.len() > 0 {
                html! {
                  <div class={"mb-4"}>
                  <label class={"font-medium text-gray-500 text-sm"}>{ "Team" }</label>
                  <ul>
                    { project.team.iter().map(|member| {
                      if let Some(link) = &member.link {
                        html! {
                          <li><a class={"underline hover:bg-black hover:text-white"} target={"_blank"} href={link}>{ &member.name }</a>{format!(", {}", &member.role)}</li>
                        }
                      } else {
                        html! {
                          <li>{format!("{}, {}", &member.name, &member.role)}</li>
                        }
                      }
                    }).collect::<Html>() }
                  </ul>
                </div>
                }
              } else {
                html! {<></>}
              }}

              { if let Some(links) = &project.links {
                html! {
                  <div class={"mb-4"}>
                    <label class={"font-medium text-gray-500 text-sm"}>{ "Links" }</label>
                    <ul>
                    { links.iter().map(|link| {
                        html! {
                          <li><a class={"underline hover:bg-black hover:text-white"} target={"_blank"} href={&link.link}>{ &link.name }</a></li>
                        }
                    }).collect::<Html>() }
                  </ul>
                  </div>
                }
              } else {
                html!{<></>}
              }}
              <div class={"mb-4"}>
                <label class={"font-medium text-gray-500 text-sm"}>{ "When" }</label>
                <p>{ &project.date }</p>
              </div>
            </div>
            <div class={"flex-1"}>
              { if let Some(image) = &project.image {
                html! { <img class={"object-scale-down"} src={image} /> }
              } else { 
                html! { <></> } 
              }}
            </div>
          </div>
        </header>
        <hr />
        <section class=classes!("mt-8")>
          { if loaded { 
            html! { <> { project.sections.iter().map(|(section)| html! { 
              <article class={"mb-8"}> { self.sections.get(section).unwrap().to_html() } </article> }).collect::<Html>() } </> }
          } else {
            html! { <h1> {"..."} </h1> }
          }}
        </section>
      </div>
    }
  }

}
