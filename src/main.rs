
use std::io::Write;
use std::net::TcpListener;
use std::thread;

use mysql::*;
use mysql::prelude::*;

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap(); 
    return io::BufReader::new(file).lines(); 
}

fn main() {
	// запуск Web-сервера.
	let listener = TcpListener::bind("127.0.0.1:9123").unwrap();
    println!("listening started, ready to accept on http://127.0.0.1:9123/ ...");
    for stream in listener.incoming() {
        thread::spawn(|| {
            let mut stream = stream.unwrap();
            stream.write(b"HTTP/1.1 200 OK; Content-Type: text/html; charset=utf-8 \n\r\n\r <html></html>").unwrap();

			// откр. подкл. к БД.
			let url = "mysql://root:@localhost:3306/test";
			let pool = Pool::new(url).unwrap();
			let mut conn = pool.get_conn().unwrap();
			
			// построчное чтение шаблона.
			let lines = read_lines("select.html".to_string());
			for line in lines {
				let s = line.unwrap();
				
				if !s.contains("@tr") && !s.contains("@ver") {
					stream.write(s.as_bytes()).unwrap();
				}
				
				if s.contains("@tr") {
					println!("@tr OK");

				/*	let mut s_line: String = "<tr>".to_owned();
					conn.query_iter("SELECT TABLE_NAME AS tb FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_SCHEMA='myarttable'").unwrap().for_each(|row| {
						let r1:String = from_row(row.unwrap());
						println!("ok={}", r1);
						
						s_line.push_str("<td>");s_line.push_str(&*r1);s_line.push_str("</td>");
					});
					s_line.push_str("</tr>");
					stream.write(s_line.as_bytes()).unwrap();*/

					conn.query_iter("SELECT * FROM myarttable WHERE id>14 ORDER BY id DESC").unwrap().for_each(|row| {
						let r:(i32, String, String, String) = from_row(row.unwrap());
						println!("{}, {}, {}, {}", r.0, r.1, r.2, r.3);
							
						let mut s_line: String = "<tr>".to_owned();
						s_line.push_str("<td>");s_line.push_str(&*r.0.to_string());s_line.push_str("</td>");
						s_line.push_str("<td>");s_line.push_str(&*r.1);s_line.push_str("</td>");
						s_line.push_str("<td>");s_line.push_str(&*r.2);s_line.push_str("</td>");
						s_line.push_str("<td>");s_line.push_str(&*r.3);s_line.push_str("</td>");
						s_line.push_str("</tr>");
						stream.write(s_line.as_bytes()).unwrap();

					});
				}
				if s.contains("@ver") {
					println!("@ver OK");
					
					conn.query_iter("SELECT VERSION() AS ver").unwrap().for_each(|row| {
						let r1:String = from_row(row.unwrap());
						println!("{}", r1);
						stream.write(&*r1.as_bytes()).unwrap();

					});
				}				
			}
			println!("User is ok.");
        });
    }


}
