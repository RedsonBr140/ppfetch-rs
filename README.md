<h1 align="center"><code>ppfetch</code></h1>
<p align="center">ppfetch, but written in Rust.</p>
<p align="center">
<img src="https://github.com/RedsonBr140/ppfetch-rs/actions/workflows/checks.yml/badge.svg"> 
<img src="https://img.shields.io/github/commits-since/RedsonBr140/ppfetch-rs/latest/master">
<img src="https://img.shields.io/github/license/RedsonBr140/ppfetch-rs?style=flat">
</p>

<p align="center">
  <a href="ppfetch-show.png">
    <img src="ppfetch-show.png">
  </a>
</p>

## ๐ก **About**
The ppfetch was created to be a simple, basic, and posix fetch. Now, it's simple, fast and written in Rust.

## ๐ Setup

### ๐งพ Dependencies

- [`sh`](https://en.wikipedia.org/wiki/Unix_shell)
- [`wmctrl`](https://www.freedesktop.org/wiki/Software/wmctrl/)
- [`Rust`](https://rust-lang.org/)

## ๐ฅ **Installation**

#### ๐ง Manually

Option 1: using `curl` (release binary)
```sh
curl https://github.com/RedsonBr140/ppfetch-rs/releases/latest/download/ppfetch-rs > ~/.local/bin/ppfetch-rs
chmod +x ~/.local/bin/ppfetch-rs
```
(Make sure that you have `~/.local/bin` on your `$PATH`.)

Option 2: Compiling from `source`:

```sh
git clone https://github.com/RedsonBr140/ppfetch-rs.git
cd ppfetch-rs
cargo build --release
cargo install --path .
```
(Make sure that you have `~/.cargo/bin` on your `$PATH`.)

#### ๐ฆ Package manager

For [AUR](https://aur.archlinux.org) users:
```sh
yay -S ppfetch-rs-bin # For the release binary.
yay -S ppfetch-rs-git # If you're redpill and want to compile
```
> *If you can and want to port ppfetch-rs to any package manager, feel free to do so.*

## ๐ Todo
 - [ ] Remove uptime and implement packages count instead.
 - [X] Port ppfetch-rs to AUR.

## ๐ **License**

MIT License

<p align="center">:heart: <code>Keep It Simple, Stupid.</code></p>


