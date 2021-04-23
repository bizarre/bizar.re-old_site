use yew_router::{prelude::*, switch::Permissive};

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
  #[at = "/page-not-found"]
  PageNotFound(Permissive<String>),

  #[at = "/!"]
  Home
}

pub type AppRouter = Router<AppRoute>;