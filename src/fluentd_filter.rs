#![feature(macro_rules, globs, phase)]
#[phase(plugin, link)]
extern crate log;

extern crate msgpack;

pub use std::io;
pub use std::io::IoResult;
pub use std::collections::HashMap;
pub use msgpack::{
  MsgPack,
  Map,
  StreamParser
};

pub type Event = HashMap<String, MsgPack>;
pub type FilterResult = IoResult<Vec<Event>>;

#[macro_export]
macro_rules! fluentd_filter(
  (($input: ident) $block: block) => ({
    || {
      let procedure = |$input: Event| -> FilterResult $block;
      let mut parser = StreamParser::new(io::stdin());
      for msgpack in parser {
        match msgpack {
          Map(event) => {
            let res = match procedure(event) {
              Ok(res) => res,
              Err(e) => vec!(res!{"tag": "error".into_string(), "message": format!("{}", e)})
            };
            for output in res.iter() {
              match io::stdout().write(output.to_msgpack().clone().into_bytes().as_slice()) {
                Ok(_) => (),
                Err(e) => fail!(e)
              }
            }
          }
          _ => warn!("Invalid input")
        }
      }
    }
  })
)

#[macro_export]
macro_rules! break_if_err(
  ($result: expr) => {
    match $result {
      Ok(v) => v,
      Err(_) => return Ok(vec!())
    }
  }
)

#[macro_export]
macro_rules! res_if_some(
  ($option: expr) => {
    match $option {
      Some(v) => return Ok(v),
      None => ()
    }
  }
)

#[macro_export]
macro_rules! break_if_none(
  ($option: expr) => {
    match $option {
      Some(v) => v,
      None => return Ok(vec!())
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
    res
  })
)

#[cfg(test)]
mod test {
  use super::*;

  use msgpack::ToMsgPack;

  #[test]
  fn test() {
    let test_filter = fluentd_filter!(
      (input) {
        Ok(vec!(res!{"tag": "test.test".to_string(), "message": "test message".to_string()}))
      }
    );
    
    test_filter()
  }
}
