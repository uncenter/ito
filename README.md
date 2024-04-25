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
- [x] replace

### Encode/decode

- [x] base64-decode
- [x] base64-encode
- [ ] base64url-decode
- [ ] base64url-encode
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

- [x] count-chars
- [x] count-words
- [x] count-lines

### Lines

- [x] reverse-lines
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

### Dates

- [ ] epoch-date (https://github.com/abhimanyu003/sttr/issues/40)
- [ ] date-epoch (https://github.com/abhimanyu003/sttr/issues/40)

## License

[MIT](LICENSE)
