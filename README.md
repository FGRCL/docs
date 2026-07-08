# docs
Search your man pages and tldr pages with fzf!
 
Docs adds two brings two commands to your shell: `docs` and `tldocs`. `docs` allows your to search all the man pages on your system with fzf's fuzzy search to allow you to quickly find the documentation for the command you need. `tldocs` allows you to do the same thing but with [tldr pages](https://github.com/tldr-pages/tldr), to find quick commands instead of the entire documentation.

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
