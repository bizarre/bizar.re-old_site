use yew::{prelude::*, virtual_dom::VNode};
use web_sys::Node;

pub trait Renderable {
  fn render(&self) -> Html;
}

impl Renderable for &str {
  fn render(&self) -> Html {
    let div = web_sys::window()
      .unwrap()
      .document()
      .unwrap()
      .create_element("article")
      .unwrap();

    div.set_inner_html(&markdown::to_html(self));
    div.set_class_name("prose");

    let node = Node::from(div);
    let vnode = VNode::VRef(node);

    html! {
      <> {vnode} </>
    }
  }
}

impl Renderable for String {
  fn render(&self) -> Html {
    let div = web_sys::window()
      .unwrap()
      .document()
      .unwrap()
      .create_element("article")
      .unwrap();

    div.set_inner_html(&markdown::to_html(&self));
    div.set_class_name("prose");

    let node = Node::from(div);
    let vnode = VNode::VRef(node);

    html! {
      <> {vnode} </>
    }
  }
}