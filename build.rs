use std::{fs, io};
use std::collections::HashMap;

#[derive(Debug, serde::Serialize)]
struct BuildInfo {
  git_remote: Option<String>,
  git_commit_id: String,
  git_author_name: String,
  git_author_email: String,
  git_commit_summary: String,
  git_commit_time: i64
}

#[derive(Debug, serde::Serialize)]
struct AppData {
  pub projects: Vec<String>,
  pub shots: Vec<String>,
  pub build_info: Option<BuildInfo>,
  pub sketches: Vec<String>,
  pub journal_entries: Vec<HashMap<String, String>>
}

impl Default for AppData {
  fn default() -> Self {
    Self { projects: vec![], shots: vec![], build_info: None, sketches: vec![], journal_entries: vec![] }
  }
}

fn main() -> io::Result<()> {
  println!("cargo:rerun-if-changed=content"); 
  println!("cargo:rerun-if-changed=assets");
  println!("cargo:rerun-if-changed=build.rs");

  let mut app_data = AppData::default();

  plot_projects(&mut app_data)?;
  plot_shots(&mut app_data)?;
  plot_build_information(&mut app_data)?;
  plot_journal_entries(&mut app_data)?;
  plot_sketches(&mut app_data)?;

  serde_json::to_writer(&fs::File::create("site.json")?, &app_data).unwrap();

  Ok(())
}

fn plot_projects(app_data: &mut AppData) -> io::Result<()> {
  let mut paths = fs::read_dir("content/projects")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

  paths.sort_by(|f1, f2| fs::metadata(f1).unwrap().created().unwrap().cmp(&fs::metadata(f2).unwrap().created().unwrap()));

  app_data.projects = paths.iter().map(|path| path.as_path().file_name().unwrap().to_str().unwrap().to_owned()).collect::<Vec<String>>();

  Ok(())
}

fn plot_shots(app_data: &mut AppData) -> io::Result<()> {
  let mut paths = fs::read_dir("assets/images/shots")?
  .map(|res| res.map(|e| e.path()))
  .collect::<Result<Vec<_>, io::Error>>()?;

  app_data.shots = paths.iter().map(|path| format!("/{}", path.to_str().unwrap())).collect::<Vec<String>>();

  Ok(())
}

fn plot_build_information(app_data: &mut AppData) -> io::Result<()> {
  let repo = match git2::Repository::open(".") {
    Ok(repo) => repo,
    Err(err) => panic!("failed to open: {}", err),
  };

  if let Ok(head) = repo.head() {
    if let Ok(commit) = head.peel_to_commit() {
      let author = commit.author();
      let mut remote = None;

      if let Ok(origin) = repo.find_remote("origin") {
          if let Some(url) = origin.url() {
            remote = Some(url.to_owned());
          }
      }

      let build_info = BuildInfo {
        git_remote: remote,
        git_commit_id: commit.id().to_string(),
        git_author_name: author.name().unwrap_or("unknown").to_string(),
        git_author_email: author.email().unwrap_or("unknown email").to_string(),
        git_commit_summary: commit.summary().unwrap_or("").to_string(),
        git_commit_time: commit.time().seconds()
      };

      app_data.build_info = Some(build_info);
    }
  }

  Ok(())
}


fn plot_sketches(app_data: &mut AppData) -> io::Result<()> {
  let mut paths = fs::read_dir("content/sketches")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

  paths.sort_by(|f1, f2| fs::metadata(f1).unwrap().created().unwrap().cmp(&fs::metadata(f2).unwrap().created().unwrap()));

  app_data.sketches = paths.iter().map(|path| path.as_path().file_name().unwrap().to_str().unwrap().to_owned()).collect::<Vec<String>>();

  Ok(())
}

fn plot_journal_entries(app_data: &mut AppData) -> io::Result<()> {
  let mut paths = fs::read_dir("content/journal")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

  paths.sort();
  paths.reverse();

  let mut entries: Vec<HashMap<String, String>> = vec![];
  let mut keys = vec![];

  for path in paths {
    let string = fs::read_to_string(&path)?;
    let tokens = markdown::tokenize(&string);

    let mut entry: HashMap<String, String> = HashMap::new();
    let path = path.to_str().unwrap();
    entry.insert("_path".to_owned(), format!("/{}", path));

    for token in tokens {
      match token {
        markdown::Block::Header(span, _size) => {
          for element in span {
            match element {
              markdown::Span::Text(text) => {
                let split: Vec<&str> = text.split(":").collect();
                let key = split[0].to_lowercase();
                let value = split[1].trim();
                entry.insert(key, value.to_owned());
              }
              _ => {}
            }
          }

          break;
        }
        _ => {}
      }
    }
    
    if let Some(key) = entry.get("date") {
      if keys.contains(key) {
        panic!("Duplicate date for {}", &path);
      }

      keys.push(key.to_owned());
    } else {
      panic!("No date provided for {}!", &path);
    }

    entries.push(entry);
  }

  app_data.journal_entries = entries;

  Ok(())
}