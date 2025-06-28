use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

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

fn main() -> std::io::Result<()> {
    let f = File::open("rss.xml")?;
    let file = BufReader::new(f);

    let parser = EventReader::new(file);

    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                let local = name.local_name;
                println!("{:spaces$}+{local}", "", spaces = depth * 2);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                let local = name.local_name;
                depth -= 1;
                println!("{:spaces$}-{local}", "", spaces = depth * 2);
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
