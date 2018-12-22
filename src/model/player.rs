pub struct PlayerToken {
    name: String
}

impl PlayerToken {
    pub fn new(name: String) -> Self {
        PlayerToken { name: name }
    }
}
