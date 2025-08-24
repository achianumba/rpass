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
