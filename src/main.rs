use tokio::{io::{self, AsyncReadExt, AsyncWriteExt}, fs::File};

// Working with asynchronous Reading

#[tokio::main]
async fn main() {
    let mut file = tokio::fs::File::open("example.txt").await.unwrap();
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).await.unwrap();

    println!("Read the example fiel : {:?}", buffer);

    // Write to a file asynchronously

    let mut output_file = File::create("output.txt").await.unwrap();
    let data = b"hello, tokio!";
    output_file.write_all(data).await.unwrap();
    println!("Data written to output.txt");
}