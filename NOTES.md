## How to recompile proto files
Suppose you want to change the proto files and update the library accordingly:

- Follow [prost-build](https://github.com/danburkert/prost/tree/master/prost-build) installation instructions
- Look at the `build.rs` file to pick which files to build. By default we build `data.rs`, `types.rs` and `protocol.rs`
- Build this library with the `rebuild-proto` feature enabled: `cargo build --features=rebuild-proto`
- If everything goes well, the files will be added to the `src` folder
- If the modules available changed, make sure `lib.rs` exposes them