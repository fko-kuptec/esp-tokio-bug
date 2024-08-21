use std::{future::Future, time::Duration};

use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
    wifi::{AccessPointConfiguration, AuthMethod, BlockingWifi, Configuration, EspWifi},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

static SSID: &str = "EspWifi";
static PASSWORD: &str = "s0meth1ng";

const FILE_SIZE: usize = 8 * 1024;
static FILE: [u8; FILE_SIZE] = [0; FILE_SIZE];

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    // Enable the use of the `eventfd` syscall for async runtimes
    let _eventfd = esp_idf_svc::io::vfs::MountedEventfs::mount(5).unwrap();

    // Initialize wifi peripheral
    let peripherals = esp_idf_hal::peripherals::Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take().unwrap();
    let partition = EspDefaultNvsPartition::take().unwrap();
    let wifi = EspWifi::new(peripherals.modem, sysloop.clone(), Some(partition)).unwrap();
    let mut wifi = BlockingWifi::wrap(wifi, sysloop).unwrap();

    // Start WIFI network
    wifi.set_configuration(&Configuration::AccessPoint(AccessPointConfiguration {
        ssid: SSID.try_into().unwrap(),
        auth_method: AuthMethod::WPA2Personal,
        password: PASSWORD.try_into().unwrap(),
        ..Default::default()
    }))
    .unwrap();
    wifi.start().unwrap();

    // Start worker thread for HTTP server with tokio runtime
    std::thread::Builder::new()
        .name(String::from("runtime"))
        .stack_size(16_000)
        .spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_time()
                .enable_io()
                .build()
                .unwrap()
                .block_on(run_http_server())
        })
        .unwrap()
        .join()
        .unwrap();
}

fn run_http_server() -> impl Future<Output: Send> + 'static {
    let local = tokio::task::LocalSet::new();
    local.spawn_local(async move {
        let socket = TcpListener::bind("0.0.0.0:80").await.unwrap();

        loop {
            match socket.accept().await {
                Ok((mut stream, address)) => {
                    tokio::task::spawn_local(async move {
                        log::info!("[HTTP] connected to {address}");

                        // Read the HTTP request
                        let mut buffer = vec![0; 1024];
                        let mut index = 0;
                        while !std::str::from_utf8(&buffer)
                            .map(|requ| requ.contains("\r\n\r\n"))
                            .unwrap_or_default()
                        {
                            // Resize buffer if it is full
                            if index == buffer.len() {
                                buffer.resize(buffer.len() + 1024, 0);
                            }

                            index += stream.read(&mut buffer[index..]).await.unwrap();
                        }

                        // Ignore the request and send the default file as response
                        stream.write_all(b"HTTP/1.1 200 OK\r\n").await.unwrap();
                        stream
                            .write_all(b"Content-Type: application/x-binary\r\n")
                            .await
                            .unwrap();
                        stream
                            .write_all(format!("Content-Length: {FILE_SIZE}\r\n\r\n").as_bytes())
                            .await
                            .unwrap();

                        // Send the raw file data, limiting the write by a timeout
                        if tokio::time::timeout(Duration::from_secs(2), stream.write_all(&FILE))
                            .await
                            .is_ok()
                        {
                            // No timeout
                            panic!("this is never reached");
                        } else {
                            // Timeout
                            log::warn!("[HTTP] timed out while writing");
                            stream.writable().await.unwrap();
                            panic!("this is never reached");
                        }
                    });
                }
                Err(error) => {
                    log::error!("[HTTP] failed to accept connection: {error}");
                }
            }
        }
    });

    local
}
