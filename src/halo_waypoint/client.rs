use async_trait::async_trait;
use http::{Request, StatusCode};
use std::str::FromStr;

use crate::error::Error;
use crate::halo_waypoint::requests::{GetServiceRecordRequest, GetServiceRecordResponse};
use crate::utils::Chainable;

#[async_trait]
trait Client: Sync {
    async fn request<Req, Res>(&self, req: Req) -> Result<Res, Error>
    where
        Req: Into<Request<hyper::Body>> + Send,
        Res: FromStr<Err = Error>;

    async fn get_service_record(
        &self,
        req: &GetServiceRecordRequest,
    ) -> Result<GetServiceRecordResponse, Error> {
        self.request(req).await
    }
}

#[derive(Clone)]
struct HyperClient {
    client: hyper::client::Client<
        hyper_tls::HttpsConnector<hyper::client::HttpConnector>,
        hyper::body::Body,
    >,
}

impl Default for HyperClient {
    fn default() -> HyperClient {
        let https = hyper_tls::HttpsConnector::new();
        let client = hyper::Client::builder().build(https);
        HyperClient { client }
    }
}

#[async_trait]
impl Client for HyperClient {
    async fn request<Req, Res>(&self, req: Req) -> Result<Res, Error>
    where
        Req: Into<Request<hyper::body::Body>> + Send,
        Res: FromStr<Err = Error>,
    {
        let res = self.client.request(req.into()).await?;

        let status = res.status();

        let body = hyper::body::to_bytes(res)
            .await?
            .to_vec()
            .pipe(String::from_utf8)
            .unwrap();

        match status {
            StatusCode::OK => body.parse(),
            other => Err(Error::HaloWaypointStatus {
                status: other.as_u16(),
                body,
            }),
        }
    }
}

#[cfg(test)]
mod hyper_client_tests {
    use super::*;
    use crate::halo_waypoint::models::{CampaignMode, Game};
    use crate::halo_waypoint::requests::GetServiceRecordRequest;

    #[tokio::test]
    #[ignore]
    async fn get_products_page() {
        let req = GetServiceRecordRequest::new(
            "".to_string(),
            "John117".to_string(),
            Game::HaloCombatEvolved,
            CampaignMode::Solo,
        );

        let res = HyperClient::default().get_service_record(&req).await;
        println!("{:?}", res);

        assert!(res.is_ok());
    }
}
