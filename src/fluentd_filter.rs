#![feature(macro_rules, globs, phase)]
#[phase(plugin, link)]
extern crate log;

extern crate msgpack;

pub use std::io;
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

#[cfg(test)]
mod test {
  use super::*;

  use msgpack::ToMsgPack;

  fluentd_filter!(
    test_filter(input) {
      vec!("test".into_string().to_msgpack())
    }
  )
  
  #[test]
  fn test() {
    test_filter()
  }
}
