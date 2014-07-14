#![feature(macro_rules, globs, phase)]
#[phase(plugin, link)]
extern crate log;

extern crate msgpack;

pub use std::io;
pub use std::collections::HashMap;
pub use msgpack::{
  MsgPack,
  StreamParser
};

#[macro_export]
macro_rules! fluentd_filter(
  ($name: ident($input: ident) $block: block) => (
    fn filter($input: MsgPack) -> Vec<MsgPack> $block

    fn $name() {
      let mut parser = StreamParser::new(io::stdin());
      for $input in parser {
        for output in filter($input).iter() {
          match io::stdout().write(output.clone().into_bytes().as_slice()) {
            Ok(_) => (),
            Err(e) => warn!("{}", e)
          }
        }
      }
    }
  )
)

#[macro_export]
macro_rules! try_fluentd_filter(
  ($result: expr) => {
    match $result {
      Ok(v) => v,
      Err(_) => { return vec!() }
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
    vec!(res.to_msgpack())
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
