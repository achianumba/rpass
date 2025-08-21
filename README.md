# rpass

`rpass` is a [`pass`](https://www.passwordstore.org/) inspired secrets manager.

> [!WARNING]
> ⚠️ THIS PROJECT IS AN ACTIVE WORK-IN-PROGRESS.
> CORE COMMANDS ARE IMPLEMENTED AND USABLE, BUT THIS PROJECT IS STILL EVOLVING. 
> ⚠️ SUBSEQUENT RELEASES MAY INCLUDE BREAKING CHANGES.

## Features

- Asymmetrically/symmetrically encrypted secrets
- Secret name/path anonymization
- Clipboard support

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


<!-- markdownlint-disable first-line-h1 -->

## Usage

The following commands have been implemented:

- `init`: Initialize a new password store
- `insert`: Add a new secret to the store
- `list`: List secrets saved in a path or list the fields saved in an entry
- `show`: Display secrets values and optionally copy them to the clipboard
- `edit`: Modify field names and values or add fields to a secret
- `remove`: Delete a secret from the store


<!-- markdownlint-disable first-line-h1 no-inline-html -->
<details>
<summary>TODO</summary>

## Roadmap / Planned Features

The following are on the roadmap but not yet implemented:

- `generate`: Generate random passwords and passphrases
- `move`: Move a secret or path to a new destination or rename the secret/path
- `copy`: Duplicate a secret
- `grep`: Search for strings in secrets
- `find`: Search for strings in secret pathnames
- `git`: Manage scecrets through revision control
- `serve`: Serve secrets over a REST API.

</details>