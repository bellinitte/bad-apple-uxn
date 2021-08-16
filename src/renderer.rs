use std::fmt::Write;
use textwrap::indent;

#[derive(Clone, Copy)]
pub enum Value {
    Byte(u8),
    Short(u16),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Byte(value) => write!(f, "{}", &format!("{:#04x}", value)[2..]),
            Self::Short(value) => write!(f, "{}", &format!("{:#06x}", value)[2..]),
        }
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::Byte(value)
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::Short(value)
    }
}

pub fn render_uxntal(data: Vec<Vec<u8>>, tweakables: Vec<(&'static str, Value)>) -> String {
    let mut source = String::new();

    write!(
        &mut source,
        "(\n{}\n)\n\n(\n  {}\n  v{}\n  {}\n)\n\n( Tweakables )\n\n",
        indent(include_str!("../LICENSE").trim(), "  "),
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_REPOSITORY"),
    )
    .unwrap();

    let tweakable_pad: usize = tweakables
        .iter()
        .max_by_key(|(name, _)| name.len())
        .unwrap()
        .0
        .len();
    for (name, value) in tweakables {
        write!(
            &mut source,
            "%{:pad$} {{ #{} }}\n",
            name,
            value,
            pad = tweakable_pad
        )
        .unwrap();
    }

    write!(&mut source, "\n{}\n@data\n", include_str!("main.tal")).unwrap();

    let mut line_separator = "";
    for frame in data {
        write!(&mut source, "{}", line_separator).unwrap();
        line_separator = "\n";
        let frame: &[u8] = frame.as_ref();
        for chunk in frame.chunks(16) {
            write!(&mut source, " ").unwrap();
            for byte in chunk {
                write!(&mut source, " {}", &format!("{:#04x}", byte)[2..]).unwrap();
            }
            write!(&mut source, "\n").unwrap();
        }
    }

    source
}
