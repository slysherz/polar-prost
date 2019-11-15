# Polar Prost

polar-prost is a library with pre-generated protobuf Rust bindings to interact with Polar watches. It uses [Prost](https://github.com/danburkert/prost) to generate the bindings.

Why pre-generated? Because it's easier to use and avoids autocompletion bugs like [this one](https://github.com/racer-rust/racer/issues/191).

_Warning_: This library has barely been tested and the definitions might not match the ones on the watch. Use at your own risk.

## Usage
*cargo.toml*
```
[dependencies]
polar-prost = "0.1.0"
```

*your_code.rs*
```
extern crate polar_prost;

use polar_prost::{encode, types, Message};

fn main() {
    let text = "Hello, World!";

    // Manually create a polar line text object
    let line = types::PbOneLineText {
        text: text.to_string()
    };

    // Encode it into a buffer
    let buffer = encode(line).unwrap();

    // Decode it again
    let new_line = types::PbOneLineText::decode(buffer).unwrap();

    // Make sure the text didn't change
    assert_eq!(text, new_line.text);
}
```

## Thanks
[@cmaion](https://github.com/cmaion) for writting a [Ruby tool](https://github.com/cmaion/polar) to interact with Polar watches. The proto files on this library are based on his.
