use std::{fs, io};
use std::collections::HashMap;

fn main() -> io::Result<()> {
  println!("cargo:rerun-if-changed=content"); 
  println!("cargo:rerun-if-changed=build.rs");
  
  plot_journal_entries()?;
  plot_sketches()?;

  Ok(())
}


fn plot_sketches() -> io::Result<()> {
  let mut paths = fs::read_dir("content/sketches")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

  paths.sort_by(|f1, f2| fs::metadata(f1).unwrap().created().unwrap().cmp(&fs::metadata(f2).unwrap().created().unwrap()));

  let names = paths.iter().map(|path| path.as_path().file_name().unwrap().to_str().unwrap()).collect::<Vec<&str>>();

  serde_json::to_writer(&fs::File::create(".sketches.json")?, &names).unwrap();

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

  serde_json::to_writer(&fs::File::create(".journal.json")?, &entries).unwrap();

  Ok(())
}