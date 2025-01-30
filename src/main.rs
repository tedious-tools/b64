use base64::prelude::*;
use clap::{Arg, ArgAction, ArgGroup, Command};
use std::str;

fn main() {
  let encode = Arg::new("encode")
    .short('e')
    .long("encode")
    .action(ArgAction::SetTrue)
    .help("Perform base64 encoding");

  let decode = Arg::new("decode")
    .short('d')
    .long("decode")
    .action(ArgAction::SetTrue)
    .help("Perform base64 decoding");

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
  // We hilariously don't need to catch this because clap should ensure we
  // only get either encoding or decoding.
  // let is_decoding = matches.get_flag("decode");
  let is_url_safe = matches.get_flag("url_safe");
  let passed_text = matches
    .get_one::<String>("text")
    .expect("Must pass text to encode or decode")
    .clone();

  let result = if is_encoding {
    do_encoding(passed_text, is_url_safe)
  } else {
    do_decoding(passed_text, is_url_safe)
  };

  match result {
    Ok(converted) => println!("{}", converted),
    Err(error_msg) => {
      println!("{}", error_msg);
      std::process::exit(1)
    }
  }
}

fn do_encoding(text: String, url_safe: bool) -> Result<String, String> {
  if url_safe {
    Ok(BASE64_URL_SAFE.encode(text))
  } else {
    Ok(BASE64_STANDARD.encode(text))
  }
}

fn do_decoding(text: String, url_safe: bool) -> Result<String, String> {
  let decode_attempt = if url_safe {
    BASE64_URL_SAFE.decode(text)
  } else {
    BASE64_STANDARD.decode(text)
  };

  match decode_attempt {
    Err(decode_error) => Err(decode_error.to_string()),
    Ok(vec_data) => match str::from_utf8(&vec_data) {
      Ok(s) => Ok(String::from(s)),
      Err(..) => Err("Non-UTF8 data found, use another utility".to_string()),
    },
  }
}
