# pmis

[![builds.sr.ht status](https://builds.sr.ht/~misterio/pmis.svg)](https://builds.sr.ht/~misterio/pmis?)
[![built with nix](https://img.shields.io/static/v1?logo=nixos&logoColor=white&label=&message=Built%20with%20Nix&color=41439a)](https://builtwithnix.org)

[![crates.io](https://img.shields.io/crates/v/pmis.svg)](https://crates.io/crates/pmis)

[![Packaging status](https://repology.org/badge/vertical-allrepos/pmis.svg)](https://repology.org/project/pmis/versions)

## About

### What

This is a CLI companion tool for [paste.misterio.me](https://paste.misterio.me), allowing you to easily upload and manage your pastes, as well as download any pastes you want.

### Where

The source code (licensed under GPLv3) for this app can be found (together with paste.misterio.me's) at [sourcehut](https://sr.ht/~misterio/paste.misterio.me) or [github](https://github.com/misterio77/pmis). Feel free to contribute wherever you feel more confortable.

### How

The CLI is handled by [clap](https://github.com/clap-rs/clap), the API requests are made through [reqwest](https://github.com/seanmonstar/reqwest), and the output is formatted using [bat](https://github.com/sharkdp/bat).

## Installation

`pmis` is available on crates.io, on the AUR, and there's also a nix flake in the repo for usage with nix.

### Cargo

Use `cargo install pmis`.

You can generate completions using `pmis completions <SHELL>` (check your distro docs on where to install them).

### Nix/NixOS/home-manager

You can get a shell with `pmis` using `nix shell github:misterio77/pmis`.

For a more permanent solution, you should add `pmis` to your flake inputs, add the overlay, and put it wherever you usually put packages (i recommend using `home-manager`, we even have a module you can import).

If you want to avoid compiling, `pmis` is cached on [cachix](https://app.cachix.org/cache/misterio): `cachix use misterio`.

Completions are provided through the derivation.

### Arch Linux

Use your favorite AUR helper: `paru -S pmis`.

Completions are provided through the package.

## Usage

The default API URL is `https://paste.misterio.me`, you can switch to another (if you're self hosting an instance, for example) using `--api`.

All commands and options are fully documented through `--help`

### Downloading pastes

Use `pmis download <ID>`. The output is pretty printed using `bat` (unless it is piped, or if you use `--raw`).

Do keep in mind pastes can easily be downloaded using many utilities, such as `curl`: `curl https://paste.misterio.me/p/ID/raw`. This makes it easy to get them on any barebones system or to share with friends that don't use `pmis`.

### Listing pastes

You can list a users public pastes (or all of them, if you're authenticated and the user is you) using `pmis list [OWNER]`. You can ommit `OWNER` if you're authentiucated. If you just want the IDs, add `--ids-only`.

### Authenticating

You should [generate a key](https://paste.misterio.me/keys), and then use `pmis auth`.

### Uploading pastes

Use `pmis upload [FILE]`. The title of the paste is the filename, by default. You can ommit `FILE` to read from stdin. Use `--description` to add a description, and `--unlisted` if you don't want it to appear on your profile. When the upload is complete the link and ID will be output, you can get just the link by piping or using `--link-only`.

### Deleting pastes

You can delete your pastes by using `pmis delete <ID>`.
