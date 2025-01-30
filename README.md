# b64

I built this because I wanted a small alternative to the built-in `base64` command that would
just accept a string/command line input without having to `echo` and pipe in a value to quickly
decode and encode a base64 value. I had been leaving open an `irb` session just to do this
and was tired of it.

It is focused on UTF-8 valid strings both ways. It might work for those that aren't, but I 
wouldn't bet on it. 

## Usage

First installation. You can install it via Homebrew `brew install tedious-tools/formulae/b64`.
Alternatively, `cargo install --git https://github.com/tedious-tools/b64` I think will work.

```
Usage: b64 [OPTIONS] <--encode|--decode> <TEXT>

Arguments:
  <TEXT>

Options:
  -e, --encode    Perform base64 encoding
  -d, --decode    Perform base64 decoding
  -u, --url-safe
  -h, --help      Print help
```

### Example

```
b64 -e "the dog ran away"
# dGhlIGRvZyByYW4gYXdheQ==

b64 -d dGhlIGRvZyByYW4gYXdheQ==
# the dog ran away
```

One thing I may try to add is copying to a clipboard, but that's for future me.
