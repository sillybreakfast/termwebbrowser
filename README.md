# termwebbrowser

termwebbrowser is another dumb project by me. this program accesses the termweb and displays it in a standard web browser format, all from the terminal.

## what is the termweb?

the termweb is a fancy name for [this repository](https://github.com/sillybreakfast/termwebsites), which is the repository for all websites accessed by termwebbrowser.

## installation

to install from source, you need to make sure rust is installed. [download rustup](https://sh.rustup.rs/)
```shell
rustup --version
```

first, clone the repository from github:
```shell
git clone https://github.com/sillybreakfast/termwebbrowser.git
cd termwebbrowser
```

then, compile the code into an executable:
```shell
cargo build --release
mv ./target/release/termwebbrowser .
chmod u+x ./termwebbrowser
```

(optional) to make it easy to run, move the executable to `~/.local/bin`:
```shell
mv ./termwebbrowser ~/.local/bin/
```