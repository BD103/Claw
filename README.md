# Claw

Hello, and welcome to my cool project! I've spent... at least a month on this, and it's still not finished! Feel free to check it out though. :D

## Description

Claw is a programming language that compiles into Scratch blocks. Yeah, [that Scratch](https://scratch.mit.edu). Some benefits from this include:

- Better support for Git and version control
- Static type system (UNFINISHED)
- Readable complex logic
- Easy real-time collaboration if your text editor supports it
- Helpful error messages (UNFINISHED)
- And more!

The current installation process requires you to clone the repository and build it from source. Once we start releasing versions, prebuilt binaries will be available.

```shell
$ git clone https://github.com/BD103/Claw.git
$ cd Claw
$ cargo build --release
$ cargo run --release -p claw_driver -- PATH/TO/file.claw
```

## Contributing

Thanks for wanting to take some time to contribute! I haven't yet set up a system / guide for this, but feel free to crawl through the code. (Don't shudder, it shouldn't be that bad!) I've tried my best to document all of the items in these crates, even the private ones, so people don't get lost. I've started work on the [developer wiki](https://github.com/BD103/Claw/wiki), but that is also unfinished. :)

Also, if you do decide to submit a PR, this is the legal bit:

_Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as below, without any additional terms or conditions._

## License

Claw is licensed under either:

- Apache 2.0 License ([LICENSE-APACHE](LICENSE-APACHE) or (apache.org/licenses/LICENSE-2.0)[https://www.apache.org/licenses/LICENSE-2.0])
- MIT License ([LICENSE-MIT](LICENSE-MIT) or [opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
