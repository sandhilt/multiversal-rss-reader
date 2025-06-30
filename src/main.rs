use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use xml::reader::{ErrorKind, EventReader, XmlEvent};

struct Link {
    rel: Option<String>,
    url: String,
}

struct RSS {
    title: String,
    subtitle: Option<String>,
    items: Vec<Entry>,
    updated: String,
    link: Vec<Link>,
    id: String,
    icon: Option<String>,
    entry: Vec<Entry>,
}

struct Entry {
    title: String,
}

fn read_xml_file(file_path: &str) -> Result<EventReader<impl Read>, Box<dyn Error>> {
    let f = File::open(file_path)?;
    let file = BufReader::new(f);
    let parser = EventReader::new(file);
    Ok(parser)
}

fn parse_element(name: OwnedName, attributes: Vec<OwnedAttribute>, namespace: Namespace) {
    println!("Row: Element: {name:?}, attributes: {attributes:?}, namespace: {namespace:?}");
}

fn main() -> Result<(), Box<dyn Error>> {
    let feed = read_xml_file("rss.xml")?;

    let mut depth = 0;
    for event in feed {
        match event {
            Ok(XmlEvent::StartElement {
                name,
                attributes,
                namespace,
            }) => {
                parse_element(name.clone(), attributes, namespace);
                let local = name.local_name;
                // println!("{:spaces$}+{local}", "", spaces = depth * 2);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                let local = name.local_name;
                // println!("{:spaces$}-{local}", "", spaces = depth * 2);
                depth -= 1;
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
            _ => {}
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_opml() {
        unimplemented!();
    }

    #[test]
    fn test_add_rss_feed() {
        unimplemented!();
    }
}
