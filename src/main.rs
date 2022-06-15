use tokio::{net::TcpListener, io::{AsyncWriteExt, BufReader, AsyncBufReadExt}};

#[tokio::main]
async fn main() { 
    //Create new TcpListener
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    println!("Starting...");
    loop {
        //Await incoming connection
        let (mut socket, _addr) = listener.accept().await.unwrap();
        println!("Incoming Connection...");
        //Spawn new async task
        tokio::spawn(async move {
            println!("Spawning...");
            //Deconstruct Reader and Writer from socket.split()
            let (reader, mut writer) = socket.split();
            //Create memory buffer to read/store the socket data
            let mut line = String::new();
            //Create mutable BufReader helper
            let mut reader = BufReader::new(reader);
        
            //Read and write all the lines
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
        });
    }
}
