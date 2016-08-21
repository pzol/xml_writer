# XmlWriter

a no fluff, minimalistic, zero-copy xml writer for Rust.

## Usage

```rust
extern crate xml_writer;
use xml_writer::::XmlWriter;

let mut xml = XmlWriter::new(Vec::new()); // supply any Writer, preferrably BufferedWriter
xml.begin_elem("root");
    xml.comment("nice to see you");
    xml.begin_elem("node");
        xml.attr_esc("name", "\"123\"");
        xml.attr("id", "abc");
        xml.attr("'unescaped'", "\"123\""); // this WILL intentionally generate invalid xml
        xml.text("'text'");
    xml.end_elem();
    xml.begin_elem("stuff");
        xml.cdata("blablab");
    // xml.end_elem(); // the latter close() will close all open nodes
 // xml.end_elem();
 xml.close();
 xml.flush();

let actual = xml.into_inner();
```

## Changes
### 0.2.0
- add `XmlWriter::empty_elem` to write an empty element

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
