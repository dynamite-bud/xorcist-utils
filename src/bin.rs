use anyhow::Result;
use std::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
    // spawn a new command
    // eprintln!("version 0.3.2");
    // let mut cmd = Command::new("/bin/wasmer")
    //     .arg("run")
    //     .arg("python")
    //     .spawn()
    //     .expect("failed to execute process");

    // use reqwest to get a response from the server
    let res = reqwest::get("https://httpbin.org/ip").await?;
    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Body: {}", body);
    Ok(())
}
