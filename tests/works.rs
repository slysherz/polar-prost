extern crate polar_prost;

use polar_prost::{encode, types, Message};

#[test]
fn works() {
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