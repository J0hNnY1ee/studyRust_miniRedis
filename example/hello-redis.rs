use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("0.0.0.0:6324").await?;
    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;
    println!("从服务器端获取到结果={:?}", result);

    Ok(())
}
