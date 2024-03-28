pub mod initialize;

pub struct Request {
    pub rpc: String,
    pub id: i32,
    pub method: String,
}

pub struct Response {
    pub rpc: String,
    pub id: Option<i32>,
}

pub struct Notification {
    pub rpc: String,
    pub method: String,
}
