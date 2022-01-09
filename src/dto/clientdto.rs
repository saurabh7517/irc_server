use std::fmt::Display;
pub struct ClientDto {
    pub username:String,
    pub password: String,
}

impl Display for ClientDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Username : {}\n Password : {}\n",self.username, self.password)
    }
}