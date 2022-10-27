# Getting started

Thank you for being interested in contributing to Claw! Before you can start helping, making sure to download the code first.

First, make sure that there is an [issue](https://github.com/BD103/Claw/issues) for your change. If one doesn't exist already, make it yourself! This will allow us to give you feedback before you start programming the change.

Once you've done that, [fork](https://github.com/BD103/Claw/fork) the repository and clone your personal version.

```bash
$ git clone https://github.com/USERNAME/Claw
# You can use https://github.com/BD103/Claw as well, but you won't be able to push changes.
$ cd Claw/
```

To contribute to the code, you need to have Rust installed. Claw runs on the latest stable Rust version, though Nightly is required for some tools (like Rustfmt). You can find the installer [here](https://rustup.rs).

To make sure everything is working, try building the main package.

```bash
$ cargo build
Finished dev [unoptimized + debuginfo] target(s) in 7.02s
```

Ok, you're good to go! Feel free to continue on with the rest of the documentation.
