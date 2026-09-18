use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub struct Request {
    pub path: String,
    pub query: HashMap<String, String>,
}

pub fn read_request(stream: &TcpStream) -> Option<Request> {
    let mut reader = BufReader::new(stream);

    let mut request_line = String::new();
    reader.read_line(&mut request_line).ok()?;

    // Skip headers until the empty line.
    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line).ok()?;
        if n == 0 || line == "\r\n" || line == "\n" {
            break;
        }
    }

    // "GET /?a=1&b=2&c=3 HTTP/1.1"
    let mut parts = request_line.split_whitespace();
    let method = parts.next()?;
    let target = parts.next()?;

    if method != "GET" {
        return None;
    }

    let (path, query_string) = target.split_once('?').unwrap_or((target, ""));

    Some(Request {
        path: path.to_string(),
        query: parse_query(query_string),
    })
}

pub fn write_response(mut stream: &TcpStream, status: &str, body: &str) {
    let response = format!(
        "HTTP/1.1 {status}\r\n\
         Content-Type: text/html; charset=utf-8\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {body}",
        body.len()
    );

    let _ = stream.write_all(response.as_bytes());
}

fn parse_query(query: &str) -> HashMap<String, String> {
    query
        .split('&')
        .filter_map(|pair| pair.split_once('='))
        .map(|(key, value)| (url_decode(key), url_decode(value)))
        .collect()
}

// "1e%2B5" -> "1e+5", "+" -> " "
fn url_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'+' => out.push(b' '),
            b'%' if i + 2 < bytes.len() => match (hex(bytes[i + 1]), hex(bytes[i + 2])) {
                (Some(high), Some(low)) => {
                    out.push(high * 16 + low);
                    i += 2;
                }
                _ => out.push(b'%'),
            },
            byte => out.push(byte),
        }
        i += 1;
    }

    String::from_utf8_lossy(&out).into_owned()
}

fn hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
