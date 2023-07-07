use std::io::{BufRead, BufReader, Read, Write};
use std::{
    fs,
    net::{TcpListener, TcpStream},
};

use super::model::*;

// Start server on localhost:7878
pub fn serve(collection: &Collection) {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, collection)
    }
}

fn handle_connection(mut stream: TcpStream, collection: &Collection) {
    let mut buf = BufReader::new(&stream);
    let mut request = String::new();
    buf.read_line(&mut request).unwrap();
    let mut response = String::new();

    match &request[..] {
        "GET / HTTP/1.1\r\n" => {
            fill_response(
                &mut response,
                "HTTP/1.1 200 OK",
                "static/index.html",
                "text/html",
            )
        },
        "POST /search HTTP/1.1\r\n" => {
            let body = get_body(&mut buf, &mut request);
            let res = collection.search(&body);
            fill_search_response(
                &mut response,
                "HTTP/1.1 200 OK",
                res,
                "application/json",
            );
        },
        "GET /index.js HTTP/1.1\r\n" => fill_response(
            &mut response,
            "HTTP/1.1 200 OK",
            "src/index.js",
            "text/javascript",
        ),
        _ => fill_response(
            &mut response,
            "HTTP/1.1 404 NOT FOUND",
            "static/404.html",
            "text/html",
        ),
    };

    stream.write_all(response.as_bytes()).unwrap();
}

fn fill_search_response(response: &mut String, status: &str, res: String, ctype: &str) {
    let len = res.len();

    *response = format!("{status}\r\nContent-Type: {ctype}\r\nContent-Length: {len}\r\n\r\n{res}");
}

fn fill_response(response: &mut String, status: &str, page: &str, ctype: &str) {
    let contents = fs::read_to_string(page).unwrap();
    let len = contents.len();

    *response =
        format!("{status}\r\nContent-Type: {ctype}\r\nContent-Length: {len}\r\n\r\n{contents}");
}

fn get_body(buf: &mut BufReader<&TcpStream>, request: &mut String) -> String {
    while buf.read_line(request).unwrap() >= 3 {}

    let content_lenght = request
        .split('\n')
        .find(|l| l.starts_with("Content-Length"))
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .parse::<usize>()
        .unwrap();

    let mut body = vec![0; content_lenght];
    buf.read_exact(&mut body).unwrap();

    String::from_utf8(body).unwrap()
}
