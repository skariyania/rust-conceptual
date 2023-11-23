use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
fn request_logger(buf_reader: BufReader<&mut TcpStream>) {
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:#?}", http_request);
}
fn handle_connection(mut stream: TcpStream) {
    //reading request data
    let buf_reader = BufReader::new(&mut stream);
    // request_logger(buf_reader);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, file_name) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 200 OK", "404.json"),
    };
    let file_path = String::from("web_server/static/");
    let contents = fs::read_to_string(file_path + file_name).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-lenth: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("starting server :8080");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let _ = stream.set_nonblocking(true);
        let _ = stream.set_nodelay(true);

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}
