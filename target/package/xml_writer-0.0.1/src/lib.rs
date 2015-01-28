#![allow(unstable)]
#![allow(unused_must_use)]
use std::io::{ Writer, IoResult };
use std::fmt;

pub type Result = IoResult<()>;

pub struct XmlWriter<'a, W: Writer> {
    stack: Vec<&'a str>,
    writer: Box<W>,
    opened: bool
}

impl<'a, W: Writer> fmt::Debug for XmlWriter<'a, W> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(try!(write!(f, "XmlWriter {{ stack: {:?}, opened: {} }}", self.stack, self.opened)))
    }
}

impl<'a, W: Writer> XmlWriter<'a, W> {
    pub fn new(writer: W) -> XmlWriter<'a, W>{
        XmlWriter { stack: Vec::new(), writer: Box::new(writer), opened: false }
    }

    /// Begin an elem, make sure name contains only allowed chars
    pub fn begin_elem(&mut self, name: &'a str) -> Result {
        self.stack.push(name);
        try!(self.close_elem());
        try!(self.write("<"));
        try!(self.write(name));
        self.opened = true;
        Ok(())
    }

    /// Close an elem if open, do nothing otherwise
    fn close_elem(&mut self) -> Result {
        if self.opened {
            try!(self.write(">"));
            self.opened = false;
        }
        Ok(())
    }


    /// End and elem
    pub fn end_elem(&mut self) -> Result {
        match self.stack.pop() {
            Some(name) => {
                try!(self.close_elem());
                try!(self.write("</"));
                try!(self.write(name));
                try!(self.write(">"));
                Ok(())
            },
            None => panic!("Attempted to close and elem, when no elem was opened")
        }
    }

    /// Write an attr, make sure name and value contain only allowed chars.
    /// For an escaping version use `attr_esc`
    pub fn attr(&mut self, name: &str, value: &str) -> Result {
        if !self.opened {
            panic!("Attempted to write attr to elem, when no elem was opened");
        }
        try!(self.write(" "));
        try!(self.write(name));
        try!(self.write("=\""));
        try!(self.write(value));
        try!(self.write("\""));
        Ok(())
    }

    /// Write an attr, make sure name contains only allowed chars
    pub fn attr_esc(&mut self, name: &str, value: &str) -> Result {
        if !self.opened {
            panic!("Attempted to write attr to elem, when no elem was opened");
        }
        try!(self.write(" "));
        try!(self.escape(name, true));
        try!(self.write("=\""));
        try!(self.escape(value, false));
        try!(self.write("\""));
        Ok(())
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
                _    => try!(self.writer.write_u8(c as u8))
            }
        }
        Ok(())
    }

    /// Write a text, escapes the text automatically
    pub fn text(&mut self, text: &str) -> Result {
        self.close_elem();
        self.escape(text, false)
    }

    pub fn write(&mut self, text: &str) -> Result {
        Ok(try!(self.writer.write_str(text)))
    }

    /// Write a CDATA
    pub fn cdata(&mut self, cdata: &str) -> Result {
        try!(self.close_elem());
        try!(self.write("<![CDATA["));
        try!(self.write(cdata));
        try!(self.write("]]>"));
        Ok(())
    }

    /// Write a comment
    pub fn comment(&mut self, comment: &str) -> Result {
        try!(self.close_elem());
        try!(self.write("<!-- "));
        try!(self.escape(comment, false));
        try!(self.write(" -->"));
        Ok(())
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


#[cfg(test)]
mod tests {
    use super::XmlWriter;
    use std::str;

    #[test]
    fn integration() {
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
            // xml.end_elem();
         // xml.end_elem();
         xml.close();
         xml.flush();

         let actual = xml.into_inner();
         assert_eq!(str::from_utf8(actual.as_slice()).unwrap(), "<OTDS><!-- nice to see you --><node name=\"&quot;123&quot;\" id=\"abc\" \'unescaped\'=\"\"123\"\">&apos;text&apos;</node><stuff><![CDATA[blablab]]></stuff></OTDS>");
    }

    #[test]
    fn comment() {
        let mut xml = XmlWriter::new(Vec::new());
        xml.comment("comment");

        let actual = xml.into_inner();
        assert_eq!(str::from_utf8(actual.as_slice()).unwrap(), "<!-- comment -->");
    }
}
