use std::io::{ self, Write };
use std::fmt;

pub type Result = io::Result<()>;

/// The XmlWriter himself
pub struct XmlWriter<'a, W: Write> {
    stack: Vec<&'a str>,
    writer: Box<W>,
    opened: bool,
    /// if `true` it will indent all opening elements
    pub pretty: bool
}

impl<'a, W: Write> fmt::Debug for XmlWriter<'a, W> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(try!(write!(f, "XmlWriter {{ stack: {:?}, opened: {} }}", self.stack, self.opened)))
    }
}

impl<'a, W: Write> XmlWriter<'a, W> {
    /// Create a new writer, by passing an `io::Write`
    pub fn new(writer: W) -> XmlWriter<'a, W>{
        XmlWriter { stack: Vec::new(), writer: Box::new(writer), opened: false, pretty: true }
    }

    /// Write the DTD
    pub fn dtd(&mut self, encoding: &str) -> Result {
        try!(self.write("<?xml version=\"1.0\" encoding=\""));
        try!(self.write(encoding));
        self.write("\" ?>\n")
    }

    fn indent(&mut self) -> Result {
        if self.pretty {
            if self.stack.len() > 0 {
                try!(self.write("\n"));
                let indent = self.stack.len() * 2;
                for _ in 0..indent { try!(self.write(" ")); };
            }
        }
        Ok(())
    }

    /// Write a self-closing element like <br/>
    pub fn elem(&mut self, name: &str) -> Result {
        try!(self.close_elem());
        try!(self.indent());
        try!(self.write("<"));
        try!(self.write(name));
        self.write("/>")
    }

    /// Write an element with inlined text (escaped)
    pub fn elem_text(&mut self, name: &str, text: &str) -> Result {
        try!(self.close_elem());
        try!(self.indent());
        try!(self.write("<"));
        try!(self.write(name));
        try!(self.write(">"));

        try!(self.escape(text, false));

        try!(self.write("</"));
        try!(self.write(name));
        self.write(">")
    }

    /// Begin an elem, make sure name contains only allowed chars
    pub fn begin_elem(&mut self, name: &'a str) -> Result {
        try!(self.close_elem());
        try!(self.indent());
        self.stack.push(name);
        try!(self.write("<"));
        self.opened = true;
        // stderr().write_fmt(format_args!("\nbegin {}", name));
        self.write(name)
    }

    /// Close an elem if open, do nothing otherwise
    fn close_elem(&mut self) -> Result {
        if self.opened {
            if self.pretty {
                try!(self.write(">"));
            } else {
                try!(self.write(">"));
            }
            self.opened = false;
        }
        Ok(())
    }

    /// End and elem
    pub fn end_elem(&mut self) -> Result {
        try!(self.close_elem());
        match self.stack.pop() {
            Some(name) => {
                try!(self.write("</"));
                try!(self.write(name));
                if self.pretty {
                    try!(self.write(">"));
                } else {
                    try!(self.write(">"));
                }
                Ok(())
            },
            None => panic!("Attempted to close and elem, when none was open, stack {:?}", self.stack)
        }
    }

    /// Begin an empty elem
    pub fn empty_elem(&mut self, name: &'a str) -> Result {
        try!(self.close_elem());
        try!(self.indent());
        try!(self.write("<"));
        try!(self.write(name));
        self.write("/>")
    }

    /// Write an attr, make sure name and value contain only allowed chars.
    /// For an escaping version use `attr_esc`
    pub fn attr(&mut self, name: &str, value: &str) -> Result {
        if !self.opened {
            panic!("Attempted to write attr to elem, when no elem was opened, stack {:?}", self.stack);
        }
        try!(self.write(" "));
        try!(self.write(name));
        try!(self.write("=\""));
        try!(self.write(value));
        self.write("\"")
    }

    /// Write an attr, make sure name contains only allowed chars
    pub fn attr_esc(&mut self, name: &str, value: &str) -> Result {
        if !self.opened {
            panic!("Attempted to write attr to elem, when no elem was opened, stack {:?}", self.stack);
        }
        try!(self.write(" "));
        try!(self.escape(name, true));
        try!(self.write("=\""));
        try!(self.escape(value, false));
        self.write("\"")
    }

    /// Escape identifiers or text
    fn escape(&mut self, text: &str, ident: bool) -> Result {
        for c in text.chars() {
            match c {
                '"'  => try!(self.write("&quot;")),
                '\'' => try!(self.write("&apos;")),
                '&'  => try!(self.write("&amp;")),
                '<'  => try!(self.write("&lt;")),
                '>'  => try!(self.write("&gt;")),
                '\\' if ident => try!(self.write("\\\\")),
                _    => try!(self.write_slice(c.encode_utf8(&mut [0;4]).as_bytes()))
                   // if let Some(len) =  {
                   //      try!(self.writer.write(&self.utf8[0..len])); ()
                   //  } else {
                   //      try!(; ()
                   //  }
            };
        }
        Ok(())
    }

    /// Write a text, escapes the text automatically
    pub fn text(&mut self, text: &str) -> Result {
        try!(self.close_elem());
        self.escape(text, false)
    }

    /// Raw write, no escaping, no safety net, use at own risk
    pub fn write(&mut self, text: &str) -> Result {
        try!(self.writer.write(text.as_bytes()));
        Ok(())
    }

    /// Raw write, no escaping, no safety net, use at own risk
    fn write_slice(&mut self, slice: &[u8]) -> Result {
        try!(self.writer.write(slice));
        Ok(())
    }

    /// Write a CDATA
    pub fn cdata(&mut self, cdata: &str) -> Result {
        try!(self.close_elem());
        try!(self.write("<![CDATA["));
        try!(self.write(cdata));
        self.write("]]>")
    }

    /// Write a comment
    pub fn comment(&mut self, comment: &str) -> Result {
        try!(self.close_elem());
        try!(self.indent());
        try!(self.write("<!-- "));
        try!(self.escape(comment, false));
        self.write(" -->")
    }

    /// Close all open elems
    pub fn close(&mut self) -> Result {
        for _ in 0..self.stack.len() {
            try!(self.end_elem());
        }
        Ok(())
    }

    /// Flush the underlying Writer
    pub fn flush(&mut self) -> Result {
        self.writer.flush()
    }

    /// Consume the XmlWriter and return the inner Writer
    pub fn into_inner(self) -> W {
        *self.writer
    }
}


#[allow(unused_must_use)]
#[cfg(test)]
mod tests {
    use super::XmlWriter;
    use std::str;

    #[test]
    fn integration() {
        let mut xml = XmlWriter::new(Vec::new());
        xml.begin_elem("OTDS");
            xml.comment("nice to see you");
            xml.empty_elem("success");
            xml.begin_elem("node");
                xml.attr_esc("name", "\"123\"");
                xml.attr("id", "abc");
                xml.attr("'unescaped'", "\"123\""); // this WILL generate invalid xml
                xml.text("'text'");
            xml.end_elem();
            xml.begin_elem("stuff");
                xml.cdata("blablab");
            // xml.end_elem();
         // xml.end_elem();
         xml.close();
         xml.flush();

         let actual = xml.into_inner();
         assert_eq!(str::from_utf8(&actual).unwrap(), "<OTDS>\n  <!-- nice to see you -->\n  <success/>\n  <node name=\"&quot;123&quot;\" id=\"abc\" \'unescaped\'=\"\"123\"\">&apos;text&apos;</node>\n  <stuff><![CDATA[blablab]]></stuff></OTDS>");
    }

    #[test]
    fn comment() {
        let mut xml = XmlWriter::new(Vec::new());
        xml.comment("comment");

        let actual = xml.into_inner();
        assert_eq!(str::from_utf8(&actual).unwrap(), "<!-- comment -->");
    }
}
