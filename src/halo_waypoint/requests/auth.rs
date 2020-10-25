use http::method::Method;
use http::uri::Uri;
use http::{header, Response};
use http::{Request, StatusCode};
use hyper::Body;
use regex::Regex;
use std::convert::TryFrom;
use std::env;
use std::result::Result;

use crate::error::{Error, HaloWaypointError};

pub struct GetAuthRequest {
    login: String,
    password: String,
}

impl Default for GetAuthRequest {
    fn default() -> Self {
        let login = env::var("HALO_WAYPOINT_LOGIN")
            .expect("Environment variable not found: HALO_WAYPOINT_LOGIN");

        let password = env::var("HALO_WAYPOINT_PASSWORD")
            .expect("Environment variable not found: HALO_WAYPOINT_PASSWORD");

        Self { login, password }
    }
}

pub struct GetAuthRequestLoginFormRequest;

impl From<&GetAuthRequestLoginFormRequest> for Request<Body> {
    fn from(_req: &GetAuthRequestLoginFormRequest) -> Self {
        Request::builder()
            .method(Method::GET)
            .uri(Uri::from_static("https://login.live.com/oauth20_authorize.srf?client_id=000000004C0BD2F1&scope=xbox.basic+xbox.offline_access&response_type=code&redirect_uri=https:%2f%2fwww.halowaypoint.com%2fauth%2fcallback&locale=en-us&display=touch&state=https%253a%252f%252fwww.halowaypoint.com%252fen-us"))
            .body(Body::empty())
            .unwrap()
    }
}

#[derive(Debug)]
pub struct GetAuthRequestLoginFormResponse {
    uri: Uri,
    ppft: String,
    cookies: Vec<String>,
}

impl GetAuthRequestLoginFormResponse {
    fn regex_set_cookie_name_value() -> Regex {
        Regex::new("^([^;]+);").unwrap()
    }

    fn regex_ppft_input_value() -> Regex {
        Regex::new("<input type=\"hidden\" name=\"PPFT\" id=\"i0327\" value=\"([^\"]+)\"/>")
            .unwrap()
    }

    fn regex_url_post() -> Regex {
        Regex::new("urlPost:'([^']+)'").unwrap()
    }

    fn new(uri: Uri, ppft: String, cookies: Vec<String>) -> Self {
        Self { uri, ppft, cookies }
    }
}

impl TryFrom<Response<String>> for GetAuthRequestLoginFormResponse {
    type Error = Error;
    fn try_from(res: Response<String>) -> Result<Self, Self::Error> {
        let cookies = res
            .headers()
            .get_all(header::SET_COOKIE)
            .into_iter()
            .flat_map(|header| header.to_str())
            .flat_map(|header| Self::regex_set_cookie_name_value().captures(header))
            .flat_map(|header| header.get(1))
            .map(|header| header.as_str().to_string())
            .collect();

        let ppft = Self::regex_ppft_input_value()
            .captures(res.body())
            .and_then(|capture| capture.get(1))
            .map(|header| header.as_str().to_string());

        let uri = Self::regex_url_post()
            .captures(res.body())
            .and_then(|capture| capture.get(1))
            .and_then(|uri| Uri::try_from(uri.as_str()).ok());

        match (res.status(), ppft, uri) {
            (StatusCode::OK, Some(ppft), Some(uri)) => Ok(Self::new(uri, ppft, cookies)),
            _ => Err(HaloWaypointError::Http { response: res }.into()),
        }
    }
}

pub struct GetAuthRequestLoginRequest {
    login: String,
    password: String,
    uri: Uri,
    ppft: String,
    cookies: Vec<String>,
}

impl GetAuthRequestLoginRequest {
    pub fn new(
        login: String,
        password: String,
        uri: Uri,
        ppft: String,
        cookies: Vec<String>,
    ) -> Self {
        Self {
            login,
            password,
            uri,
            ppft,
            cookies,
        }
    }
}

impl From<(&GetAuthRequest, &GetAuthRequestLoginFormResponse)> for GetAuthRequestLoginRequest {
    fn from(req: (&GetAuthRequest, &GetAuthRequestLoginFormResponse)) -> Self {
        Self::new(
            req.0.login.clone(),
            req.0.password.clone(),
            req.1.uri.clone(),
            req.1.ppft.clone(),
            req.1.cookies.clone(),
        )
    }
}

impl From<&GetAuthRequestLoginRequest> for Request<Body> {
    fn from(req: &GetAuthRequestLoginRequest) -> Self {
        let body = format!(
            "login={}&passwd={}&PPFT={}",
            req.login, req.password, req.ppft
        );

        Request::builder()
            .method(Method::POST)
            .uri(req.uri.clone())
            .header(header::COOKIE, req.cookies.join(";"))
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(Body::from(body))
            .unwrap()
    }
}

#[derive(Debug)]
pub struct GetAuthRequestLoginResponse {
    location: Uri,
}

impl GetAuthRequestLoginResponse {
    fn new(location: Uri) -> Self {
        Self { location }
    }
}

impl TryFrom<Response<String>> for GetAuthRequestLoginResponse {
    type Error = Error;
    fn try_from(res: Response<String>) -> Result<Self, Self::Error> {
        let location = res
            .headers()
            .get(header::LOCATION)
            .and_then(|location| location.to_str().ok())
            .and_then(|location| Uri::try_from(location).ok());

        match (res.status(), location) {
            (StatusCode::FOUND, Some(location)) => Ok(Self::new(location)),
            _ => Err(HaloWaypointError::Http { response: res }.into()),
        }
    }
}

impl From<&GetAuthRequestLoginResponse> for GetAuthRequestRedirectRequest {
    fn from(req: &GetAuthRequestLoginResponse) -> Self {
        Self::new(req.location.clone())
    }
}

pub struct GetAuthRequestRedirectRequest {
    location: Uri,
}

impl GetAuthRequestRedirectRequest {
    fn new(location: Uri) -> Self {
        Self { location }
    }
}

impl From<&GetAuthRequestRedirectRequest> for Request<Body> {
    fn from(req: &GetAuthRequestRedirectRequest) -> Self {
        Request::builder()
            .method(Method::GET)
            .uri(req.location.clone())
            .body(Body::empty())
            .unwrap()
    }
}

#[derive(Debug)]
pub struct GetAuthRequestRedirectResponse {
    auth_header: String,
}

impl GetAuthRequestRedirectResponse {
    fn regex_auth_header() -> Regex {
        Regex::new("^Auth=([^;]+);").unwrap()
    }

    fn new(auth_header: String) -> Self {
        Self { auth_header }
    }
}

impl TryFrom<Response<String>> for GetAuthRequestRedirectResponse {
    type Error = Error;
    fn try_from(res: Response<String>) -> Result<Self, Self::Error> {
        let auth_header = res
            .headers()
            .get_all(header::SET_COOKIE)
            .into_iter()
            .flat_map(|header| header.to_str())
            .flat_map(|header| Self::regex_auth_header().captures(header))
            .next()
            .and_then(|header| header.get(1))
            .map(|header| header.as_str().to_string());

        match (res.status(), auth_header) {
            (StatusCode::FOUND, Some(auth_header)) => Ok(Self::new(auth_header)),
            _ => Err(HaloWaypointError::Http { response: res }.into()),
        }
    }
}

#[derive(Debug)]
pub struct GetAuthResponse {
    auth_header: String,
}

impl GetAuthResponse {
    fn new(auth_header: String) -> Self {
        Self { auth_header }
    }

    pub fn auth_header(&self) -> String {
        self.auth_header.clone()
    }
}

impl From<&GetAuthRequestRedirectResponse> for GetAuthResponse {
    fn from(req: &GetAuthRequestRedirectResponse) -> Self {
        Self::new(req.auth_header.clone())
    }
}
