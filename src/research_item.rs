#[derive(serde::Deserialize, Debug)]
pub struct ResearchItem {
    pub id: i32,
    pub url: String,
    pub name: String,
    pub amount: Amount,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum Amount {
    Number(i32),
    Other(String),
}

impl std::fmt::Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => n.fmt(f),
            Self::Other(o) => o.fmt(f),
        }
    }
}

impl Amount {
    pub fn as_i32(&self) -> i32 {
        match self {
            Self::Number(n) => *n,
            _ => 0,
        }
    }
}
