# Purpose of this Repository

This example code shows an issue with using async TCP via `tokio` on the ESP. A `write` call with larger amounts of data gets stuck infinitely, because no notification is received that the stream is writeable again.

When connecting with a Windows machine to the WIFI network and sending an HTTP request using `Invoke-WebRequest http://192.168.71.1/`, the download gets stuck at 5683 bytes, while the ESP prints something like the following:

```text
I (72553) esp_tokio_bug: [HTTP] connected to 192.168.71.2:61545
W (74583) esp_tokio_bug: [HTTP] timed out while writing
```

It is possible to then send more requests, which also get stuck at the same position.