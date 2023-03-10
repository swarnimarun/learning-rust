pub struct Str<'a> {
    data: &'a [u8],
}

impl Str<'_> {
    pub fn get(&self, idx: usize) -> Option<u8> {
        self.data.get(idx).copied()
    }
    pub fn split_at(&self, idx: usize) -> Option<(&[u8], &[u8])> {
        if self.data.len() > idx {
            Some((&self.data[..idx], &self.data[idx..]))
        } else {
            None
        }
    }
}

fn main() {
    let s = Str{ data: b"this is a string" };
    if let Some((x, y)) = s.split_at(5) {
        println!("{}", String::from_utf8_lossy(x));
        println!("{}", String::from_utf8_lossy(y));
    }
}
