use std::{
    collections::HashMap,
    future::{ready, Ready},
};

use actix::prelude::*;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use chrono::prelude::*;
use futures_util::future::LocalBoxFuture;

#[derive(Clone)]
pub struct RateLimiterStorage {
    pub clients: HashMap<String, (i32, NaiveDateTime)>,
    pub maxrpm: i32,
}

struct Request {
    pub ip: String,
}

impl Message for Request {
    type Result = Result<bool, std::io::Error>;
}

impl Actor for RateLimiterStorage {
    type Context = Context<Self>;
}

impl RateLimiterStorage {
    pub fn new(max: i32) -> Self {
        RateLimiterStorage {
            clients: HashMap::new(),
            maxrpm: max,
        }
    }
}

pub struct RateLimiter {
    pub storage: Addr<RateLimiterStorage>,
}

impl Handler<Request> for RateLimiterStorage {
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, req: Request, _: &mut Context<Self>) -> Self::Result {
        if let Some(&(r, s)) = self.clients.get(&req.ip) {
            if Local::now().naive_local().signed_duration_since(s) > chrono::Duration::minutes(1) {
                self.clients.insert(req.ip, (0, Local::now().naive_local()));
            } else if r > self.maxrpm {
                return Ok(false);
            } else {
                self.clients.insert(req.ip, (r + 1, s));
            }
        } else {
            self.clients.insert(req.ip, (1, Local::now().naive_local()));
        }
        Ok(true)
    }
}

pub struct RateLimiterTransform<S> {
    pub service: S,
    pub ratelimiter: Addr<RateLimiterStorage>,
}

impl<S, B> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimiterTransform<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimiterTransform {
            service,
            ratelimiter: self.storage.clone(),
        }))
    }
}

impl<S, B> Service<ServiceRequest> for RateLimiterTransform<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let conn_info = req.connection_info().clone();
        let res = self.ratelimiter.send(Request {
            ip: conn_info.peer_addr().unwrap().to_string(),
        });
        let resp = self.service.call(req);
        Box::pin(async move {
            let res = res.await.unwrap().unwrap();
            if res {
                // COOL PERSON
                let resp = resp.await.unwrap();
                Ok(resp)
            } else {
                // UNCOOL PERSON
                // TODO: Lazy static impl of MAX_REQUESTS_PER_MINUTE
                Err(actix_web::error::ErrorTooManyRequests(format!(
                    "You have sent more than {} requests during the last minute. SLOW DOWN!",
                    std::env::var("MAX_REQUESTS_PER_MINUTE").unwrap_or("10".to_string())
                )))
            }
        })
    }
}