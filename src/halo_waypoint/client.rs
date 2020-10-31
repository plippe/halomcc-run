use async_trait::async_trait;
use http::header::HeaderValue;
use http::{header, Request, Response};
use std::convert::{TryFrom, TryInto};
use std::hash::Hash;
use std::sync::RwLock;
use std::time::Duration;
use ttl_cache::TtlCache;

use crate::chainable::Chainable;
use crate::error::Error;
use crate::halo_waypoint::requests::auth::*;
use crate::halo_waypoint::requests::service_record::*;

#[async_trait]
pub trait Client {
    async fn get_auth(&self, req: &GetAuthRequest) -> Result<GetAuthResponse, Error>;
    async fn get_service_record(
        &self,
        auth: &GetAuthResponse,
        req: &GetServiceRecordRequest,
    ) -> Result<GetServiceRecordResponse, Error>;
}

#[derive(Clone)]
pub struct HyperClient {
    client: hyper::client::Client<
        hyper_tls::HttpsConnector<hyper::client::HttpConnector>,
        hyper::body::Body,
    >,
}

impl HyperClient {
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

impl Default for HyperClient {
    fn default() -> Self {
        let https = hyper_tls::HttpsConnector::new();
        let client = hyper::Client::builder().build(https);

        HyperClient { client }
    }
}

#[async_trait]
impl Client for HyperClient {
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

pub struct InMemoryCacheClient<A: Client> {
    client: A,
    auth_cache: RwLock<TtlCache<GetAuthRequest, Result<GetAuthResponse, Error>>>,
    auth_cache_ttl: Duration,
    service_record_cache: RwLock<
        TtlCache<
            (GetAuthResponse, GetServiceRecordRequest),
            Result<GetServiceRecordResponse, Error>,
        >,
    >,
    service_record_cache_ttl: Duration,
}

impl<A: Client> InMemoryCacheClient<A> {
    async fn request<Req, Res, FutureRes, Execute, GetCache, GetCacheTtl>(
        &self,
        req: &Req,
        execute: Execute,
        get_cache: GetCache,
        get_cache_ttl: GetCacheTtl,
    ) -> Result<Res, Error>
    where
        Req: Eq + Hash + Clone,
        Res: Clone,
        FutureRes: std::future::Future<Output = Result<Res, Error>> + std::marker::Send,
        Execute: FnOnce(&Self, &Req) -> FutureRes,
        GetCache: FnOnce(&Self) -> &RwLock<TtlCache<Req, Result<Res, Error>>> + Copy,
        GetCacheTtl: FnOnce(&Self) -> &Duration,
    {
        let res = get_cache(self)
            .read()
            .unwrap()
            .get(req)
            .map(|res| res.clone());
        match res {
            Some(res) => res,
            None => {
                let res = execute(self, req).await;
                get_cache(self).write().unwrap().insert(
                    req.clone(),
                    res.clone(),
                    get_cache_ttl(self).clone(),
                );

                res
            }
        }
    }
}

impl Default for InMemoryCacheClient<HyperClient> {
    fn default() -> Self {
        let https = hyper_tls::HttpsConnector::new();
        let client = hyper::Client::builder()
            .build(https)
            .pipe(|client| HyperClient { client });

        let auth_cache = RwLock::new(TtlCache::new(10));
        let auth_cache_ttl = Duration::from_secs(14400);

        let service_record_cache = RwLock::new(TtlCache::new(1000));
        let service_record_cache_ttl = Duration::from_secs(600);

        Self {
            client,
            auth_cache,
            auth_cache_ttl,
            service_record_cache,
            service_record_cache_ttl,
        }
    }
}

#[async_trait]
impl<A: Client + Sync> Client for InMemoryCacheClient<A> {
    async fn get_auth(&self, req: &GetAuthRequest) -> Result<GetAuthResponse, Error> {
        self.request(
            req,
            // TODO: fix lifetime
            |_s, _r| self.client.get_auth(req),
            |s| &s.auth_cache,
            |s| &s.auth_cache_ttl,
        )
        .await
    }

    async fn get_service_record(
        &self,
        auth: &GetAuthResponse,
        req: &GetServiceRecordRequest,
    ) -> Result<GetServiceRecordResponse, Error> {
        self.request(
            &(auth.clone(), req.clone()),
            // TODO: fix lifetime
            |_s, (_a, _r)| self.client.get_service_record(auth, req),
            |s| &s.service_record_cache,
            |s| &s.service_record_cache_ttl,
        )
        .await
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
