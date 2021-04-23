mod pages;
mod router;

use yew::prelude::*;
use yew_router::{route::Route, switch::Permissive};

use router::{AppRouter, AppRoute};

struct Model {

}

impl Component for Model {
  type Message = ();
  type Properties = ();

  fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self {}
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    false
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    html! {
        <>
          <AppRouter
              render=AppRouter::render(Self::switch)
              redirect=AppRouter::redirect(|route: Route| {
                AppRoute::PageNotFound(Permissive(Some(route.route)))
            })
          />
        </>
    }
  }

}

impl Model {
  fn switch(route: AppRoute) -> Html {
    match route {
      AppRoute::Home => {
        html! { <pages::Home/> }
      }

      AppRoute::PageNotFound(Permissive(route)) => {
        html! { <pages::NotFound route=route /> }
      }
    }
  }
}

fn main() {
    yew::start_app::<Model>();
}
