mod nodesigned;

#[tokio::main]
async fn main() {
    nodesigned::main().await.ok();
}
