# sttr-rs

Easily change strings and convert string representations of data. A port of [`sttr`](https://github.com/abhimanyu003/sttr) in Rust.

## Installation

```sh
cargo install --git https://github.com/uncenter/sttr-rs.git
```

## Usage

```
sttr <command> <text>
```

## Roadmap

- [x] reverse

### Encode/decode

- [ ] ascii85-encode
- [ ] ascii85-decode
- [ ] base32-decode
- [ ] base32-encode
- [ ] base64-decode
- [ ] base64-encode
- [ ] base85-encode
- [ ] base85-decode
- [ ] base64url-decode
- [ ] base64url-encode
- [ ] html-decode
- [ ] html-encode
- [ ] rot13-encode
- [ ] url-decode
- [ ] url-encode

### Hash

- [ ] bcrypt
- [ ] md5
- [ ] sha1
- [ ] sha256
- [ ] sha512

### Case conversion

- [x] upper
- [x] lower
- [x] title
- [x] alternating
- [x] camel
- [x] pascal
- [x] snake
- [x] screaming / screaming-snake
- [x] kebab

### Count

- [ ] count-chars
- [ ] count-words
- [x] count-lines

### Lines

- [ ] reverse-lines
- [ ] shuffle-lines
- [ ] sort-lines
- [ ] unique-lines

### Hex

- [ ] hex-rgb
- [ ] hex-encode
- [ ] hex-decode

### Data conversion

- [ ] json-yaml
- [ ] yaml-json
- [ ] json-toml
- [ ] toml-json
- [ ] json-ini
- [ ] ini-json

### Extract

- [ ] extract-emails
- [ ] extract-ip
- [ ] extract-urls

## License

[MIT](LICENSE)
