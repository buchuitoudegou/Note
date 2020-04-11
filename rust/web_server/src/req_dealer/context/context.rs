use std::collections::HashMap;
// use std::io::prelude::*;
// use std::net::TcpStream;
use std::str;

use super::request::Request;
use super::response::Response;

pub struct Context {
  pub req: Request,
  pub resp: Response,
}


impl Context {
  pub fn new(req_str: &str) -> Context {
    // let mut buffer = [0; 512];
    // stream.read(&mut buffer).unwrap();
    // let req_str = str::from_utf8(&buffer).unwrap();
    let parse_result = Context::parse(&req_str);
    Context {
      req: Request {
        method: parse_result.get("method").unwrap().to_string(),
        href: parse_result.get("href").unwrap().to_string(),
        query_string: parse_result.get("query_string").unwrap().to_string(),
        body: String::from(""),
      },
      resp: Response {
        status: 200,
        body: String::from(""),
      }
    }
  }
  fn parse(req_str: &str) -> HashMap<&'static str, String> {
    let mut result = HashMap::new();
    let lines: Vec<&str> = req_str.split("\n").collect();
    // get method
    let first_line: Vec<&str> = lines[0].split(" ").collect();
    result.insert("method", String::from(first_line[0]));
    // get path
    let path = first_line[1];
    let href_and_query: Vec<&str> = path.split("?").collect();
    result.insert("href", String::from(href_and_query[0]));
    // get query string
    if href_and_query.len() > 1 {
      let query_string = href_and_query[1];
      result.insert("query_string", String::from(query_string));
    } else {
      result.insert("query_string", String::from(""));
    }
    result
  }
}