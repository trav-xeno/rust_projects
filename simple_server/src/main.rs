fn main() {
    let ip_port = String::from("127.0.0.1:8080"); //Server::new("127.0.0.1:8080".to_string())
    let server = Server::new(ip_port);
    server.run();
}

struct Server {
    addr: String, 
}
//implement struct
impl Server{
    fn new(addr: String ) -> Server {
        //let port = addr[length-4..];//have to reference so not to have barrow this also does bytes not char ending!
        Server{
            //can do this as well addr only
            addr:addr,
        }
    }
    fn run(self) {
        println!("Server is running! Listening on:{}",self.addr);
    }
}
