use http::method::Method;
use http::uri::Uri;
use http::{header, Response};
use http::{Request, StatusCode};
use hyper::Body;
use regex::Regex;
use std::convert::TryFrom;
use std::env;
use std::result::Result;

use crate::chainable::Chainable;
use crate::error::{Error, HaloWaypointError};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct GetAuthRequest {
    login: String,
    password: String,
}

impl GetAuthRequest {
    pub fn default() -> Self {
        let login = env::var("HALO_WAYPOINT_LOGIN")
            .expect("Environment variable not found: HALO_WAYPOINT_LOGIN");

        let password = env::var("HALO_WAYPOINT_PASSWORD")
            .expect("Environment variable not found: HALO_WAYPOINT_PASSWORD");

        Self { login, password }
    }
}

pub struct GetAuthRequestGetForm;

impl GetAuthRequestGetForm {
    fn to_request(&self) -> Request<Body> {
        Request::builder()
            .method(Method::GET)
            .uri(Uri::from_static("https://login.live.com/oauth20_authorize.srf?client_id=000000004C0BD2F1&scope=xbox.basic+xbox.offline_access&response_type=code&redirect_uri=https:%2f%2fwww.halowaypoint.com%2fauth%2fcallback&locale=en-us&display=touch&state=https%253a%252f%252fwww.halowaypoint.com%252fen-us"))
            .body(Body::empty())
            .unwrap()
    }
}

impl From<&GetAuthRequestGetForm> for Request<Body> {
    fn from(req: &GetAuthRequestGetForm) -> Self {
        req.to_request()
    }
}

#[derive(Debug, Clone)]
pub struct GetAuthRequestForm {
    uri: Uri,
    ppft: String,
    cookies: Vec<String>,
}

impl GetAuthRequestForm {
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

    fn try_from_response(res: Response<String>) -> Result<Self, Error> {
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
            _ => HaloWaypointError::Http(res.into_body())
                .pipe(Error::HaloWaypoint)
                .pipe(Err),
        }
    }
}

impl TryFrom<Response<String>> for GetAuthRequestForm {
    type Error = Error;
    fn try_from(res: Response<String>) -> Result<Self, Self::Error> {
        Self::try_from_response(res)
    }
}

#[derive(Debug, Clone)]
pub struct GetAuthRequestPostForm {
    req: GetAuthRequest,
    form: GetAuthRequestForm,
}

impl GetAuthRequestPostForm {
    pub fn new(req: &GetAuthRequest, form: &GetAuthRequestForm) -> Self {
        Self {
            req: req.clone(),
            form: form.clone(),
        }
    }

    pub fn to_request(&self) -> Request<Body> {
        let body = format!(
            "login={}&passwd={}&PPFT={}",
            self.req.login, self.req.password, self.form.ppft
        );

        Request::builder()
            .method(Method::POST)
            .uri(self.form.uri.clone())
            .header(header::COOKIE, self.form.cookies.join(";"))
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(Body::from(body))
            .unwrap()
    }
}

impl From<&GetAuthRequestPostForm> for Request<Body> {
    fn from(req: &GetAuthRequestPostForm) -> Self {
        req.to_request()
    }
}

#[derive(Debug, Clone)]
pub struct GetAuthRequestRedirect {
    location: Uri,
}

impl GetAuthRequestRedirect {
    fn new(location: Uri) -> Self {
        Self { location }
    }

    fn try_from_response(res: Response<String>) -> Result<Self, Error> {
        let location = res
            .headers()
            .get(header::LOCATION)
            .and_then(|location| location.to_str().ok())
            .and_then(|location| Uri::try_from(location).ok());

        match (res.status(), location) {
            (StatusCode::FOUND, Some(location)) => Ok(Self::new(location)),
            _ => HaloWaypointError::Http(res.into_body())
                .pipe(Error::HaloWaypoint)
                .pipe(Err),
        }
    }

    pub fn to_request(&self) -> Request<Body> {
        Request::builder()
            .method(Method::GET)
            .uri(self.location.clone())
            .body(Body::empty())
            .unwrap()
    }
}

impl TryFrom<Response<String>> for GetAuthRequestRedirect {
    type Error = Error;
    fn try_from(res: Response<String>) -> Result<Self, Self::Error> {
        Self::try_from_response(res)
    }
}

impl From<&GetAuthRequestRedirect> for Request<Body> {
    fn from(req: &GetAuthRequestRedirect) -> Self {
        req.to_request()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GetAuthResponse {
    auth_header: String,
}

impl GetAuthResponse {
    fn regex_auth_header() -> Regex {
        Regex::new("^(Auth=[^;]+);").unwrap()
    }

    pub fn new(auth_header: String) -> Self {
        Self { auth_header }
    }

    pub fn auth_header(&self) -> String {
        self.auth_header.clone()
    }

    fn try_from_response(res: Response<String>) -> Result<Self, Error> {
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
            _ => HaloWaypointError::Http(res.into_body())
                .pipe(Error::HaloWaypoint)
                .pipe(Err),
        }
    }
}

impl TryFrom<Response<String>> for GetAuthResponse {
    type Error = Error;
    fn try_from(res: Response<String>) -> Result<Self, Self::Error> {
        Self::try_from_response(res)
    }
}
