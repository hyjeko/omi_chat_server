use tokio::{net::TcpListener, io::{/*AsyncReadExt, */ AsyncWriteExt, BufReader, AsyncBufReadExt}};

//create tcp echo server
#[tokio::main]
async fn main() { 
    //Create TcpListener with addr; await and unwrap to get the value
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    //Tcp connection accept exposes socket: TcpStream and addr: SocketAddr
    let (mut socket, _addr) = listener.accept().await.unwrap();

    //Use Reader
    let (reader, mut writer) = socket.split();

    //Create memory buffer to read/store the socket data
    let mut reader = BufReader::new(reader);
    //need string for each line
    let mut line = String::new();

    loop {        
    
        //Read and return count of how many bytes we read
        let bytes_read = reader.read_line(&mut line).await.unwrap();
        if bytes_read == 0 {
            break;
        }
    
        //Write all of bytes from the read in buffer back to the socket
        writer.write_all(line.as_bytes()).await.unwrap();
        line.clear();
    }

}
