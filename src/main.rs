use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::{fs, thread, time};
use mylib::ThreadPool;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7978").unwrap();
    let pool = ThreadPool::new(4);
    // let mut thread_vec :Vec<thread::JoinHandle<()>> = Vec::new();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(||{
            handle_connection(stream)
        });
        // println!("连上了");
        // 创建线程
     // let handle =    thread::spawn(move || {
     //        handle_connection(stream)
     //    });
        // thread_vec.push(handle)
    }
    // for handle in thread_vec{
    //     handle.join().unwrap();
    // }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let contents = fs::read_to_string("hello.html").unwrap();
    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    let  te = time::Duration::from_millis(10000);
    thread::sleep(te);
    // println!("request:{}",String::from_utf8_lossy(&buffer[..]));
}
// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];
//     stream.read(&mut buffer).unwrap();
//
//     let contents = fs::read_to_string("hello.html").unwrap();
//
//     let response = format!(
//         "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
//         contents.len(),
//         contents
//     );
//
//     stream.write(response.as_bytes()).unwrap();
//     stream.flush().unwrap();
// }