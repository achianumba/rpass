<!-- markdownlint-disable no-inline-html -->

# rpass

[![Latest Version](https://img.shields.io/crates/v/rpass.svg)](https://crates.io/crates/rpass)
![Crates.io Total Downloads](https://img.shields.io/crates/d/rpass)
![Deps.rs Crate Dependencies (latest)](https://img.shields.io/deps-rs/rpass/latest)
![Crates.io Size](https://img.shields.io/crates/size/rpass)

`rpass` is a GPG-based secrets manager with the following features:

<details>
<summary>Cross-platform support</summary>

`rpass` currently works on x86_64 Linux and Windows but hasn't been tested on other platforms.
</details>

<details>
<summary>Asymmetric/Symmetric encryption</summary>
You have the option of using a store of symmetricly or asymmetrically encrypted 
secrets depending on whether the store is initialized using the `-k` option.

Inputs and outputs for encryption and decryption operations are passed through pipes to and from a call to `gpg` call in a child process. 
</details>

<details>
<summary>UUID-based entry anonymization</summary>
<table>
<tr>
<th><code>rpass</code> list</th>
<th><code>tree</code> output</th>
</tr>

<tr>
<td>
<pre>
rPass Store
├── some-service
│   └── username@some-service.com
└── example
    └── bob@example.com
</pre>
</td>

<td>
<pre>
.rstore
├── 3c11af1b-2c11-411a-bc4b-9e2aef34a928
│   └── 26ee01a5-180d-4aee-9d03-2d83154c989b.gpg
├── b206d775-8adc-4e1d-9f30-88a6cd6f1a10
│   └── 67da9fe5-81fa-4990-a022-25623b788128.gpg
└── store.toml
</pre>
</td>
</tr>
</table>
</details>

<details>
<summary>Random password/passphrase generation</summary>

Random passwords are generated from [printable ASCII characters](https://en.wikipedia.org/wiki/ASCII#Printable_character_table) while random passphrases are generated from [EFF's large word list](https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt).

</details>

<details>
<summary>Multi-store management</summary>

Maintain multiple stores in different locations or change the store's default location by passing the global `STORE` CLI option
or by setting the `DEFAULT_RPASS_STORE` environment variable.

</details>

<details>
<summary>Git integration</summary>

`rpass` commits generic info for each change to an entry or to the store in general without revealing specific information about entries.
</details>

<details>
<summary>Clipboard support</summary>

Subcommands such as `show` optionally copy secrets to the clipboard.
</details>
<br>

> [!WARNING]
> ⚠️ THIS PROJECT IS AN ACTIVE WORK-IN-PROGRESS.
>
> ⚠️ CORE COMMANDS WORK BUT MAY BREAK BEAUSE THIS PROJECT IS STILL EVOLVING.
>
> ⚠️ SUBSEQUENT RELEASES MAY INCLUDE BREAKING CHANGES.


<!-- markdownlint-disable first-line-h1 no-emphasis-as-heading no-inline-html-->

## Installation

**Dependencies**

`rpass` has the following runtime dependencies:

- [gpg](https://gnupg.org/download) for data encryption/decryption.
- [Git](https://git-scm.com) for **optional** revision control.

### Build/Install from Source

**Crates.io**

```shell
cargo install rpass
```

**GitHub**

```shell
cargo install --git https://github.com/achianumba/rpass.git
```

**NixOS**

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
- `export`: Export secret, group, or store to JSON or to an archive with the same file tree a recipient.
- `import`: Import secrets.
- `serve`: Serve secrets over a REST API.

</details>


## License

[LICENSE](/LICENSE)