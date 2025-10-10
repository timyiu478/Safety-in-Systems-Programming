mod request;
mod response;

use std::collections::{HashSet, HashMap};
use clap::Parser;
use rand::{Rng, SeedableRng};
use tokio::sync::{Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{timeout, Duration, Instant};
use std::sync::Arc;
use reqwest;

/// Contains information parsed from the command-line invocation of balancebeam. The Clap macros
/// provide a fancy way to automatically construct a command-line argument parser.
#[derive(Parser, Debug)]
#[command(about = "Fun with load balancing")]
struct CmdOptions {
    /// "IP/port to bind to"
    #[arg(short, long, default_value = "0.0.0.0:1100")]
    bind: String,
    /// "Upstream host to forward requests to"
    #[arg(short, long)]
    upstream: Vec<String>,
    /// "Perform active health checks on this interval (in seconds)"
    #[arg(long, default_value = "10")]
    active_health_check_interval: usize,
    /// "Path to send request to for active health checks"
    #[arg(long, default_value = "/")]
    active_health_check_path: String,
    /// "Maximum number of requests to accept per IP per minute (0 = unlimited)"
    #[arg(long, default_value = "0")]
    max_requests_per_minute: usize,
}

/// Contains information about the state of balancebeam (e.g. what servers we are currently proxying
/// to, what servers have failed, rate limiting counts, etc.)
///
/// You should add fields to this struct in later milestones.
struct ProxyState {
    /// How frequently we check whether upstream servers are alive (Milestone 4)
    active_health_check_interval: usize,
    /// Where we should send requests when doing active health checks (Milestone 4)
    active_health_check_path: String,
    /// Maximum number of requests an individual IP can make in a minute (Milestone 5)
    max_requests_per_minute: usize,
    /// Addresses of servers that we are proxying to
    upstream_addresses: Vec<String>,
    /// Failed upstreams 
    failed_upstreams: Arc<Mutex<HashSet<String>>>,
    // Rate limiting counter
    rl_counter: Arc<Mutex<HashMap<String, (usize, Instant)>>>,
}

#[tokio::main]
async fn main() {
    // Initialize the logging library. You can print log messages using the `log` macros:
    // https://docs.rs/log/0.4.8/log/ You are welcome to continue using print! statements; this
    // just looks a little prettier.
    if let Err(_) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "debug");
    }
    pretty_env_logger::init();

    // Parse the command line arguments passed to this program
    let options = CmdOptions::parse();
    if options.upstream.len() < 1 {
        log::error!("At least one upstream server must be specified using the --upstream option.");
        std::process::exit(1);
    }

    // Start listening for connections
    let listener = match TcpListener::bind(&options.bind).await {
        Ok(listener) => listener,
        Err(err) => {
            log::error!("Could not bind to {}: {}", options.bind, err);
            std::process::exit(1);
        }
    };
    log::info!("Listening for requests on {}", options.bind);

    // Handle incoming connections
    // Arc allow multiple ownership and safe sharing across threads/tasks.
    let state = Arc::new(ProxyState {
        upstream_addresses: options.upstream,
        active_health_check_interval: options.active_health_check_interval,
        active_health_check_path: options.active_health_check_path,
        max_requests_per_minute: options.max_requests_per_minute,
        // tx_shutdown: broadcast::channel(16).0,
        failed_upstreams: Arc::new(Mutex::new(HashSet::new())),
        rl_counter: Arc::new(Mutex::new(std::collections::HashMap::new())),
    });


    // Start active health check task
    let state_clone = Arc::clone(&state);
    tokio::spawn(async move {
        active_health_check(&state_clone).await;
    });

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let state = Arc::clone(&state);
        tokio::spawn(async move {
            // each future needs its own copy of state reference
            handle_connection(socket, &state).await;
        });
    }
}

async fn connect_to_upstream(state: &ProxyState) -> Result<TcpStream, std::io::Error> {
    // If connecting to the upstream fails, we can assume that the upstream server is dead, and we can pick a different upstream server.
    let mut failed = state.failed_upstreams.lock().await; 
    while failed.len() < state.upstream_addresses.len() {
        let candidates: Vec<_> = state.upstream_addresses.iter()
            .filter(|addr| !failed.contains(*addr))
            .collect();
        let mut rng = rand::rngs::StdRng::from_entropy();
        let upstream_idx = rng.gen_range(0..candidates.len());
        let upstream_ip = &candidates[upstream_idx];
        match TcpStream::connect(upstream_ip).await {
            Ok(stream) => return Ok(stream),
            Err(err) => {
                log::error!("Failed to connect to upstream {}: {}", upstream_ip, err);
                // Mark this server as failed
                failed.insert(upstream_ip.to_string());
                // Broadcast the failure to other tasks
                // let _ = state.tx_shutdown.send((upstream_ip.to_string(), false));
            }
        }
    }
    log::error!("Failed to connect to upstream: all upstream servers are down");
    Err(std::io::Error::new(std::io::ErrorKind::Other, "All upstream servers are down"))
}

// Active health checks
async fn active_health_check(state: &ProxyState) {
    let path = state.active_health_check_path.clone();

    loop {
        tokio::time::sleep(Duration::from_secs(state.active_health_check_interval as u64)).await;
        let addresses = state.upstream_addresses.clone();
        for addr in addresses {
            let path = path.clone();
            let failed_upstreams = state.failed_upstreams.clone();
            tokio::spawn(async move {
                let check_url = format!("http://{}{}", addr, path);
                let client = reqwest::Client::new();
                let result = timeout(Duration::from_secs(1), client.get(&check_url).send()).await;
                match result {
                    Ok(Ok(response)) if response.status().is_success() => {
                        // Server is healthy
                        let mut failed = failed_upstreams.lock().await;
                        failed.remove(&addr)
                    }
                    _ => {
                        // Server is unhealthy
                        let mut failed = failed_upstreams.lock().await;
                        failed.insert(addr.clone())
                    }
                }
            });
        }
    }

}

async fn send_response(client_conn: &mut TcpStream, response: &http::Response<Vec<u8>>) {
    let client_ip = client_conn.peer_addr().unwrap().ip().to_string();
    log::info!(
        "{} <- {}",
        client_ip,
        response::format_response_line(&response)
    );
    if let Err(error) = response::write_to_stream(&response, client_conn).await {
        log::warn!("Failed to send response to client: {}", error);
        return;
    }
}

async fn is_rate_limit_allowed(client_ip: &str, state: &ProxyState) -> bool {
    if state.max_requests_per_minute == 0 {
        return true;
    }

    let mut rl_counter = state.rl_counter.lock().await;

    let entry = rl_counter.entry(client_ip.to_string()).or_insert((0, Instant::now()));

    let (ref mut current_count, ref mut current_start) = *entry;

    // Reset the counter if a minute has passed
    if current_start.elapsed() >= Duration::from_secs(60) {
        *current_count = 0;
        *current_start = Instant::now();
    }

    *current_count += 1;

    *current_count <= state.max_requests_per_minute
}

async fn handle_connection(mut client_conn: TcpStream, state: &ProxyState) {
    let client_ip = client_conn.peer_addr().unwrap().ip().to_string();
    log::info!("Connection received from {}", client_ip);

    // Rate limiting
    if !is_rate_limit_allowed(&client_ip, state).await {
        log::info!("Rate limit exceeded for {}", client_ip);
        let response = response::make_http_error(http::StatusCode::TOO_MANY_REQUESTS);
        send_response(&mut client_conn, &response).await;
        return;
    }

    // Open a connection to a random destination server
    let mut upstream_conn = match connect_to_upstream(state).await {
        Ok(stream) => stream,
        Err(_error) => {
            let response = response::make_http_error(http::StatusCode::BAD_GATEWAY);
            send_response(&mut client_conn, &response).await;
            return;
        }
    };
    let upstream_ip = upstream_conn.peer_addr().unwrap().ip().to_string();

    // The client may now send us one or more requests. Keep trying to read requests until the
    // client hangs up or we get an error.
    loop {
        // Read a request from the client
        let mut request = match request::read_from_stream(&mut client_conn).await {
            Ok(request) => request,
            // Handle case where client closed connection and is no longer sending requests
            Err(request::Error::IncompleteRequest(0)) => {
                log::debug!("Client finished sending requests. Shutting down connection");
                return;
            }
            // Handle I/O error in reading from the client
            Err(request::Error::ConnectionError(io_err)) => {
                log::info!("Error reading request from client stream: {}", io_err);
                return;
            }
            Err(error) => {
                log::debug!("Error parsing request: {:?}", error);
                let response = response::make_http_error(match error {
                    request::Error::IncompleteRequest(_)
                    | request::Error::MalformedRequest(_)
                    | request::Error::InvalidContentLength
                    | request::Error::ContentLengthMismatch => http::StatusCode::BAD_REQUEST,
                    request::Error::RequestBodyTooLarge => http::StatusCode::PAYLOAD_TOO_LARGE,
                    request::Error::ConnectionError(_) => http::StatusCode::SERVICE_UNAVAILABLE,
                });
                send_response(&mut client_conn, &response).await;
                continue;
            }
        };
        log::info!(
            "{} -> {}: {}",
            client_ip,
            upstream_ip,
            request::format_request_line(&request)
        );

        // Add X-Forwarded-For header so that the upstream server knows the client's IP address.
        // (We're the ones connecting directly to the upstream server, so without this header, the
        // upstream server will only know our IP, not the client's.)
        request::extend_header_value(&mut request, "x-forwarded-for", &client_ip);

        // Forward the request to the server
        if let Err(error) = request::write_to_stream(&request, &mut upstream_conn).await {
            log::error!(
                "Failed to send request to upstream {}: {}",
                upstream_ip,
                error
            );
            let response = response::make_http_error(http::StatusCode::BAD_GATEWAY);
            send_response(&mut client_conn, &response).await;
            return;
        }
        log::debug!("Forwarded request to server");

        // Read the server's response
        let response = match response::read_from_stream(&mut upstream_conn, request.method()).await {
            Ok(response) => response,
            Err(error) => {
                log::error!("Error reading response from server: {:?}", error);
                let response = response::make_http_error(http::StatusCode::BAD_GATEWAY);
                send_response(&mut client_conn, &response).await;
                return;
            }
        };
        // Forward the response to the client
        send_response(&mut client_conn, &response).await;
        log::debug!("Forwarded response to client");
    }
}
