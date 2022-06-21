use warp::{http::StatusCode};
use std::fs;
use std::io::{ prelude::*, BufReader};
use serde::Serialize;


//Return body structure 
#[derive(Serialize)]
struct MyResponse {
  suggestions: Vec<String>,
  correct: bool
}

//handler for POST /spellcheck/{word}

pub async fn spell_checker(
  word: String,
  ) -> Result<impl warp::Reply, warp::Rejection> {
    let suggestions = find_suggestions(word.clone());
    let length = suggestions.len();
    let mut code = StatusCode::OK;
    let mut correct = false;
    if length == 0 {
      code = StatusCode::NOT_FOUND;
    } else {
      if(length == 1) && (suggestions[0] == word.clone()) {
        correct = true;
      }
    }
    let json = warp::reply::json( &MyResponse {
        suggestions: suggestions,
        correct: correct,
    });
    Ok(warp::reply::with_status(
      json,
      code
    ))
}


pub fn find_suggestions(word: String) -> Vec<String> {
  let file = fs::File::open("dictionary.txt").expect("Failed to open dictionary"); 
  let reader = BufReader::new(file);
  let mut suggestion_list: Vec<String> = Vec::new();

  
  for line in reader.lines() {
    let line_word = line.unwrap().to_string();
    if line_word == word {
      suggestion_list = Vec::new();
      suggestion_list.push(word);
      return suggestion_list;
    }

    if line_word.clone() != line_word.clone().trim_start_matches(&word.to_lowercase()) {
        suggestion_list.push(line_word);
    }
  }
  return suggestion_list;
}