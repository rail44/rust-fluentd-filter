#![feature(macro_rules, globs, phase)]
#[phase(plugin, link)]
extern crate log;

extern crate msgpack;
extern crate serialize;

pub use std::io;
pub use std::io::IoResult;
pub use std::collections::TreeMap;
pub use serialize::Encodable;
pub use msgpack::{
    Encoder,
    MsgPack,
    Map,
    StreamParser
};

pub type FilterResult = IoResult<Vec<MsgPack>>;

#[macro_export]
macro_rules! fluentd_filter(
    (($input: ident) $block: block) => ({
        || {
            let (in_tx, in_rx): (Sender<MsgPack>, Receiver<MsgPack>) = channel();
            let (out_tx, out_rx): (Sender<Vec<MsgPack>>, Receiver<Vec<MsgPack>>) = channel();

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
                    in_tx.send(input);
                }
            });

            let procedure = |$input: &MsgPack| -> FilterResult $block;

            loop {
                match in_rx.recv_opt() {
                    Ok(input) => {
                        let res = match procedure(&input) {
                            Ok(res) => res,
                            Err(e) => vec!(event!{"tag": "error".into_string(), "message": format!("{}", e)})
                        };
                        out_tx.send(res)
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
macro_rules! event(
    ($($key: expr: $value: expr),+) => ({
        let mut event = TreeMap::new();
        $(
            event.insert($key.into_string(), $value.to_msgpack());
         )+
            Map(box event)
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
                Ok(vec!(event!{"tag": "test.test".to_string(), "message": "test message".to_string()}))
            }
        );
    }
}
