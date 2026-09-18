mod http;

use http::{read_request, write_response};
use quadratic_rust::equation::{QuadraticEquation, Solution};
use quadratic_rust::solver;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::thread;

const PAGE: &str = include_str!("../static/index.html");

fn main() {
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    let listener = TcpListener::bind(&addr).expect("Unable to bind the address.");
    println!("Listening on http://{addr}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_connection(&stream));
            }
            Err(e) => eprintln!("Connection failed: {e}"),
        }
    }
}

fn handle_connection(stream: &TcpStream) {
    let Some(request) = read_request(stream) else {
        write_response(stream, "400 Bad Request", "Bad Request");
        return;
    };

    match request.path.as_str() {
        "/" => write_response(stream, "200 OK", &render_page(&request.query)),
        _ => write_response(stream, "404 Not Found", "Not Found"),
    }
}

fn render_page(query: &HashMap<String, String>) -> String {
    let a = parse_number(query, "a");
    let b = parse_number(query, "b");
    let c = parse_number(query, "c");

    let result = if query.is_empty() {
        String::new()
    } else {
        match (a, b, c) {
            (Some(a), Some(b), Some(c)) => solve_to_html(a, b, c),
            _ => r#"<p class="error">Введите корректные числа.</p>"#.to_string(),
        }
    };

    PAGE.replace("{{a}}", &show(a))
        .replace("{{b}}", &show(b))
        .replace("{{c}}", &show(c))
        .replace("{{result}}", &result)
}

fn parse_number(query: &HashMap<String, String>, key: &str) -> Option<f64> {
    query
        .get(key)?
        .trim()
        .parse::<f64>()
        .ok()
        .filter(|n| n.is_finite())
}

fn show(n: Option<f64>) -> String {
    n.map(|n| n.to_string()).unwrap_or_default()
}

fn solve_to_html(a: f64, b: f64, c: f64) -> String {
    if a == 0.0 {
        return r#"<p class="error">Коэффициент a не может быть равен 0.</p>"#.to_string();
    }

    let eq = QuadraticEquation::new(a, b, c);

    match solver::solve(&eq) {
        Solution::TwoRoots(x1, x2) => format!("<p>Два корня: x₁ = {x1}, x₂ = {x2}</p>"),
        Solution::OneRoot(x) => format!("<p>Один корень: x = {x}</p>"),
        Solution::NoRealRoots => "<p>Действительных корней нет</p>".to_string(),
    }
}
