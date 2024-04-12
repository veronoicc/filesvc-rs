use filesvc_rs::{http::Upload, Client};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let client = Client::default();
    println!("Uploading file");
    let (admin_url, download_url) = client.upload_web_file("examples/test.jpg", Upload::builder().build()?).await?;
    println!("Done uploading file! Admin url: {admin_url} | Download url: {download_url}");
    Ok(())
}