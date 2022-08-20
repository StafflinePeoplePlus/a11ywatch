pub mod apicore {
    tonic::include_proto!("apicore");
}

use std::env::var;
use tonic::transport::Channel;
use tonic::Request;

pub use apicore::{core_service_client::CoreServiceClient, CrawlParams};
use crate::EXTERNAL;
use crate::utils::Website;

/// create gRPC client from the API CORE server.
pub async fn create_client() -> Result<CoreServiceClient<Channel>, tonic::transport::Error> {
    lazy_static! {
        static ref RPC_CLIENT: String = match var(EXTERNAL) {
            Ok(_) => "https://api.a11ywatch.com:50051",
            Err(_) => "http://127.0.0.1:50051",
        }
        .to_string();
    };

    let client = CoreServiceClient::connect(RPC_CLIENT.as_ref()).await?;

    Ok(client)
}

/// run accessibility single page results.
pub async fn scan(
    url: String,
    authorization: String,
) -> Website {
    let mut client = create_client().await.unwrap();
    let page = CrawlParams {
        url,
        authorization,
        ..Default::default()
    };
    let request = Request::new(page);
    let req = client.scan(request).await.unwrap().into_inner();

    req.data.unwrap_or_default().into()
}

/// run accessibility streaming multi-page results.
pub async fn crawl(
    url: String,
    authorization: String,
    subdomains: bool,
    tld: bool
) -> Vec<Website> {
    let mut client = create_client().await.unwrap();
    let page = CrawlParams {
        url,
        authorization,
        subdomains,
        tld,
        ..Default::default()
    };
    let request = Request::new(page);
    let mut stream = client.crawl(request).await.unwrap().into_inner();

    let mut websites: Vec<Website> = Vec::new();

    while let Some(res) = stream.message().await.unwrap() {
        websites.push(res.data.unwrap_or_default().to_owned().into())
    }

    websites
}