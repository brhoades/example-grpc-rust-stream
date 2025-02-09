mod rpx;

use std::env;
use tokio::sync::mpsc::channel;
use rpx::rippling_client;
use tokio_stream::wrappers::ReceiverStream;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("insufficient args, needs exactly 2 but got {}", args.len() - 1);
        return;
    }
    let arg = args.get(1).unwrap();
    let cnt = args.get(2).unwrap();
    let cnt = cnt.parse::<usize>().unwrap();

    for i in 0..cnt {
        let arg = arg.clone();
        tokio::spawn(async move {
            spawn_client(&arg).await;
        });
        if i % 50 == 0 {
            println!("spawned client #{}", i+1);
        }
        tokio::time::sleep(std::time::Duration::from_millis(2)).await;
    }

    println!("all clients spawned");
    tokio::time::sleep(std::time::Duration::from_secs(60 * 60 * 24)).await;
}

async fn spawn_client(addr: &str) {
    let mut c = rippling_client::RipplingClient::connect(addr.to_owned()).await.unwrap();

    let (tx, rx) = channel::<rpx::DeviceMessage>(1);
    let req_stream = ReceiverStream::new(rx);

    tokio::spawn(async move {
        loop {
            tx.send(rpx::DeviceMessage{}).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    });

    let mut stream = c.agent(req_stream).await.unwrap().into_inner();
    while let Some(msg) = stream.message().await.unwrap() {
        println!("got message: {msg:?}");
    }

    println!("stream exited");
}
