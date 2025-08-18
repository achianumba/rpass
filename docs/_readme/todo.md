<!-- markdownlint-disable first-line-h1 no-inline-html -->
<details>
<summary>TODO</summary>

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

</details>