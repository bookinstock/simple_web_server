use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};
use std::fs;
use std::thread;
use std::time::Duration;
use hello::ThreadPool;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(5);

    for stream in listener.incoming() {

        // #[derive(Debug)]
        // struct A;

        // println!("{:?}", A{});

        println!("{:?}", "xxx");
        let stream = stream.unwrap();
        println!("stream={:?}", stream);
        // println!("pool={}", pool);
        pool.execute(|| {
            println!("start");
            handle_connection(stream);
            println!("end");
        });
    }

    fn handle_connection(mut stream: TcpStream) {
        println!("stream={:?}", stream);
        let mut buffer = [0; 512];
        println!("?????");
        stream.read(&mut buffer).unwrap();
        println!("???");
        // println!("buffer={:?}", buffer);

        let get = b"GET / HTTP/1.1\r\n";
        let sleep = b"GET /sleep HTTP/1.1\r\n";

        let (status_line, filename) = if buffer.starts_with(get) {
            println!("a");
            let status_line = "HTTP/1.1 200 OK\r\n\r\n";
            (status_line, "hello.html")
        } else if buffer.starts_with(sleep) {
            println!("b");
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
        } else {
            println!("c");
            let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

            (status_line, "404.html")
        };
        let contents = fs::read_to_string(filename).unwrap();
        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
