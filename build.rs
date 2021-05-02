use std::{fs, io};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};

fn main() -> io::Result<()> {
  println!("cargo:rerun-if-changed=content"); 
  println!("cargo:rerun-if-changed=build.rs");

  
  cache_bust()?;
  plot_build_information()?;
  plot_journal_entries()?;
  plot_sketches()?;

  Ok(())
}

fn cache_bust() -> io::Result<()> {
    let current_time_millis = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();

    let mut file = File::open(Path::new("base.html"))?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    drop(file);

    let mut replaced = data.replace(".journal", &format!(".journal?{}", current_time_millis));
    replaced = replaced.replace(".sketches", &format!(".sketches?{}", current_time_millis));
    replaced = replaced.replace(".build_info", &format!(".build_info?{}", current_time_millis));

    // Recreate the file and dump the processed contents to it
    let mut out = File::create(Path::new("index.html"))?;
    out.write(replaced.as_bytes())?;

    Ok(())
}

fn plot_build_information() -> io::Result<()> {
  let repo = match git2::Repository::open(".") {
    Ok(repo) => repo,
    Err(err) => panic!("failed to open: {}", err),
  };

  #[derive(Debug, serde::Serialize)]
  struct BuildInfo {
    git_remote: Option<String>,
    git_commit_id: String,
    git_author_name: String,
    git_author_email: String,
    git_commit_summary: String,
    git_commit_time: i64
  }

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

      serde_json::to_writer(&fs::File::create(".build_info")?, &build_info).unwrap();
    
      
    }
  }

  Ok(())
}


fn plot_sketches() -> io::Result<()> {
  let mut paths = fs::read_dir("content/sketches")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

  paths.sort_by(|f1, f2| fs::metadata(f1).unwrap().created().unwrap().cmp(&fs::metadata(f2).unwrap().created().unwrap()));

  let names = paths.iter().map(|path| path.as_path().file_name().unwrap().to_str().unwrap()).collect::<Vec<&str>>();

  serde_json::to_writer(&fs::File::create(".sketches")?, &names).unwrap();

  Ok(())
}

fn plot_journal_entries() -> io::Result<()> {
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

  serde_json::to_writer(&fs::File::create(".journal")?, &entries).unwrap();

  Ok(())
}