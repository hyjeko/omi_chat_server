use tokio::{net::TcpListener, io::{AsyncWriteExt, BufReader, AsyncBufReadExt}, sync::broadcast};

#[tokio::main]
async fn main() { 
    //Create new TcpListener
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    println!("Starting...");

    //Create broadcast Channel
    let (tx, _rx) = broadcast::channel(10);

    loop {
        //Clone tx
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        //Await incoming connection
        let (mut socket, addr) = listener.accept().await.unwrap();
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
                //Declare two results, tokio selects whichever is first
                //result1: read in from i/o
                //result2: read in from broadcast receiver        
                tokio::select! {
                    //read in from i/o
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }

                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    //read in from broadcast receiver
                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();

                        //Only write out if you didn't write it yourself
                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
