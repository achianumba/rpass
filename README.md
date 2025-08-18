# rpass

`rpass` is a [`pass`](https://www.passwordstore.org/) inspired secrets manager.

> [!WARNING]
> ⚠️ THIS PROJECT IS CURRENTLY A WORK-IN-PROGRESS. SUBSEQUENT RELEASES MAY INCLUDE BREAKING CHANGES

## Features

- Asymmetrically encrypted secrets
- Symmetrically encrypted secrets
- Secret name/path anonymization


<!-- markdownlint-disable first-line-h1 -->

## Installation

<!-- markdownlint-disable-next-line no-trailing-spaces no-emphasis-as-heading -->
**Requirements**

`rpass` depends on the [`gpgme`](https://crates.io/crates/gpgme) crate for
all interactions with a host's GPG keyring. So, it you need to first install
the following dependencies.

- **Debian/Ubuntu based systems:** `libgpgme11-dev` (`sudo apt-get install -y libgpgme11-dev`)
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


<!-- markdownlint-disable first-line-h1 -->

## TODO

- [x] Reserve Crates.io package name.
- [x] Configure devcontainer.
- [x] Initialize a new store.
  - [x] Symmetrically encrypt store's index map as armored GPG message.
  - [x] Asymmetrically encrypt store's index map as armored GPG message.
  - [x] Save asymmetrically encrypted store's key ID alongside encrypted index map.
- [ ] Setup multi-os CI build/testing matrix
- [x] Add secrets (`insert`).
  - [x] Anonymize secret filename.
  - [x] Prevent addition of duplicate secret names to a path.
  - [x] Insert standard username/password secret.
  - [x] Insert custom fields.
  - [x] Encrypt secret.
  - [x] Echo user input.
  - [x] Suppress secret user input.
- [x] List stored secrets (`list`).
  - [x] List the fields in a secret.
  - [x] List secrets in a given path.
  - [x] List secrets in the entire store.
  - [ ] Fix store/entry tree display.
- [x] Decrypt and print a secret (`show`).
  - [x] Display the value of a single secret
  - [x] Display multiple secrets as JSON
  - [x] Optionally copy displayed secrets to the clipboard.
  - [x] Allow user defined clipboard wait time.
- [ ] Search for strings in secrets (`grep`).
- [ ] Search for secrets whose paths (`id`) match a given string (`find` | `search`).
- [ ] Update a secret's fields (`edit` | `update`).
- [ ] Generate a new secret (`generate`).
- [ ] Remove a secret (`remove` | `rm`).
- [ ] Rename/Move a secret (`rename` | `move`).
- [ ] Duplicate a secret (`copy` | `cp`).
- [ ] Implement revision control integration (`git` & `jj`).
- [ ] Implement server/client interface.
