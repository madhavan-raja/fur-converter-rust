use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use std::error::Error;

#[derive(serde::Deserialize)]
struct EncodeParams {
    pub data: String,
    pub size: String,
}

#[get("/unsigned_integer/encode")]
pub async fn encode(req: HttpRequest) -> Result<impl Responder, Box<dyn Error>> {
    let params = web::Query::<EncodeParams>::from_query(req.query_string()).unwrap();

    let data = params.data.parse::<u128>()?;
    let size = params.size.parse::<usize>()?;

    let bin = format!("{:0size$b}", data, size = size);

    Ok(HttpResponse::Ok().body(bin))
}

#[derive(serde::Deserialize)]
struct DecodeParams {
    pub binary: String,
}

#[get("/unsigned_integer/decode")]
pub async fn decode(req: HttpRequest) -> Result<impl Responder, Box<dyn Error>> {
    let params = web::Query::<DecodeParams>::from_query(req.query_string()).unwrap();

    let binary = params.binary.clone();
    let decimal = format!("{}", isize::from_str_radix(&binary, 2)?);

    Ok(HttpResponse::Ok().body(decimal))
}

#[derive(serde::Deserialize)]
struct CompareParams {
    pub a: String,
    pub b: String,
}

#[get("/unsigned_integer/compare")]
pub async fn compare(req: HttpRequest) -> Result<impl Responder, Box<dyn Error>> {
    let params = web::Query::<CompareParams>::from_query(req.query_string()).unwrap();

    let a = isize::from_str_radix(&params.a, 2)?;
    let b = isize::from_str_radix(&params.b, 2)?;

    let mut res = "0";

    if a > b {
        res = "-1";
    } else if a < b {
        res = "1";
    }

    Ok(HttpResponse::Ok().body(res))
}
