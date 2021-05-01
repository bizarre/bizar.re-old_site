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

use router::{AppRouter, AppRoute};

use wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub settings: Option<Settings>
}

enum Msg {
  LoadSettings(Settings),
  Error
}
struct Model {
  _link: ComponentLink<Self>,
  _fetch: FetchTask,
  _timeout: TimeoutTask,
  props: Props,
  error: bool
}

impl Default for Props {
  fn default() -> Self {
    Self { settings: None }
  }
}

impl Component for Model {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let request = Request::get("/settings.yaml")
    .body(Nothing)
    .expect("Failed to build request.");


    let _fetch = FetchService::fetch(request, link.callback(|response: Response<Text>| {
      Msg::LoadSettings(Settings::new(response.body().as_ref().unwrap()))
    })).unwrap();

    let _timeout = TimeoutService::spawn(Duration::new(5, 0), link.callback(|_res| {
      Msg::Error
    }));

    Self { props, _link: link, _fetch, _timeout, error: false }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::LoadSettings(settings) => {
        self.props.settings = Some(settings);
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
    let settings = self.props.settings.clone();
   
    html! {
      <main class=classes!("h-screen", "w-screen", "flex", "p-4", "md:w-2/3", "md:m-auto", "md:px-0", "md:pt-40")>
      { if settings.is_some() { 
          html! {
          <AppRouter
              render=AppRouter::render(move |route: AppRoute| {
                match route {
                  AppRoute::Home => {
                    html! { <pages::Home settings=settings.clone().unwrap() /> }
                  }
            
                  AppRoute::PageNotFound(Permissive(route)) => {
                    html! { <pages::NotFound route=route /> }
                  }

                  AppRoute::About => {
                    html! { <pages::About /> }
                  }

                  AppRoute::JournalEntry(date) => {
                    html! { <pages::JournalEntry date=date /> }
                  }

                  AppRoute::Sketch(sketch) => {
                    html! { <pages::Sketch sketch=sketch /> }
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
      </main>
    }
  }

}

fn main() {
  yew::start_app::<Model>();
}
