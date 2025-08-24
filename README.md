<!-- markdownlint-disable no-inline-html -->
# rpass

![Crates.io Total Downloads](https://img.shields.io/crates/d/rpass)
![Deps.rs Crate Dependencies (latest)](https://img.shields.io/deps-rs/rpass/latest)
![Crates.io Size](https://img.shields.io/crates/size/rpass)

`rpass` is a [`pass`](https://www.passwordstore.org/) inspired secrets manager.

> [!WARNING]
> ⚠️ THIS PROJECT IS AN ACTIVE WORK-IN-PROGRESS.
>
> ⚠️ CORE COMMANDS WORK BUT MAY BREAK BEAUSE THIS PROJECT IS STILL EVOLVING.
>
> ⚠️ SUBSEQUENT RELEASES MAY INCLUDE BREAKING CHANGES.

`rpass list` output:

<pre>
rPass Store
├── some-service
│   └── username@some-service.com
└── example
    └── bob@example.com
</pre>

`tree` output:

<pre>
.rstore
├── 3c11af1b-2c11-411a-bc4b-9e2aef34a928
│   └── 26ee01a5-180d-4aee-9d03-2d83154c989b.gpg
├── b206d775-8adc-4e1d-9f30-88a6cd6f1a10
│   └── 67da9fe5-81fa-4990-a022-25623b788128.gpg
└── store.toml
</pre>

## Features

- Asymmetric encryption
- Symmetric encryption
- Entry anonymization
- Clipboard support
- Git integration
- Random password generation
- Random passphrase generation
- Manage multiple stores

<!-- markdownlint-disable first-line-h1 no-emphasis-as-heading no-inline-html-->

## Installation

**Requirements**

Building `rpass` from source (including **installing it from Crates.io**) on the underlisted platforms requires that you run the given command command(s) to install the crate's system dependencies.

<details>
<summary><strong>Debian/Ubuntu based</strong></summary>

```shell
apt install -y pkg-config gcc libgpgme-dev
```

</details>

<details>
<summary><strong>RHEL/Fedora based</strong></summary>

```shell
dnf install -y gcc gpgme-devel
```

</details>

<details>
<summary><strong>NixOS (INCOMPLETE)</strong></summary>

```shell
nix-env -iA nixpkgs.gcc \
  nixpkgs.pkg-config \
  nixpkgs.gpgme.dev \
  nixpkgs.libassuan.dev \
  nixpkgs.libgpg-error.dev
```

```shell
export EXPORT $HOME/.nix-profile/lib/pkgconfig/
```

</details>

<details>
<summary><strong>Alpine (INCOMPLETE)</strong></summary>

```shell
apk add musl-dev gcc libgpg-error-dev gpgme-dev
```

</details>

<details>
<summary><strong>Arch</strong></summary>

```shell
pacman -Syu gcc pkg-config gpgme
```

</details>

<details>
<summary><strong>macOS (NOT TESTED)</strong></summary>

```shell
brew install gnupg pinentry-mac
```

</details>

<details>
<summary><strong>Windows (NOT TESTED)</strong></summary>

Download and install [`Gpg4win`](https://www.gpg4win.org)

</details>

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


<!-- markdownlint-disable first-line-h1 no-inline-html no-emphasis-as-heading -->

## Usage

> [!NOTE]
> All operations run against the store at `$HOME/.rstore` or `%USERPROFILE%/.rstore`
> by default. To override this behaviour, set the `DEFAULT_RPASS_STORE` environment
> to a different location in your shell profile/rc files.

<details>
<summary><strong>Create a store</strong></summary>

Create a store of symmetrically encrypted secrets:

```shell
rpass init
```

Create a store of asymmetrically encrypted secrets:

```shell
rpass init -k A6C4C64CCC8E8D4A278660B0A78A721FDBC087D9
```

Create a store and manage its history through revision control

```shell
rpass init -gk A6C4C64CCC8E8D4A278660B0A78A721FDBC087D9
```

Create a store in `Documents/store` by explicitly setting target store path.

```shell
rpass init -k A6C4C64CCC8E8D4A278660B0A78A721FDBC087D9 Documents/store
```

</details>

<details>
<summary><strong>Insert a secret</strong></summary>

Insert a `username` and a `password` for `bob@example.com`:

```shell
rpass insert example/bob@example.com
```

Insert a secret containing whatever field name you want:

```shell
rpass insert -c some-service/username@some-service.com
```

Echo each characters to the screen as it's enter by the user while inserting a secret:

```shell
rpass insert -e example/bob@example.com
```

Insert a secret into the store at `Documents/store`:

```shell
rpass insert example/bob@example.com Documents/store
```

</details>

<details>
<summary><strong>Generate and optionally insert secrets</strong></summary>

Generate a 32-character password (**default**):

```shell
rpass generate -p
```

Generate a 15-character password:

```shell
rpass generate -pl 15
```

Generate a 6-word passphrase:

```shell
rpass generate -P
```

Generate a 10-word passphrase:

```shell
rpass generate -Pl 10
```

Generate and insert a password for `whois@home.local`:

```shell
rpass generate -p whois@home.local
```

</details>

<details>
<summary><strong>List human-friendly secret names and fields</strong></summary>

List the fields in a secret:

```shell
rpass list whois@home.local
```

List the secrets in a group:

```shell
rpass list example
```

</details>

<details>
<summary><strong>Show the fields and values of a secret</strong></summary>

Print the value of the `password` field to the console:

```shell
rpass show whois@home.local -f password
```

Print a JSON object containing the field names and values of the `password` and `username` to the console:

```shell
rpass show whois@home.local -f password username
```

Print a JSON object containing all field names and their values to the console:

```shell
rpass show whois@home.local
```

</details>

<details>
<summary><strong>Edit an existing secret</strong></summary>

Rename the `username` field:

```shell
rpass edit whois@home.local -f username
```

Change the `username` field's value:

```shell
rpass edit whois@home.local -v username
```

Add an `api-token` field to an existing secret:

```shell
rpass edit whois@home.local -n api-token
```

</details>

<details>
<summary><strong>Delete a secret</strong></summary>

Delete a secret named `throwaway123`

```shell
rpass remove throwaway123
```

</details>

<details>
<summary><strong>Perfom Git operations against a store</strong></summary>

For example, view a summary of the store's history by running:

```shell
rpass git log --oneline
```

</details>

<details>

<summary><strong>Copy a secret</strong></summary>

Copy a secret named `whois@home.local` to `whois@localhost`:

```shell
rpass copy whois@home.local whois@localhost
```

</details>

<details>
<summary><strong>Move a secret</strong></summary>

Move a secret named `whois@localhost` to `local`:

```shell
rpass move whois@localhost local
```

</details>


<!-- markdownlint-disable first-line-h1 no-inline-html -->
---

<details>
<summary><strong>Roadmap / Planned Features</strong></summary> 

The following are on the roadmap but not yet implemented:

- `grep`: Search for strings in secrets
- `find`: Search for strings in secret pathnames
- `export`: Export secret, group, or store to JSON
- `import`: Import secrets.
- `serve`: Serve secrets over a REST API.

</details>


## License

[LICENSE](/LICENSE)