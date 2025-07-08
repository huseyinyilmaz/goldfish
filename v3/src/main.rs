use goldfish::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut env = env_logger::Env::new();
    env = env.filter_or("GOLDFISH_LOG_LEVEL", "info");
    env_logger::Builder::from_env(env).try_init().unwrap();

    let result = run().await;
    result
}
