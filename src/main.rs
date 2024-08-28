use std::net::TcpListener;

use async_io_mini::Async;
use edge_http::{
    io::server::{Connection, Handler},
    Method,
};
use edge_nal::{TcpAccept, TcpBind};
use edge_nal_std::{Stack, TcpSocket};
use esp_idf_hal::{
    cpu::Core,
    io::asynch::{Read, Write},
    task::{block_on, thread::ThreadSpawnConfiguration},
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
    wifi::{AccessPointConfiguration, AuthMethod, BlockingWifi, Configuration, EspWifi},
};

static SSID: &str = "EspWifi";
static PASSWORD: &str = "s0meth1ng";

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    // Enable the use of the `eventfd` syscall for async runtimes
    esp_idf_svc::io::vfs::initialize_eventfd(5).unwrap();

    // Initialize WIFI peripheral
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

    // Spawn worker thread
    ThreadSpawnConfiguration {
        pin_to_core: Some(Core::Core0),
        ..Default::default()
    }
    .set()
    .unwrap();

    std::thread::Builder::new()
        .name("http-server".into())
        .stack_size(32_000)
        .spawn(move || {
            let mut buffer = [0; 10_000];

            // block_on(http_server_async_io(&mut buffer))
            block_on(http_server_edge_nal_std(&mut buffer))
        })
        .expect("failed to spawn worker thread")
        .join()
        .expect("worker thread panicked")
        .expect("worker thread failed");
}

/// Works, even with logging and default pthread stack size
async fn http_server_async_io(buffer: &mut [u8]) -> anyhow::Result<()> {
    log::info!("before socket create"); // XXX
    let socket = Async::<TcpListener>::bind(([0, 0, 0, 0], 80)).unwrap();

    while let Ok((stream, _)) = socket.accept().await {
        let stream = TcpSocket::new(stream);
        let mut connection = Connection::<_, 32>::new(buffer, stream, None)
            .await
            .unwrap();
        HttpHandler.handle(&mut connection).await.unwrap();
    }

    Ok(())
}

/// * Works, if the logging line marked with `// XXX` is commented out and the
///   default pthread stack size is increased.
/// * Crashes, if the logging line is kept or the default pthread stack size is
///   not increased.
async fn http_server_edge_nal_std(buffer: &mut [u8]) -> anyhow::Result<()> {
    log::info!("before socket create"); // XXX
    let socket = Stack::new().bind(([0, 0, 0, 0], 80).into()).await.unwrap();

    while let Ok((_, stream)) = socket.accept().await {
        let mut connection = Connection::<_, 32>::new(buffer, stream, None)
            .await
            .unwrap();
        HttpHandler.handle(&mut connection).await.unwrap();
    }

    Ok(())
}

struct HttpHandler;

impl<'b, T, const N: usize> Handler<'b, T, N> for HttpHandler
where
    T: Read + Write,
    T::Error: Send + Sync + std::error::Error + 'static,
{
    type Error = anyhow::Error;

    async fn handle(&self, conn: &mut Connection<'b, T, N>) -> Result<(), Self::Error> {
        let headers = conn.headers()?;

        if !matches!(headers.method, Some(Method::Get)) {
            conn.initiate_response(405, Some("Method Not Allowed"), &[("Content-Length", "0")])
                .await?;
        } else if !matches!(headers.path, Some("/")) {
            conn.initiate_response(404, Some("Not Found"), &[("Content-Length", "0")])
                .await?;
        } else {
            conn.initiate_response(
                200,
                Some("OK"),
                &[("Content-Type", "text/plain"), ("Content-Length", "12")],
            )
            .await?;

            conn.write_all(b"Hello world!").await?;
        }

        Ok(())
    }
}
