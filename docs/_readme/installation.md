<!-- markdownlint-disable first-line-h1 no-emphasis-as-heading no-inline-html-->

## Installation

**Dependencies**

`rpass` has the following runtime dependencies:

- [gpg](https://gnupg.org/download) for data encryption/decryption.
- [Git](https://git-scm.com) for **optional** revision control.

### Install from source

```shell
git clone https://github.com/achianumba/rpass.git
cd rpass
cargo build --release
sudo mv target/release/rpass /usr/local/bin
```

### Install from Crates.io

```shell
cargo install rpass
```
