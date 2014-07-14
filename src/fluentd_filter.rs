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
    fn $name() {
      let mut parser = StreamParser::new(io::stdin());
      for $input in parser {
        match io::stdout().write($block.into_bytes().as_slice()) {
          Ok(_) => (),
          Err(e) => warn!("{}", e)
        }
      }
    }
  )
)

#[cfg(test)]
mod test {
  use super::*;

  use msgpack::String;

  fluentd_filter!(
    test_filter(input) {
      String("test".into_string())
    }
  )
  
  #[test]
  fn test() {
    test_filter()
  }
}
