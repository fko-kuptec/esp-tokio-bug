use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener},
};

use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
    wifi::{AccessPointConfiguration, AuthMethod, BlockingWifi, Configuration, EspWifi},
};
use polling::{Event, Events, Poller};

static SSID: &str = "EspWifi";
static PASSWORD: &str = "s0meth1ng";

const FILE_SIZE: usize = 16 * 1024;
static FILE: [u8; FILE_SIZE] = [0; FILE_SIZE];

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    // Enable the use of the `eventfd` syscall for async runtimes
    let _eventfd = esp_idf_svc::io::vfs::MountedEventfs::mount(5).unwrap();

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

    // Run HTTP server
    run().unwrap();
}

fn run() -> std::io::Result<()> {
    let socket_key = 1337;
    let socket = TcpListener::bind("0.0.0.0:80")?;
    socket.set_nonblocking(true)?;

    let poller = Poller::new()?;
    unsafe { poller.add(&socket, Event::readable(socket_key))? };

    let mut events = Events::new();
    loop {
        events.clear();
        poller.wait(&mut events, None)?;
        log::info!("ACCEPT");

        for event in events.iter() {
            if event.key == socket_key {
                let stream_key = 2664;
                let (mut stream, address) = socket.accept()?;
                log::info!("connected to {address}");

                stream.set_nonblocking(true)?;
                unsafe { poller.add(&stream, Event::readable(stream_key))? };

                let mut events = Events::new();
                let mut buffer = vec![0; 1024];
                let mut index = 0;
                loop {
                    events.clear();
                    poller.wait(&mut events, None)?;
                    log::info!("READ");

                    for event in events.iter() {
                        if event.key == stream_key {
                            index += stream.read(&mut buffer[index..])?;
                        }
                    }

                    if let Ok(request) = std::str::from_utf8(&buffer) {
                        if request.contains("\r\n\r\n") {
                            break;
                        }
                    }

                    poller.modify(&stream, Event::readable(stream_key))?;
                }

                poller.modify(&stream, Event::writable(stream_key))?;

                let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/x-binary\r\nContent-Length: {FILE_SIZE}\r\n\r\n");
                let mut index = 0;
                while index < response.len() {
                    events.clear();
                    poller.wait(&mut events, None)?;
                    log::info!("WRITE HEADERS");

                    for event in events.iter() {
                        if event.key == stream_key {
                            index += stream.write(&response.as_bytes()[index..])?;
                            poller.modify(&stream, Event::writable(stream_key))?;
                        }
                    }
                }

                let mut index = 0;
                while index < FILE_SIZE {
                    events.clear();
                    poller.wait(&mut events, None)?;
                    log::info!("WRITE BODY");

                    for event in events.iter() {
                        if event.key == stream_key {
                            index += stream.write(&FILE[index..])?;
                            poller.modify(&stream, Event::writable(stream_key))?;
                        }
                    }
                }

                poller.delete(&stream)?;
                stream.shutdown(Shutdown::Both)?;
                log::info!("disconnected from {address}");
            }
        }

        poller.modify(&socket, Event::readable(socket_key))?;
    }
}
