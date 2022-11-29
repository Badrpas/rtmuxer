
pub trait StrFromVec {
    fn into_str(&self) -> String;
}

impl StrFromVec for Vec<u8> {
    fn into_str(&self) -> String {
        String::from_utf8_lossy(self).to_string()
    }
}

