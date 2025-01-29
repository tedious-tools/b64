use base64::prelude::*;
use clap::{Arg, ArgAction, ArgGroup, Command};

struct Cmd {
  encoding: bool,
  decoding: bool,
  url_safe: bool,
  text: String,
}

fn main() {
  let encode = Arg::new("encode")
    .short('e')
    .long("encode")
    .action(ArgAction::SetTrue)
    .help("Provide text to base64 encode");

  let decode = Arg::new("decode")
    .short('d')
    .long("decode")
    .action(ArgAction::SetTrue)
    .help("Provide text to base64 decode");

  let action = ArgGroup::new("action")
    .args(["encode", "decode"])
    .required(true);

  let url_safe = Arg::new("url_safe")
    .short('u')
    .long("url-safe")
    .action(ArgAction::SetTrue);

  let text = Arg::new("text").value_name("TEXT").required(true);

  let cmd = Command::new("b64")
    .arg(encode)
    .arg(decode)
    .arg(url_safe)
    .arg(text)
    .group(action);

  let matches = cmd.get_matches();

  let is_encoding = matches.get_flag("encode");
  let is_decoding = matches.get_flag("decode");
  let is_url_safe = matches.get_flag("url_safe");
  let passed_text = matches
    .get_one::<String>("text")
    .expect("Must pass text to encode or decode")
    .clone();

  let val = Cmd {
    encoding: is_encoding,
    decoding: is_decoding,
    url_safe: is_url_safe,
    text: passed_text,
  };

  match val {
    Cmd {
      encoding: true,
      decoding: _,
      url_safe: false,
      text,
    } => println!("{}", BASE64_STANDARD.encode(text)),
    Cmd {
      encoding: _,
      decoding: true,
      url_safe: false,
      text,
    } => println!("{:?}", BASE64_STANDARD.decode(text)),
    Cmd {
      encoding: true,
      decoding: _,
      url_safe: true,
      text,
    } => println!("{}", BASE64_URL_SAFE.encode(text)),
    Cmd {
      encoding: _,
      decoding: true,
      url_safe: true,
      text,
    } => println!("{:?}", BASE64_URL_SAFE.decode(text)),
    _ => panic!("Something has gone terribly wrong"),
  }
}
