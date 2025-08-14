<!-- markdownlint-disable first-line-h1 -->

## TODO

- [x] Reserve Crates.io package name.
- [x] Configure devcontainer.
- [x] Initialize a new store.
  - [x] Symmetrically encrypt store's index map as armored GPG message.
  - [x] Asymmetrically encrypt store's index map as armored GPG message.
  - [x] Save asymmetrically encrypted store's key ID alongside encrypted index map.
- [ ] Add secrets (`insert`).
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
