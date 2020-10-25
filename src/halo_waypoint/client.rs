use async_trait::async_trait;
use http::header::HeaderValue;
use http::{header, Request, Response};
use std::convert::TryFrom;
use std::convert::TryInto;

use crate::chainable::Chainable;
use crate::error::Error;
use crate::halo_waypoint::requests::auth::*;
use crate::halo_waypoint::requests::service_record::*;

#[async_trait]
pub trait Client: Sync {
    async fn request<Req, Res>(&self, req: Req) -> Result<Res, Error>
    where
        Req: Into<Request<hyper::Body>> + Send,
        Res: TryFrom<Response<String>, Error = Error>;

    async fn get_auth(&self, req: &GetAuthRequest) -> Result<GetAuthResponse, Error> {
        let res = self.request(&GetAuthRequestLoginFormRequest).await?;
        let req = GetAuthRequestLoginRequest::from((req, &res));
        let res = self.request(&req).await?;
        let req = GetAuthRequestRedirectRequest::from(&res);
        let res = self.request(&req).await?;

        Ok(GetAuthResponse::from(&res))
    }

    async fn get_service_record(
        &self,
        auth: &GetAuthResponse,
        req: &GetServiceRecordRequest,
    ) -> Result<GetServiceRecordResponse, Error> {
        let req = GetServiceRecordRequestAuthenticated::from((auth, req));
        self.request(&req).await
    }
}

#[derive(Clone)]
pub struct HyperClient {
    client: hyper::client::Client<
        hyper_tls::HttpsConnector<hyper::client::HttpConnector>,
        hyper::body::Body,
    >,
}

impl Default for HyperClient {
    fn default() -> Self {
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
        Res: TryFrom<Response<String>, Error = Error>,
    {
        let mut req = req.into();
        req.headers_mut().append(
            header::USER_AGENT,
            HeaderValue::from_static("halomcc.run/0.1"),
        );

        let res = self.client.request(req).await?;
        let res_without_body = res
            .headers()
            .into_iter()
            .fold(Response::builder(), |res, (name, value)| {
                res.header(name, value)
            })
            .status(res.status());

        let body = hyper::body::to_bytes(res)
            .await?
            .to_vec()
            .pipe(String::from_utf8)
            .unwrap();

        res_without_body.body(body).unwrap().try_into()
    }
}

#[cfg(test)]
mod hyper_client_tests {
    use super::*;
    use crate::halo_waypoint::models::campaign_mode::CampaignMode;
    use crate::halo_waypoint::models::game::Game;

    #[tokio::test]
    #[ignore]
    async fn get_auth() {
        let req = GetAuthRequest::default();
        let res = HyperClient::default().get_auth(&req).await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn get_service_record() {
        let req = GetAuthRequest::default();
        let auth = HyperClient::default().get_auth(&req).await.unwrap();

        let req = GetServiceRecordRequest::new(
            "John117".to_string(),
            Game::HaloCombatEvolved,
            CampaignMode::Solo,
        );

        let res = HyperClient::default().get_service_record(&auth, &req).await;

        assert!(res.is_ok());
    }
}
