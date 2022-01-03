use std::fmt::Display;
struct Client {
    username:String,
    password:String,
}

impl Client {
    pub fn new(username:String, password:String) -> Self {
        Client{
            username:username,
            password:password
        }
    }
}

impl Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Username : {}\n Password : {}\n",self.username, self.password)
    }
}