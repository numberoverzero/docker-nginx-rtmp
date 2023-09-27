use core::fmt::Write;
use hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server, StatusCode,
};
use nanorand::{BufferedRng, ChaCha20, Rng};
use std::{convert::Infallible, env, net::SocketAddr, sync::Arc};
use subtle::ConstantTimeEq;
use tokio::{signal::unix::SignalKind, task::JoinSet};
use url;

const ACCESS_KEY_ENV: &'static str = "MA_ACCESS_KEY";
const DEFAULT_ACCESS_KEY_LENGTH_BYTES: usize = 4;

const SOCKET_ENV: &'static str = "MA_SOCKET";
const DEFAULT_SOCKET: &'static str = "0.0.0.0:5000";

const QUERY_STRING_KEY_ENV: &'static str = "MA_QUERYSTRING_KEY";
const DEFAULT_QUERY_STRING_KEY: &'static str = "key";

struct ServiceConfig {
    access_key: String,
    listen_socket: SocketAddr,
    qs_key: String,
}

macro_rules! expect {
    ( $e: expr, $msg: expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => {
                die!($msg);
            }
        }
    };
}

macro_rules! die {
    ( $arg: expr ) => {
        eprintln!("error: {}", $arg);
        std::process::exit(1);
    };
}

#[tokio::main]
async fn main() {
    let svc_cfg = Arc::new(load_service_config());
    let make_svc = make_service_fn(|_: &AddrStream| {
        let svc_cfg = svc_cfg.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let svc_cfg = svc_cfg.clone();
                async move { handle(req, &svc_cfg).await }
            }))
        }
    });
    let server = Server::bind(&svc_cfg.listen_socket).serve(make_svc);
    println!("socket: {}", svc_cfg.listen_socket);
    println!("access_key: {}", svc_cfg.access_key);
    println!("querystring_key: {}", svc_cfg.qs_key);

    match server.with_graceful_shutdown(register_signals()).await {
        Ok(_) => {
            println!("shutting down");
        }
        Err(e) => {
            die!(format!("server: {e}"));
        }
    }
}

async fn register_signals() {
    /// The server will shut down when any of these signals is received
    const EXIT_SIGNALS: &[SignalKind] = &[
        SignalKind::terminate(),
        SignalKind::interrupt(),
        SignalKind::quit(),
    ];
    async fn register_signal(kind: SignalKind) {
        tokio::signal::unix::signal(kind)
            .expect(format!("expected to register signal handler {kind:?}").as_str())
            .recv()
            .await;
    }
    let mut signals = JoinSet::new();
    for sig in EXIT_SIGNALS {
        signals.spawn(register_signal(*sig));
    }
    // finish on first received
    signals.join_next().await;
}

async fn handle(req: Request<Body>, cfg: &ServiceConfig) -> Result<Response<Body>, Infallible> {
    let claimed_key = find_one_qs_value(req, &cfg.qs_key);
    let status = match claimed_key {
        Some(claimed_key) => {
            let actual_key = &cfg.access_key;
            if claimed_key.as_bytes().ct_eq(actual_key.as_bytes()).into() {
                println!("allow: correct access key");
                StatusCode::NO_CONTENT
            } else {
                println!("deny: incorrect access key");
                StatusCode::UNAUTHORIZED
            }
        }
        None => {
            println!("deny: missing access key");
            StatusCode::BAD_REQUEST
        }
    };
    Ok(expect!(
        Response::builder().status(status).body(Body::empty()),
        "failed to build body"
    ))
}

fn find_one_qs_value(req: Request<Body>, key: &String) -> Option<String> {
    let qs = req.uri().query().unwrap_or("");
    let mut it = url::form_urlencoded::parse(qs.as_bytes());
    it.find_map(|(k, v)| match k == *key {
        true => Some(v.trim().to_string()),
        false => None,
    })
}

fn load_service_config() -> ServiceConfig {
    let mut access_key: String = env::var(ACCESS_KEY_ENV).unwrap_or("".into()).trim().into();
    if access_key.len() == 0 {
        println!("generating random access key (set your own with {ACCESS_KEY_ENV}=)");
        access_key = generate_access_key();
    }

    let listen_str = env::var(SOCKET_ENV).unwrap_or(DEFAULT_SOCKET.into());
    let listen_socket = expect!(
        listen_str.parse::<SocketAddr>(),
        format!("malformed {SOCKET_ENV}='{listen_str}'")
    );

    let qs_key = env::var(QUERY_STRING_KEY_ENV).unwrap_or(DEFAULT_QUERY_STRING_KEY.into());

    ServiceConfig {
        access_key,
        listen_socket,
        qs_key,
    }
}

fn generate_access_key() -> String {
    let mut raw = [0u8; DEFAULT_ACCESS_KEY_LENGTH_BYTES];
    let mut rng = BufferedRng::new(ChaCha20::new());
    rng.fill(&mut raw);
    let mut key = String::with_capacity(2 * raw.len());
    for byte in raw {
        expect!(write!(key, "{:02x}", byte), "failed to generate access key");
    }
    key
}
