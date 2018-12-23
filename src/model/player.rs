#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct PlayerToken(String);

impl PlayerToken {
    pub fn new(name: String) -> Self {
        PlayerToken(name)
    }
}

impl std::fmt::Display for PlayerToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let PlayerToken(name) = self;
        write!(f, "{}", name)
    }
}
