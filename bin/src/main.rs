#![allow(dead_code, unused_imports)]
use std::net::{TcpListener,TcpStream};
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::time::Duration;
use mythread::threadpool;

fn main() {
    // ① 绑定IP 和 端口
    // bind 函数返回类型是 Result<T, E>，意味着绑定操作可能发生失败。
    // 如
    // 绑定小于等于1024的端口，需要管理员权限。
    // 端口已被绑定。
    let listener = TcpListener::bind("127.0.0.1:9998").unwrap();

    let pool = threadpool::ThreadPool::new(4);
    
    for stream in listener.incoming(){
        // ③ unwrap 出现任何错误结束程序
        let stream = stream.unwrap();

        pool.execute(||{
            handle_connection(stream);
        });
        // thread::spawn(|| {
        //     handle_connection(stream);
        // });
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer:[u8; 1024] = [0;1024];
    stream.read(&mut buffer).unwrap();
    let normal = b"GET / HTTP/1.1";
    let get = b"GET /hello.html HTTP/1.1";
    let sleep = b"GET /sleep HTTP/1.1";

    println!("Request:\n{}",String::from_utf8_lossy(&buffer[..]));


    let (status_line,filename)=if buffer.starts_with(normal){
        ("HTTP/1.1 200 OK\r\n","public/learn.html")
    }else if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n","public/hello.html")
    }else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n", "public/hello.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND\r\n", "public/hello.html")
    };

    let contens = fs::read_to_string(filename).unwrap();
    let response = format!("{}Content-Length: {}\r\n\r\n{}", status_line, contens.len(), contens);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("deal!");
}