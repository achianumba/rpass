<!-- markdownlint-disable first-line-h1 -->

## Installation

<!-- markdownlint-disable-next-line no-trailing-spaces no-emphasis-as-heading -->
**Requirements**

`rpass` depends on the [`gpgme`](https://crates.io/crates/gpgme) crate for
all interactions with a host's GPG keyring. So, it you need to first install
the following dependencies.

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
```

### Install from Crates.io

```shell
cargo binstall rpass
```
