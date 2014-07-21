#![feature(macro_rules, globs, phase)]
#[phase(plugin, link)]
extern crate log;

extern crate msgpack;
extern crate serialize;

pub use std::io;
pub use std::io::IoResult;
pub use std::collections::HashMap;
pub use serialize::Encodable;
pub use msgpack::{
  Encoder,
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
      let (in_tx, in_rx): (Sender<Box<Event>>, Receiver<Box<Event>>) = channel();
      let (out_tx, out_rx): (Sender<Box<Vec<Event>>>, Receiver<Box<Vec<Event>>>) = channel();

      spawn(proc() {
        loop {
          match out_rx.recv_opt() {
            Ok(res) => {
              for output in res.iter() {
                let _ = output.encode(&mut Encoder::new(&mut io::stdout()));
              }
            }
            Err(_) => break
          }
        }
      });

      spawn(proc() {
        let mut parser = StreamParser::new(io::stdin());
        for input in parser {
          match input {
            Map(event) => {
              in_tx.send(event);
            }
            _ => ()
          }
        }
      });

      let procedure = |$input: &Event| -> FilterResult $block;

      loop {
        match in_rx.recv_opt() {
          Ok(input) => {
            let res = match procedure(&*input) {
              Ok(res) => res,
              Err(e) => vec!(res!{"tag": "error".into_string(), "message": format!("{}", e)})
            };
            out_tx.send(box res)
          }
          Err(_) => break
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
