# ito

A powerful string manipulation tool. Originally a port of [`sttr`](https://github.com/abhimanyu003/sttr) in Rust.

## Installation

### Cargo

```sh
cargo install --git https://github.com/uncenter/ito.git
```

### Nix

```sh
nix run github:uncenter/ito
```

## Usage

```
ito <COMMAND> <TEXT>
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
- [x] shuffle-lines
- [x] sort-lines
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
