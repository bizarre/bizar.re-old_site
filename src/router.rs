use yew_router::{prelude::*, switch::Permissive};

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
  #[at = "/page-not-found"]
  PageNotFound(Permissive<String>),

  #[at = "/s/{sketch}"]
  Sketch(String),

  #[at = "/{date}"]
  JournalEntry(String),

  #[at = "/!"]
  Home
}

pub type AppRouter = Router<AppRoute>;
pub type AppAnchor = RouterAnchor<AppRoute>;