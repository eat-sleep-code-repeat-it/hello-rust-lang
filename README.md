# "When nature exceeds culture, we have the rustic. When culture exceeds nature then we the pedant."

http://www.quoteambition.com/famous-confucius-quotes/


## install rustup

https://www.rust-lang.org/tools/install

```bash

// install c++ build tools for Visual Studio 2019 from Other Tools and Frameworks section

// install at least one Rust toolchain
rustup install stable-msvc

// set the default toolchain:
rustup default stable-msvc

// update
rustup self update
```
## compile and run

```bash
rustc main.rs
./main.exe
```
## build system and package manager

```bash
cargo --version
cargo new helloworld
cd helloworld

cargo new hello-rust-lang --vcs=git
cd hello-rust-lang
cargo build
./target/debug/hello-rust-lang.exe

// compile and run executable all in one command
cargo run
// to make sure code compiles with check periodically
cargo check

// release
 cargo build --release
```

## rust-by-example
https://doc.rust-lang.org/stable/rust-by-example/hello.html

## rust resources

- `Rustup doc` for documentation
- [Rust community](https://www.rust-lang.org/community)
- [User Forum](https://users.rust-lang.org/)
- [Stackoverflow Rust](https://stackoverflow.com/questions/tagged/rust)

## references

- [AWS sponsorship of the Rust project & Rust+Lambda](https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda/)
- [Microsoft to explore using Rust](https://www.zdnet.com/article/microsoft-to-explore-using-rust/)
- [Rust Runtime for AWS Lambda](https://github.com/awslabs/aws-lambda-rust-runtime)
- [Hands-On Data Structures and Algorithms with Rust](https://github.com/PacktPublishing/Hands-On-Data-Structures-and-Algorithms-with-Rust)
