use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}};

//create tcp echo server
#[tokio::main]
async fn main() { 
    //Create TcpListener with addr; await and unwrap to get the value
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    //Tcp connection accept exposes socket: TcpStream and addr: SocketAddr
    let (mut socket, _addr) = listener.accept().await.unwrap();

    loop {        
        //Create memory buffer to read/store the socket data
        let mut buffer = [0u8; 1024];
    
        //Read and return count of how many bytes we read
        let bytes_read = socket.read(&mut buffer).await.unwrap();
        
        //Write all of bytes from the read in buffer back to the socket
        socket.write_all(&buffer[..bytes_read]).await.unwrap();
    }

}
