# docs
Search your man pages and tldr pages with fzf!

To search all the man pages on your system interactively:
```bash
docs
```
![docs demo](./demo/docs.gif)

To search all the tldr pages instead:
```bash
tldocs
```
![tldocs demo](./demo/tldocs.gif)

## Installation

### Arch

```bash
pacman -S docs
```

### MacOS

```bash
brew install fgrcl/homebrew-tap/docs
```

### Manual

First, make sure you have the following dependencies installed,
```
rustup
fzf
tldr
```

Then, install the binaries with,
```
git clone git@github.com:FGRCL/docs.git
cd docs
cargo install --path .
```
