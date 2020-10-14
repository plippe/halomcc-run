use async_trait::async_trait;
use http::header::{HeaderValue, COOKIE};
use http::{Request, StatusCode};
use std::env;
use std::str::FromStr;

use crate::chainable::Chainable;
use crate::error::{Error, HaloWaypointError};
use crate::halo_waypoint::requests::service_record::{
    GetServiceRecordRequest, GetServiceRecordResponse,
};

#[async_trait]
pub trait Client: Sync {
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
pub struct HyperClient {
    client: hyper::client::Client<
        hyper_tls::HttpsConnector<hyper::client::HttpConnector>,
        hyper::body::Body,
    >,
    auth_header: HeaderValue,
}

impl Default for HyperClient {
    fn default() -> Self {
        let https = hyper_tls::HttpsConnector::new();
        let client = hyper::Client::builder().build(https);
        let auth_header = env::var("HALO_WAYPOINT_AUTH")
            .expect("Environment variable not found: HALO_WAYPOINT_AUTH")
            .pipe(|auth| format!("Auth={}", auth).parse())
            .expect("Invalid header value: HALO_WAYPOINT_AUTH");

        HyperClient {
            client,
            auth_header,
        }
    }
}

#[async_trait]
impl Client for HyperClient {
    async fn request<Req, Res>(&self, req: Req) -> Result<Res, Error>
    where
        Req: Into<Request<hyper::body::Body>> + Send,
        Res: FromStr<Err = Error>,
    {
        let mut req = req.into();
        req.headers_mut().append(COOKIE, self.auth_header.clone());

        let res = self.client.request(req).await?;

        let status = res.status();

        let body = hyper::body::to_bytes(res)
            .await?
            .to_vec()
            .pipe(String::from_utf8)
            .unwrap();

        match status {
            StatusCode::OK => body.parse(),
            other => Err(HaloWaypointError::Http {
                status: other.as_u16(),
                body,
            }
            .into()),
        }
    }
}

#[cfg(test)]
mod hyper_client_tests {
    use super::*;
    use crate::halo_waypoint::models::campaign_mode::CampaignMode;
    use crate::halo_waypoint::models::game::Game;

    #[tokio::test]
    #[ignore]
    async fn get_products_page() {
        let req = GetServiceRecordRequest::new(
            "John117".to_string(),
            Game::HaloCombatEvolved,
            CampaignMode::Solo,
        );

        let res = HyperClient::default().get_service_record(&req).await;
        println!("{:?}", res);

        assert!(res.is_ok());
    }
}
