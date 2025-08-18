<!-- markdownlint-disable first-line-h1 no-emphasis-as-heading-->

## Installation

**Requirements**

`rpass` depends on the [`gpgme`](https://crates.io/crates/gpgme) crate for
all interactions with a host's GPG keyring. So, it requires the following OS-specific dependencies:

- **Debian/Ubuntu based systems:** `libgpgme11-dev` (`sudo apt-get -y install libgpgme11-dev`)
- **RHEL/Fedora based systems:** `gpgme-devel`
- **NixOS:** TODO!
- **Alpine:** TODO!
- **Arch:** TODO!
- **macOS:** `gnupg`
- **Windows:** [`Gpg4win`](https://www.gpg4win.org)

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
