use std::io::{self, prelude::*};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let host = "www.rustinaction.com:80";

    let mut conn = TcpStream::connect(host)?;

    conn.write_all(b"GET / HTTP/1.0")?;
    conn.write_all(b"\r\n")?;
    conn.write_all(b"Host: www.rustinaction.com")?;
    conn.write_all(b"\r\n\r\n")?; // End of HTTP message.

    // `copy` streams bytes from a Reader to a Writer.
    io::copy(&mut conn, &mut io::stdout())?;

    Ok(())
}

