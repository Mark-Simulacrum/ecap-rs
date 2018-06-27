#[derive(Debug, Copy, Clone, Eq)]
pub struct Version {
    pub major: Option<u32>,
    pub minor: Option<u32>,
    pub micro: Option<u32>,
}

impl Version {
    pub fn known(&self) -> bool {
        self.major.is_some()
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.known()
            && self.major == other.major
            && self.minor == other.minor
            && self.micro == other.micro
    }
}
