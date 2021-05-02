pub struct Server {
    port: i32
}

impl Server {
    pub fn build(port: i32) -> Self {
        Self {
            port
        }
    }
}