# XmlWriter

a no fluff, minimalistic xml writer for Rust.

## Usage


```rust
extern crate xml_writer;
use xml_writer::::XmlWriter;

let mut xml = XmlWriter::new(Vec::new());
xml.begin_elem("OTDS");
    xml.comment("nice to see you");
    xml.begin_elem("node");
        xml.attr_esc("name", "\"123\"");
        xml.attr("id", "abc");
        xml.attr("'unescaped'", "\"123\""); // this WILL generate invalid xml
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
