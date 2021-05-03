# Cloud Native Rust Day workshop 

Coded Along [Florian Gilcher](https://github.com/skade)
### Steps:

- Install rustup and optionally rust-analyser.
- move to /tcp-mailbox
- Run the project ```cargo run```(dunno for sure)
- To access the tcp endpoint, type ```curl -vv telnet://127.0.0.1:7878``` in other terminal.

### Resources

- [Rust belt](https://plv.mpi-sws.org/rustbelt/)
- [RustBelt: Securing the Foundations of the Rust Programming Language](https://dl.acm.org/doi/pdf/10.1145/3158154)
- [Code written by author](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=0a50e08ed776b1a01de0838fd925e210)
- [Similar code](https://github.com/skade/mailbox)

### Future

- Checkout Tokio API and see how the same functionality works there.
- Ownership is the most important thing in rust, check it out more.
- https://doc.rust-lang.org/std/sync/struct.RwLock.html Similar to Mutex ig?