#![feature(macro_rules, globs, phase)]
#[phase(plugin, link)]
extern crate log;

extern crate msgpack;

pub use std::io;
pub use std::collections::HashMap;
pub use msgpack::{
  MsgPack,
  Map,
  StreamParser
};

#[macro_export]
macro_rules! fluentd_filter(
  ($name: ident($input: ident) $block: block) => (
    fn filter($input: HashMap<String, MsgPack>) -> Vec<HashMap<String, MsgPack>> $block

    fn $name() {
      let mut parser = StreamParser::new(io::stdin());
      for msgpack in parser {
        match msgpack {
          Map(map) => {
            for output in filter(map).iter() {
              match io::stdout().write(output.to_msgpack().clone().into_bytes().as_slice()) {
                Ok(_) => (),
                Err(e) => warn!("{}", e)
              }
            }
          }
          _ => warn!("Invalid input")
        }
      }
    }
  )
)

#[macro_export]
macro_rules! break_if_err(
  ($result: expr) => {
    match $result {
      Ok(v) => v,
      Err(_) => { return vec!() }
    }
  }
)

#[macro_export]
macro_rules! res_if_some(
  ($option: expr) => {
    match $option {
      Some(v) => return v,
      None => ()
    }
  }
)

#[macro_export]
macro_rules! to_map(
  ($msgpack: expr) => {
    match $msgpack {
      Map(map) => map,
      _ => return vec!()
    }
  }
)

#[macro_export]
macro_rules! res(
  ($($key: expr: $value: expr),+) => ({
    let mut res = HashMap::new();
    $(
      res.insert($key.into_string(), $value.to_msgpack());
    )+
    vec!(res)
  })
)

#[cfg(test)]
mod test {
  use super::*;

  use msgpack::ToMsgPack;

  fluentd_filter!(
    test_filter(input) {
      res!{"tag": "test.test".to_string(), "message": "test message".to_string()}
    }
  )
  
  #[test]
  fn test() {
    test_filter()
  }
}
