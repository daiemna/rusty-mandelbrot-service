use super::common::ErrResponse;
use actix_web::{web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GcdRequest {
    n: u64,
    m: u64,
}

#[derive(Serialize, Deserialize)]
struct GcdResponse {
    result: u64,
}

pub async fn post_gcd(request: web::Json<GcdRequest>) -> Result<HttpResponse, Error> {
    match compute_gcd(request.n, request.m) {
        Ok(v) => {
            let res_str = format!("{{\"result\": {}}}", v);
            let res = serde_json::from_str::<GcdResponse>(&res_str)?;
            return Ok(HttpResponse::Ok().json(res));
        }
        Err(e) => {
            let res_str = format!("{{\"error\": {}}}", e);
            let error = serde_json::from_str::<ErrResponse>(&res_str)?;
            return Ok(HttpResponse::BadRequest().json(error));
        }
    }
}

fn compute_gcd(mut n: u64, mut m: u64) -> Result<u64, &'static str> {
    if n == 0 {
        return Err("n cannot be 0");
    }
    if m == 0 {
        return Err("m cannot be 0");
    }
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    return Ok(n);
}

#[test]
// #[allow(dead_code)]
fn test_gcd() {
    match compute_gcd(14, 15) {
        Ok(v) => assert_eq!(v, 1),
        Err(e) => assert_eq!(e, ""),
    };
    match compute_gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19) {
        Ok(v) => assert_eq!(v, 3 * 11),
        Err(e) => assert_eq!(e, ""),
    };
}
