use serde::{Serialize, Serializer};

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Permission {
    Domains,
    URLs,
}

impl Serialize for Permission {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.as_ref())
    }
}

impl AsRef<str> for Permission {
    fn as_ref(&self) -> &str {
        match self {
            Permission::Domains => "domains",
            Permission::URLs => "urls",
        }
    }
}

impl ToString for Permission {
    fn to_string(&self) -> String {
        self.as_ref().to_string()
    }
}