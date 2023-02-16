use std::fmt::Write;
use svg::{node::element::tag::Type, parser::Event};

pub fn todays_calendar(month: &str, day: &str) -> Result<String, Box<dyn std::error::Error>> {
    let icon = include_str!("./calendar.svg");
    let icon_svg = svg::read(icon)?;

    let mut target = String::new();

    for event in icon_svg {
        match event {
            Event::Text("Sol") => write!(target, "{}", month)?,
            Event::Text("34") => write!(target, "{}", day)?,
            Event::Text(t) => write!(target, "{}", t)?,
            Event::Tag(s, Type::Start, attributes) => {
                write!(target, "<{s}")?;
                for (k, v) in attributes {
                    writeln!(target, " {k}=\"{v}\"")?
                }
                write!(target, ">")?;
            }
            Event::Tag(s, Type::Empty, attributes) => {
                write!(target, "<{s}")?;
                for (k, v) in attributes {
                    writeln!(target, " {k}=\"{v}\"")?
                }
                write!(target, "/>")?;
            }
            Event::Tag(s, Type::End, _) => write!(target, "</{s}>")?,
            Event::Instruction(i) => write!(target, "{}", i)?,
            Event::Comment(c) => write!(target, "{}", c)?,
            Event::Declaration(d) => write!(target, "<!--Declaration {:?}-->", d)?,
            Event::Error(e) => write!(target, "<!--Error {:?}-->", e)?,
        }
    }
    Ok(target)
}
