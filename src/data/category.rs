use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DomainCategory {
    Phishing,
    Safe,
    Malware,
    Unknown,
}

impl Default for DomainCategory {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Serialize for DomainCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.as_ref())
    }
}

impl AsRef<str> for DomainCategory {
    fn as_ref(&self) -> &str {
        match self {
            DomainCategory::Phishing => "phishing",
            DomainCategory::Safe => "safe",
            DomainCategory::Malware => "malware",
            DomainCategory::Unknown => "unknown",
        }
    }
}

impl ToString for DomainCategory {
    fn to_string(&self) -> String {
        self.as_ref().to_string()
    }
}

impl<'de> Deserialize<'de> for DomainCategory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let data = String::deserialize(deserializer)?;
        match data.as_str() {
            "malware" => Ok(Self::Malware),
            "safe" => Ok(Self::Safe),
            "phishing" => Ok(Self::Malware),
            _ => Ok(Self::Unknown),
        }
    }
}