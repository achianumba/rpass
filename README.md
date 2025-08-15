# rpass

`rpass` is a [`pass`](https://www.passwordstore.org/) inspired secrets manager.

> [!WARNING]
> ⚠️ THIS CURRENTLY PROJECT IS A WORK-IN-PROGRESS. SUBSEQUENT RELEASES MAY INCLUDE BREAKING CHANGES

## Features

- Asymmetrically encrypted secrets
- Symmetrically encrypted secrets
- Secret name/path anonymization


<!-- markdownlint-disable first-line-h1 -->
## Installation

<!-- markdownlint-disable-next-line no-trailing-spaces -->
**Requirements** 

`rpass` depends on the [`gpgme`](https://crates.io/crates/gpgme) crate for
all interactions with a host's GPG keyring. So, it you need to first install
the following dependencies.

- **Debian/Ubuntu based systems:** `libgpgme11-dev`
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
- [ ] List stored secrets (`list`).
- [ ] Search for strings in secrets (`grep`).
- [ ] Search for secrets whose paths (`id`) match a given string (`find` | `search`).
- [ ] Decrypt and print a secret (`show`).
- [ ] Update a secret's fields (`edit` | `update`).
- [ ] Generate a new secret (`generate`).
- [ ] Remove a secret (`remove` | `rm`).
- [ ] Rename/Move a secret (`rename` | `move`).
- [ ] Duplicate a secret (`copy` | `cp`).
- [ ] Implement revision control integration (`git` & `jj`).