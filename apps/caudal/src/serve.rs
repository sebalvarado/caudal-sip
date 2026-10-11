use std::collections::{HashMap, HashSet};
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use axum::body::Bytes;
use axum::extract::{ConnectInfo, DefaultBodyLimit, Path, Request, State};
use axum::http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use caudal_types::Usd;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use tower_http::cors::CorsLayer;

use crate::commands::{clock, name_of, state_label};
use crate::sandbox::{INSTITUTIONS, LLAVES, REDES, Sandbox, institution, money};
use crate::scenarios::DISCLAIMER;

pub const DISCLAIMER_HEADER: &str = "x-caudal-disclaimer";
const DISCLAIMER_ASCII: &str = "Demostracion con datos ficticios. Proyecto Caudal es independiente y no esta afiliado al Banco Central del Ecuador.";
const REQUESTS_PER_MINUTE: u32 = 30;
const MAX_AMOUNT: Usd = Usd::from_cents(100_000);

pub struct Demo {
    sandbox: Mutex<Sandbox>,
    made: Mutex<HashSet<String>>,
    windows: Mutex<HashMap<IpAddr, (Instant, u32)>>,
}

impl Demo {
    pub fn new() -> Self {
        Self { sandbox: Mutex::new(Sandbox::new()), made: Mutex::default(), windows: Mutex::default() }
    }

    pub fn reset(&self) {
        *self.sandbox.lock().expect("not poisoned") = Sandbox::new();
        self.made.lock().expect("not poisoned").clear();
    }

    fn sandbox(&self) -> std::sync::MutexGuard<'_, Sandbox> {
        self.sandbox.lock().expect("not poisoned")
    }
}

impl Default for Demo {
    fn default() -> Self {
        Self::new()
    }
}

fn reply(status: StatusCode, mut body: Value) -> Response {
    body["disclaimer"] = json!(DISCLAIMER);
    (status, Json(body)).into_response()
}

fn bad(error: impl Into<String>) -> Response {
    reply(StatusCode::BAD_REQUEST, json!({ "error": error.into() }))
}

fn parse<T: DeserializeOwned>(body: &Bytes) -> Result<T, String> {
    serde_json::from_slice(body).map_err(|e| format!("invalid request: {e}"))
}

pub fn router(demo: Arc<Demo>, origin: HeaderValue) -> Router {
    Router::new()
        .route("/api/network", get(network))
        .route("/api/llaves", get(llaves))
        .route("/api/resolve", post(resolve))
        .route("/api/payments", post(pay))
        .route("/api/payments/{id}", get(payment))
        .route("/api/settle", post(settle))
        .fallback(|| async { reply(StatusCode::NOT_FOUND, json!({ "error": "not found" })) })
        .layer(middleware::from_fn_with_state(demo.clone(), limit))
        .layer(DefaultBodyLimit::max(4 * 1024))
        .layer(
            CorsLayer::new()
                .allow_origin(origin)
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([header::CONTENT_TYPE])
                .expose_headers([HeaderName::from_static(DISCLAIMER_HEADER)]),
        )
        .layer(middleware::from_fn(disclaim))
        .with_state(demo)
}

async fn disclaim(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    response.headers_mut().insert(DISCLAIMER_HEADER, HeaderValue::from_static(DISCLAIMER_ASCII));
    response
}

fn client(headers: &HeaderMap, peer: Option<SocketAddr>) -> Option<IpAddr> {
    headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .and_then(|v| v.trim().parse().ok())
        .or(peer.map(|p| p.ip()))
}

async fn limit(State(demo): State<Arc<Demo>>, request: Request, next: Next) -> Response {
    let peer = request.extensions().get::<ConnectInfo<SocketAddr>>().map(|c| c.0);
    if let Some(ip) = client(request.headers(), peer) {
        let mut windows = demo.windows.lock().expect("not poisoned");
        let now = Instant::now();
        windows.retain(|_, (start, _)| now.duration_since(*start) < Duration::from_secs(60));
        let (_, count) = windows.entry(ip).or_insert((now, 0));
        *count += 1;
        if *count > REQUESTS_PER_MINUTE {
            return reply(
                StatusCode::TOO_MANY_REQUESTS,
                json!({ "error": "too many requests; try again in a minute" }),
            );
        }
    }
    next.run(request).await
}

async fn network(State(demo): State<Arc<Demo>>) -> Response {
    let sandbox = demo.sandbox();
    let institutions: Vec<Value> = sandbox
        .institutions()
        .into_iter()
        .map(|v| {
            json!({
                "id": v.id, "name": v.name, "red": v.red, "sector": v.sector,
                "ceiling": money(i128::from(v.ceiling.cents())), "available": money(v.available),
                "used_percent": v.used_percent(),
                "light": v.light.label(), "net": money(v.net),
            })
        })
        .collect();
    let redes: Vec<Value> = REDES.iter().map(|r| json!({ "id": r.id, "name": r.name })).collect();
    let last = sandbox.network().settlements().last().map(|s| clock(s.at));
    reply(
        StatusCode::OK,
        json!({ "clock": clock(sandbox.now()), "redes": redes, "institutions": institutions, "last_settlement": last }),
    )
}

async fn llaves() -> Response {
    let list: Vec<Value> = LLAVES
        .iter()
        .map(|l| {
            let kind = match l.kind {
                caudal_types::LlaveType::Celular => "celular",
                caudal_types::LlaveType::Identificacion => "identificacion",
                caudal_types::LlaveType::Correo => "correo",
                caudal_types::LlaveType::Alfanumerico => "alfanumerico",
            };
            json!({ "id": l.id, "type": kind, "code": l.kind.code(), "institution": l.institution })
        })
        .collect();
    let institutions: Vec<&str> = INSTITUTIONS.iter().map(|i| i.id).collect();
    reply(StatusCode::OK, json!({ "llaves": list, "institutions": institutions }))
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ResolveRequest {
    llave: String,
}

fn catalogue_llave(id: &str) -> Result<&'static str, String> {
    LLAVES
        .iter()
        .find(|l| l.id == id)
        .map(|l| l.id)
        .ok_or_else(|| "unknown llave id; GET /api/llaves lists them".into())
}

async fn resolve(State(demo): State<Arc<Demo>>, body: Bytes) -> Response {
    let request: ResolveRequest = match parse(&body) {
        Ok(r) => r,
        Err(e) => return bad(e),
    };
    let id = match catalogue_llave(&request.llave) {
        Ok(id) => id,
        Err(e) => return bad(e),
    };
    match demo.sandbox().resolve(id) {
        Ok(r) => reply(
            StatusCode::OK,
            json!({ "llave": id, "name": r.name.as_str(), "institution": name_of(r.integrante), "red": r.red.to_string() }),
        ),
        Err(e) => bad(e.to_string()),
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PayRequest {
    from: String,
    llave: String,
    amount: String,
}

fn payment_view(sandbox: &Sandbox, id: &str) -> Option<Value> {
    let (row, hops) = sandbox.payment(id)?;
    let hops: Vec<Value> = hops
        .iter()
        .map(|h| {
            json!({
                "at": clock(h.at), "from": name_of(h.from), "to": name_of(h.to), "message": h.definition,
                "state": h.state.map(|s| s.to_string()), "reason": h.reason.map(|r| r.to_string()),
                "lost": h.dropped, "xml": caudal_iso20022::pretty(&h.xml),
            })
        })
        .collect();
    Some(json!({
        "id": row.id.to_string(), "from": name_of(row.from), "to": name_of(row.to), "amount": row.amount.to_string(),
        "state": row.state.to_string(), "reason": row.reason.map(|r| r.to_string()), "label": state_label(&row),
        "por_conciliar": row.por_conciliar, "hops": hops,
    }))
}

async fn pay(State(demo): State<Arc<Demo>>, body: Bytes) -> Response {
    let request: PayRequest = match parse(&body) {
        Ok(r) => r,
        Err(e) => return bad(e),
    };
    if institution(&request.from).is_none() {
        return bad("unknown institution id; GET /api/network lists them");
    }
    let llave = match catalogue_llave(&request.llave) {
        Ok(id) => id,
        Err(e) => return bad(e),
    };
    let amount = match request.amount.parse::<Usd>() {
        Ok(a) if a > Usd::ZERO && a <= MAX_AMOUNT => a,
        _ => return bad("amount is a decimal string between 0.01 and 1000.00 with at most two decimals"),
    };
    let mut sandbox = demo.sandbox();
    match sandbox.pay(&request.from, llave, amount) {
        Ok(id) => {
            demo.made.lock().expect("not poisoned").insert(id.to_string());
            reply(StatusCode::OK, payment_view(&sandbox, id.as_str()).expect("just made"))
        }
        Err(e) => bad(e.to_string()),
    }
}

async fn payment(State(demo): State<Arc<Demo>>, Path(id): Path<String>) -> Response {
    if !demo.made.lock().expect("not poisoned").contains(&id) {
        return reply(StatusCode::NOT_FOUND, json!({ "error": "not a payment this demo made" }));
    }
    match payment_view(&demo.sandbox(), &id) {
        Some(view) => reply(StatusCode::OK, view),
        None => reply(StatusCode::NOT_FOUND, json!({ "error": "not found" })),
    }
}

async fn settle(State(demo): State<Arc<Demo>>) -> Response {
    let s = demo.sandbox().settle();
    let nets: serde_json::Map<String, Value> =
        s.malla.nets.iter().map(|(b, n)| (name_of(*b), json!(money(*n)))).collect();
    reply(
        StatusCode::OK,
        json!({ "at": clock(s.at), "payments": s.malla.payments.len(), "gross": money(i128::from(s.malla.gross().cents())), "net": money(i128::from(s.malla.total().cents())), "nets": nets }),
    )
}

pub async fn serve(port: u16, origin: &str, reset: Duration) -> std::io::Result<()> {
    let origin = HeaderValue::from_str(origin).map_err(|e| std::io::Error::other(format!("origin: {e}")))?;
    let demo = Arc::new(Demo::new());
    let resetting = demo.clone();
    tokio::spawn(async move {
        let mut every = tokio::time::interval(reset);
        every.tick().await;
        loop {
            every.tick().await;
            resetting.reset();
        }
    });
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port)).await?;
    eprintln!("caudal demo API on :{port}, resetting every {}s", reset.as_secs());
    axum::serve(listener, router(demo, origin).into_make_service_with_connect_info::<SocketAddr>()).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    fn app() -> Router {
        router(Arc::new(Demo::new()), HeaderValue::from_static("https://caudal-sip.pages.dev"))
    }

    async fn call(app: &Router, method: &str, uri: &str, body: &str) -> (StatusCode, HeaderMap, Value) {
        let request = Request::builder()
            .method(method)
            .uri(uri)
            .header("content-type", "application/json")
            .header("x-forwarded-for", "203.0.113.7")
            .body(Body::from(body.to_owned()))
            .unwrap();
        let response = app.clone().oneshot(request).await.unwrap();
        let (status, headers) = (response.status(), response.headers().clone());
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        (status, headers, serde_json::from_slice(&bytes).unwrap_or(Value::Null))
    }

    #[tokio::test]
    async fn a_payment_through_the_api_returns_every_hop_and_the_disclaimer() {
        let app = app();
        let (status, headers, body) =
            call(&app, "POST", "/api/payments", r#"{"from":"banco-alfa","llave":"llave-01","amount":"25.40"}"#).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["state"], "ACSP");
        assert!(body["hops"].as_array().unwrap().len() >= 8);
        assert_eq!(body["disclaimer"], DISCLAIMER);
        assert!(headers.contains_key(DISCLAIMER_HEADER));
        let id = body["id"].as_str().unwrap().to_owned();
        let (status, _, read) = call(&app, "GET", &format!("/api/payments/{id}"), "").await;
        assert_eq!((status, read["id"].as_str()), (StatusCode::OK, Some(id.as_str())));
        let (_, _, settled) = call(&app, "POST", "/api/settle", "").await;
        assert!(settled["payments"].as_u64() >= Some(1));
        let (_, _, read) = call(&app, "GET", &format!("/api/payments/{id}"), "").await;
        assert_eq!(read["state"], "ACSC");
    }

    #[tokio::test]
    async fn nothing_free_form_gets_in() {
        let app = app();
        for body in [
            r#"{"from":"banco-alfa","llave":"0990000001","amount":"25.40"}"#,
            r#"{"from":"BALFECEQ","llave":"llave-01","amount":"25.40"}"#,
            r#"{"from":"banco-alfa","llave":"llave-01","amount":"1000.01"}"#,
            r#"{"from":"banco-alfa","llave":"llave-01","amount":"0.001"}"#,
            r#"{"from":"banco-alfa","llave":"llave-01","amount":"25.40","account":"4400123456"}"#,
        ] {
            let (status, _, reply) = call(&app, "POST", "/api/payments", body).await;
            assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
            assert_eq!(reply["disclaimer"], DISCLAIMER);
        }
        let (status, _, _) = call(&app, "GET", "/api/payments/BALFECEQ30112026080000000001", "").await;
        assert_eq!(status, StatusCode::NOT_FOUND, "only payments the demo made are readable");
        let (status, headers, _) = call(&app, "GET", "/admin", "").await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(headers.contains_key(DISCLAIMER_HEADER));
    }

    #[tokio::test]
    async fn resolve_returns_only_the_masked_name() {
        let (status, _, body) = call(&app(), "POST", "/api/resolve", r#"{"llave":"llave-01"}"#).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["name"], "Julio C. T. A.");
        assert!(!body.to_string().contains("Tienda Andrade"));
    }

    #[tokio::test]
    async fn a_client_gets_thirty_requests_a_minute() {
        let app = app();
        for _ in 0..REQUESTS_PER_MINUTE {
            assert_eq!(call(&app, "GET", "/api/llaves", "").await.0, StatusCode::OK);
        }
        assert_eq!(call(&app, "GET", "/api/llaves", "").await.0, StatusCode::TOO_MANY_REQUESTS);
    }
}
