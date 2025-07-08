use std::{
    error::Error,
    fs::File,
    io::{self, BufReader, Read},
};
use xml::{
    attribute::OwnedAttribute,
    name::OwnedName,
    namespace::Namespace,
    reader::{EventReader, XmlEvent},
};

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

fn read_xml_file(file_path: &str) -> Result<EventReader<impl Read>, io::Error> {
    File::open(file_path)
        .map(BufReader::new)
        .map(EventReader::new)
}

fn parse_element(name: OwnedName, attributes: Vec<OwnedAttribute>, namespace: Namespace) {
    println!("Row: Element: {name:?}, attributes: {attributes:?}, namespace: {namespace:?}");
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut feed = read_xml_file("rss.xml")?;

    let mut depth = 0;
    for event in feed {
        match event {
            Ok(XmlEvent::StartDocument {
                version,
                encoding,
                standalone,
            }) => {
                println!(
                    "Start Document: version: {version}, encoding: {encoding:?}, standalone: {standalone:?}"
                );
            }
            Ok(XmlEvent::EndDocument) => {
                println!("End Document");
            }
            Ok(XmlEvent::StartElement {
                name,
                attributes,
                namespace,
            }) => {
                parse_element(name.clone(), attributes, namespace);
                println!("{:spaces$}+{name}", "", spaces = depth * 2);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                println!("{:spaces$}-{name}", "", spaces = depth * 2);
                depth -= 1;
            }
            Err(e) => {
                eprintln!("Error reading XML: {e}");
                return Err(Box::new(e));
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
