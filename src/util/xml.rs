use std::borrow::Cow;
use std::collections::{HashMap, VecDeque};
use std::convert::TryInto;
use std::fmt::{self, Debug};
use std::str::{self, FromStr};
use std::io::{Write, Cursor, BufRead};
use log::{warn, error, info, trace};
use quick_xml::events::attributes::Attribute;
use quick_xml::events::{Event, BytesStart, BytesText, BytesEnd};
use quick_xml::name::QName;
use quick_xml::{Reader, Writer};
use super::{Result, Error};

/// A deserialized, in-memory tree-representation
/// of an XML node.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Element {
    name: String,
    content: String,
    attributes: HashMap<String, String>,
    childs: Vec<Element>
}

/// A builder that makes the construction of new
/// XML nodes more convenient.
pub struct ElementBuilder<'a> {
    name: &'a str,
    content: &'a str,
    attributes: HashMap<String, String>,
    childs: Vec<Element>
}

impl Element {
    /// Creates a new XML element builder.
    pub fn new(name: &str) -> ElementBuilder {
        ElementBuilder::new(name)
    }

    /// Deserializes an XML node tree
    /// from the given XML event reader.
    pub fn read_from<R>(reader: &mut Reader<R>) -> Result<Element> where R: BufRead {
        let mut node_stack = VecDeque::<Element>::new();
        let mut buf = Vec::new();
        
        let element = loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref start)) => {
                    trace!("Read start event");
                    let node = Element::try_from(start)?;
                    node_stack.push_back(node);
                },
                Ok(Event::Empty(ref start)) => {
                    trace!("Read empty event");
                    let node = Element::try_from(start)?;
                    if let Some(mut parent) = node_stack.pop_back() {
                        parent.childs.push(node);
                        node_stack.push_back(parent);
                    } else {
                        break Ok(node);
                    }
                },
                Ok(Event::End(ref end)) => {
                    trace!("Read end event");
                    if let Some(node) = node_stack.pop_back() {
                        if let Some(mut parent) = node_stack.pop_back() {
                            parent.childs.push(node);
                            node_stack.push_back(parent);
                        } else {
                            break Ok(node);
                        }
                    } else {
                        error!("Found closing element </{}> without an opening element before", str::from_utf8(end.name().as_ref())?);
                    }
                },
                Ok(Event::Text(ref t)) => {
                    trace!("Read text event");
                    let content = str::from_utf8(t)?.trim();
                    if !content.is_empty() {
                        if let Some(node) = node_stack.back_mut() {
                            node.content += content;
                        } else {
                            warn!("Found characters {} outside of any node", content);
                        }
                    }
                },
                Ok(Event::Eof) => break Err(Error::Eof),
                Err(e) => break Err(e.into()),
                ev => info!("Read other event: {:?}", ev),
            }
        }?;

        Ok(element)
    }
    
    /// Serializes the node to an XML string using a tree traversal.
    pub fn write_to<W>(&self, writer: &mut Writer<W>) -> Result<()> where W: Write {
        self.write_to_impl(writer)?;
        writer.get_mut().flush()?;

        Ok(())
    }

    fn write_to_impl<W>(&self, writer: &mut Writer<W>) -> Result<()> where W: Write {
        let start = BytesStart::from(self);
        
        if self.childs.is_empty() {
            // Write self-closing tag, e.g. <Element/>
            writer.write_event(Event::Empty(start))?;
        } else {
            // Write opening tag, e.g. <Element>
            writer.write_event(Event::Start(start))?;
            
            // Write text
            if !self.content.is_empty() {
                writer.write_event(Event::Text(BytesText::new(&self.content)))?;
            }

            // Write child elements
            for child in &self.childs {
                child.write_to(writer)?;
            }
            
            // Write closing tag, e.g. </Element>
            writer.write_event(Event::End(BytesEnd::new(&self.name)))?;
        }

        Ok(())
    }
    
    /// Fetches the node's tag name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    
    /// Fetches the node's textual contents.
    pub fn content(&self) -> &str {
        self.content.as_str()
    }
    
    /// Fetches an attribute's value by key.
    pub fn attribute(&self, key: &str) -> Result<&str> {
        self.attributes.get(key).map(|s| s.as_str()).ok_or_else(|| format!("No attribute with key '{}' found in <{}>!", key, self.name).into())
    }

    /// Fetches the childs.
    pub fn childs<'a>(&'a self) -> impl Iterator<Item=&'a Element> {
        self.childs.iter()
    }
    
    /// Finds the first child element with the provided tag name.
    pub fn child_by_name<'a, 'n: 'a>(&'a self, name: &'n str) -> Result<&'a Element> {
        self.childs_by_name(name).next().ok_or_else(|| format!("No <{}> found in <{}>!", name, self.name).into())
    }
    
    /// Fetches a list of all child elements matching the provided tag name.
    pub fn childs_by_name<'a, 'n: 'a>(&'a self, name: &'n str) -> impl Iterator<Item=&'a Element> + 'a {
        self.childs.iter().filter(move |c| c.name == name)
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Writes the node as XML
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        self.write_to(&mut writer).map_err(|_| fmt::Error)?;
        write!(f, "{}", str::from_utf8(&writer.into_inner().into_inner()).map_err(|_| fmt::Error)?)
    }
}

impl FromStr for Element {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Element::read_from(&mut Reader::from_str(s))
    }
}

impl<'a> ElementBuilder<'a> {
    /// Creates a new XML node builder with the
    /// specified tag name.
    pub fn new(name: &'a str) -> Self {
        Self { name: name, content: "", attributes: HashMap::new(), childs: Vec::new() }
    }
    
    /// Sets the tag name of the XML node.
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = name;
        self
    }
    
    /// Sets the contents of the XML node.
    pub fn content(mut self, data: &'a str) -> Self {
        self.content = data;
        self
    }
    
    /// Adds the specified attributes.
    pub fn attributes(mut self, attributes: impl IntoIterator<Item=(String, String)>) -> Self {
        self.attributes.extend(attributes);
        self
    }
    
    /// Adds the specified attribute.
    pub fn attribute(mut self, key: impl ToString, value: impl ToString) -> Self {
        self.attributes.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Adds the specified children.
    pub fn childs(mut self, childs: impl IntoIterator<Item=Element>) -> Self {
        self.childs.extend(childs);
        self
    }
    
    /// Adds the specified child.
    pub fn child(mut self, child: impl Into<Element>) -> Self {
        self.childs.push(child.into());
        self
    }

    /// Adds the specified child if present.
    pub fn option_child(mut self, child: Option<impl Into<Element>>) -> Self {
        if let Some(child) = child {
            self.childs.push(child.into());
        }
        self
    }
    
    /// Tries adding the specified child.
    pub fn try_child(mut self, child: impl TryInto<Element, Error=Error>) -> Result<Self> {
        self.childs.push(child.try_into()?);
        Ok(self)
    }
    
    /// Builds the XML node.
    pub fn build(self) -> Element {
        Element {
            name: self.name.to_owned(),
            content: self.content.to_owned(),
            attributes: self.attributes,
            childs: self.childs
        }
    }
}

impl<'a> Default for ElementBuilder<'a> {
    fn default() -> Self {
        Self::new("")
    }
}

impl<'a> From<ElementBuilder<'a>> for Element {
    fn from(builder: ElementBuilder<'a>) -> Self { builder.build() }
}

impl<'a> TryFrom<&BytesStart<'a>> for Element {
    type Error = Error;

    fn try_from(start: &BytesStart<'a>) -> Result<Self> {
        Ok(Element {
            name: str::from_utf8(start.name().as_ref())?.to_owned(),
            content: String::new(),
            attributes: start.attributes()
                .into_iter()
                .map(|res| {
                    let attribute = res?;
                    let key = str::from_utf8(attribute.key.as_ref())?.to_owned();
                    let value = str::from_utf8(&attribute.value)?.to_owned();
                    Ok((key, value))
                })
                .collect::<Result<HashMap<_, _>>>()?,
            childs: Vec::new()
        })
    }
}

impl<'a> From<&'a Element> for BytesStart<'a> {
    fn from(element: &'a Element) -> Self {
        BytesStart::new(&element.name)
            .with_attributes(element.attributes.iter().map(|(k, v)| Attribute {
                key: QName(k.as_bytes()),
                value: Cow::Borrowed(v.as_bytes()),
            }))
    }
}

#[cfg(test)]
mod tests {
    use super::Element;

    #[test]
    fn test_write() {
        assert_eq!("<Test/>", format!("{}", Element::new("Test").build()));
        assert_eq!("<A><B/><C/></A>", format!("{}", Element::new("A").child(Element::new("B")).child(Element::new("C")).build()))
    }

    #[test]
    fn test_read() {
        assert_eq!("<Test/>".parse::<Element>().unwrap(), Element::new("Test").build());
    }
}
