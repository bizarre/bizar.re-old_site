use serde::Deserialize;
use std::collections::HashMap;
use yew::prelude::*;
use yew_services::{fetch::{FetchService, FetchTask, Request, Response}, timeout::{TimeoutService, TimeoutTask}, ConsoleService};

#[derive(Deserialize, Clone, PartialEq)]
pub struct TeamMember {
  pub name: String,
  pub role: String,
  pub link: Option<String>
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct Project {
  pub name: String,
  pub tags: Vec<String>,
  pub tagline: String,
  pub image: Option<String>,
  pub summary: String,
  pub overview: String,
  pub roles: Vec<String>,
  pub team: Vec<TeamMember>,
  pub date: String,
  pub sections: Vec<String>,
  pub link: Option<String>
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct ProjectSection {
  pub title: String,
  pub subtitle: Option<String>,
  pub layout: Option<String>,
  pub extra: HashMap<String, String>,
  pub body: String
}

impl ProjectSection {
  pub fn parse(data: String) -> Self {
    let tokens = markdown::tokenize(&data);

    let mut entry: HashMap<String, String> = HashMap::new();
    let lines: Vec<&str> = data.split("\n").collect();

    let mut space_count = 0;
    let mut count = 0;
    for (pos, line) in lines.iter().enumerate() {
      if *line == "---" {
        if space_count == 1 {
          count = pos+1;
          break;
        } else {
          space_count += 1;
        }
      } else {
        let split = line.split(":").collect::<Vec<&str>>();
        if split.len() == 2 {
          entry.insert(split[0].to_owned(), split[1].trim().to_owned());
        }
      }
    }

    let body = lines[count .. lines.len()].join("\n");

    Self {
      title: entry.get("title").map(String::to_owned).unwrap(),
      subtitle: entry.get("subtitle").map(String::to_owned),
      layout: entry.get("layout").map(String::to_owned),
      extra: entry,
      body: body
    }
    
  }

  pub fn to_html(&self) -> Html {
    match self.layout.as_ref().map(String::as_str) {
      Some("image") => {
        html!{<crate::components::project::Image section=self />}
      }

      _ => {
        html!{<crate::components::project::Basic section=self />}
      }
    }
  }
}