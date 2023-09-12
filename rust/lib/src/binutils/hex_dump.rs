use std::fmt;

pub fn hex_dump(title: &str, data: &[u8]) -> String {
    HexDump { title, data }.to_string()
}

#[derive(Debug, Clone, Copy)]
pub struct HexDump<'s> {
    title: &'s str,
    data: &'s [u8],
}

impl fmt::Display for HexDump<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.title)?;
        for (i, byte) in self.data.iter().enumerate() {
            if i % 16 == 0 {
                write!(f, "\n{:04X}: ", i)?;
            }
            write!(f, "{:02X} ", byte)?;
        }
        writeln!(f)
    }
}
