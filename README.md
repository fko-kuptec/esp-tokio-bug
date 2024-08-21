# Purpose of this Repository

This example code shows an issue with using async TCP via `tokio` on the ESP. A `write` call with larger amounts of data gets stuck infinitely, because no notification is received that the stream is writeable again.