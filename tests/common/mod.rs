use std::net::TcpListener;
use std::process::Command;
use std::sync::OnceLock;
use std::thread;
use std::time::Duration;

static SERVER: OnceLock<String> = OnceLock::new();

pub async fn spawn_app() -> String {
    if let Some(addr) = SERVER.get() {
        return addr.clone();
    }

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener
        .local_addr()
        .expect("Failed to get local address")
        .port();
    drop(listener);

    let address = format!("http://127.0.0.1:{port}");

    let mut child = Command::new(env!("CARGO_BIN_EXE_slowpokeapi"))
        .env("SLOWPOKEAPI__SERVER__PORT", port.to_string())
        .spawn()
        .expect("Failed to start app");

    for _ in 0..10 {
        if reqwest::Client::new()
            .get(format!("{address}/healthz"))
            .send()
            .await
            .is_ok()
        {
            let _ = SERVER.set(address.clone());
            std::mem::forget(child);
            return address;
        }
        thread::sleep(Duration::from_millis(200));
    }

    let _ = child.kill();
    let _ = child.wait();
    panic!("App failed to start");
}
