This example panics, when run on an ESP32-S3. The panics are different, however, if the `log::info!(); // XXX` calls in the `http_server()` function are included or not:

With logs:
```log
I (808) esp_tokio_bug: before socket create
I (818) esp_tokio_bug: before server run
I (818) edge_http::io::server: Creating 4 handler tasks, memory: 10088B

assert failed: xQueueSemaphoreTake queue.c:1713 (pxQueue->uxItemSize == 0)


Backtrace: 0x40376a92:0x3fcb12b0 0x4037b5c5:0x3fcb12d0 0x40382a89:0x3fcb12f0 0x4037bd52:0x3fcb1410 0x403775fc:0x3fcb1450 0x403776bd:0x3fcb1480 0x42067735:0x3fcb14a0 0x420686ea:0x3fcb14c0 0x4201b1ba:0x3fcb1500 0x42019acc:0x3fcb1520 0x420174d9:0x3fcb1550 0x42018bb9:0x3fcb1640 0x42013a6c:0x3fcb16b0 0x42013cc7:0x3fcb1c90 0x42011901:0x3fcb1f30 0x420109be:0x3fcb1f50 0x42010846:0x3fcb1f90 0x4200bf4c:0x3fcb1fb0 0x42009883:0x3fcb3c70 0x42008ebd:0x3fcb7850 0x4200fda1:0x3fcb78a0 0x4200a778:0x3fcb78d0 0x4200b01b:0x3fcb79a0 0x42008af2:0x3fcbc170 0x4202cd73:0x3fcbc1a0 0x4202cd90:0x3fcbc1d0 0x4202d3f8:0x3fcbc200 0x4205ea60:0x3fcbc220
0x40376a92 - panic_abort
    at /home/florian/projects/esp-tokio-bug/.embuild/espressif/esp-idf/v5.2.2/components/esp_system/panic.c:466
0x4037b5c5 - esp_system_abort
    at /home/florian/projects/esp-tokio-bug/.embuild/espressif/esp-idf/v5.2.2/components/esp_system/port/esp_system_chip.c:93
0x40382a89 - __assert_func
    at /home/florian/projects/esp-tokio-bug/.embuild/espressif/esp-idf/v5.2.2/components/newlib/assert.c:81
0x4037bd52 - xQueueSemaphoreTake
    at /home/florian/projects/esp-tokio-bug/.embuild/espressif/esp-idf/v5.2.2/components/freertos/FreeRTOS-Kernel/queue.c:1713
0x403775fc - lock_acquire_generic
    at /home/florian/projects/esp-tokio-bug/.embuild/espressif/esp-idf/v5.2.2/components/newlib/locks.c:146
0x403776bd - _lock_acquire
    at /home/florian/projects/esp-tokio-bug/.embuild/espressif/esp-idf/v5.2.2/components/newlib/locks.c:154
0x42067735 - esp_vfs_register_fd_with_local_fd
    at /home/florian/projects/esp-tokio-bug/.embuild/espressif/esp-idf/v5.2.2/components/vfs/vfs.c:228
0x420686ea - eventfd
    at /home/florian/projects/esp-tokio-bug/.embuild/espressif/esp-idf/v5.2.2/components/vfs/vfs_eventfd.c:414
0x4201b1ba - rustix::backend::event::syscalls::eventfd
    at /home/florian/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rustix-0.38.34/src/backend/libc/event/syscalls.rs:60
0x42019acc - rustix::event::eventfd::eventfd
    at /home/florian/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rustix-0.38.34/src/event/eventfd.rs:19
0x420174d9 - polling::poll::Poller::new
    at /home/florian/.cargo/registry/src/index.crates.io-6f17d22bba15001f/polling-3.7.3/src/poll.rs:71
0x42018bb9 - polling::Poller::new
    at /home/florian/.cargo/registry/src/index.crates.io-6f17d22bba15001f/polling-3.7.3/src/lib.rs:456
0x42013a6c - async_io::reactor::Reactor::get::{{closure}}
    at /home/florian/.cargo/registry/src/index.crates.io-6f17d22bba15001f/async-io-2.3.4/src/reactor.rs:104
0x42013cc7 - async_lock::once_cell::OnceCell<T>::get_or_try_init_blocking
    at /home/florian/.cargo/registry/src/index.crates.io-6f17d22bba15001f/async-lock-3.4.0/src/once_cell.rs:451
0x42011901 - async_lock::once_cell::OnceCell<T>::get_or_init_blocking
    at /home/florian/.cargo/registry/src/index.crates.io-6f17d22bba15001f/async-lock-3.4.0/src/once_cell.rs:516
0x420109be - <async_io::Timer as futures_core::stream::Stream>::poll_next
    at /home/florian/.cargo/registry/src/index.crates.io-6f17d22bba15001f/async-io-2.3.4/src/lib.rs:502
0x42010846 - <async_io::Timer as core::future::future::Future>::poll
    at /home/florian/.cargo/registry/src/index.crates.io-6f17d22bba15001f/async-io-2.3.4/src/lib.rs:467
0x4200bf4c - <edge_nal_std::TcpAcceptor as edge_nal::stack::tcp::TcpAccept>::accept::{{closure}}
    at /home/florian/.cargo/git/checkouts/edge-net-465b5694b2f162db/947929d/edge-nal-std/src/lib.rs:95
0x42009883 - edge_http::io::server::Server<_,_,_>::run::{{closure}}
    at /home/florian/.cargo/git/checkouts/edge-net-465b5694b2f162db/947929d/edge-http/src/io/server.rs:590
0x42008ebd - async_task::raw::RawTask<F,T,S,M>::run
    at /home/florian/.cargo/registry/src/index.crates.io-6f17d22bba15001f/async-task-4.7.1/src/raw.rs:542
0x4200fda1 - async_task::runnable::Runnable<M>::run
    at /home/florian/.cargo/registry/src/index.crates.io-6f17d22bba15001f/async-task-4.7.1/src/runnable.rs:781
0x4200a778 - edge_executor::Executor<_>::run_unchecked::{{closure}}
    at /home/florian/.cargo/registry/src/index.crates.io-6f17d22bba15001f/edge-executor-0.4.1/src/lib.rs:256
0x4200b01b - esp_tokio_bug::main::{{closure}}
    at /home/florian/projects/esp-tokio-bug/src/main.rs:59
0x42008af2 - std::thread::Builder::spawn_unchecked_::{{closure}}::{{closure}}
    at /home/florian/.rustup/toolchains/esp/lib/rustlib/src/rust/library/std/src/thread/mod.rs:542
0x4202cd73 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
    at /home/florian/.rustup/toolchains/esp/lib/rustlib/src/rust/library/alloc/src/boxed.rs:2063
0x4202cd90 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
    at /home/florian/.rustup/toolchains/esp/lib/rustlib/src/rust/library/alloc/src/boxed.rs:2063
0x4202d3f8 - std::sys::pal::unix::thread::Thread::new::thread_start
    at /home/florian/.rustup/toolchains/esp/lib/rustlib/src/rust/library/std/src/sys/pal/unix/thread.rs:108
0x4205ea60 - pthread_task_func
    at /home/florian/projects/esp-tokio-bug/.embuild/espressif/esp-idf/v5.2.2/components/pthread/pthread.c:196
```

Without logs, one of these panics can happen:
```log
I (816) edge_http::io::server: Creating 4 handler tasks, memory: 10088B
Guru Meditation Error: Core  1 panic'ed (LoadProhibited). Exception was unhandled.

Core  1 register dump:
PC      : 0x4205ed70  PS      : 0x00060333  A0      : 0x8205eea8  A1      : 0x3fcc2dc0  
0x4205ed70 - find_key
    at /home/florian/projects/esp-tokio-bug/.embuild/espressif/esp-idf/v5.2.2/components/pthread/pthread_local_storage.c:80
A2      : 0x00060823  A3      : 0x3fcc3138  A4      : 0x3fcb1460  A5      : 0x00060023  
A6      : 0x4205e8dc  A7      : 0x00000003  A8      : 0x00000000  A9      : 0x3fcc2d90  
0x4205e8dc - pthread_task_func
    at /home/florian/projects/esp-tokio-bug/.embuild/espressif/esp-idf/v5.2.2/components/pthread/pthread.c:177
A10     : 0x00000001  A11     : 0xffffffff  A12     : 0x00060720  A13     : 0x00060323  
A14     : 0x00060023  A15     : 0x0000abab  SAR     : 0x00000018  EXCCAUSE: 0x0000001c  
EXCVADDR: 0x00060823  LBEG    : 0x00000000  LEND    : 0x00000000  LCOUNT  : 0x00000000  


Backtrace: 0x4205ed6d:0x3fcc2dc0 0x4205eea5:0x3fcc2de0 0x4203de2c:0x3fcc2e00 0x4203dd26:0x3fcc2e40 0x4203da39:0x3fcc2e70 0x4203842f:0x3fcc2e90 0x4203b267:0x3fcc2eb0 0x42038917:0x3fcc2f00 0x42011161:0x3fcc2f20 0x4202cc4f:0x3fcc2f50 0x4202cc6c:0x3fcc2f80 0x4202d2d4:0x3fcc2fb0 0x4205e904:0x3fcc2fd0
0x4205ed6d - find_key
    at /home/florian/projects/esp-tokio-bug/.embuild/espressif/esp-idf/v5.2.2/components/pthread/pthread_local_storage.c:86
0x4205eea5 - pthread_setspecific
    at /home/florian/projects/esp-tokio-bug/.embuild/espressif/esp-idf/v5.2.2/components/pthread/pthread_local_storage.c:197
0x4203de2c - std::sys::pal::unix::thread_local_key::set
    at /home/florian/.rustup/toolchains/esp/lib/rustlib/src/rust/library/std/src/sys/pal/unix/thread_local_key.rs:16
0x4203dd26 - std::sys::thread_local::os_local::Key<T>::try_initialize
    at /home/florian/.rustup/toolchains/esp/lib/rustlib/src/rust/library/std/src/sys/thread_local/os_local.rs:97
0x4203da39 - std::sys::thread_local::os_local::Key<T>::get
    at /home/florian/.rustup/toolchains/esp/lib/rustlib/src/rust/library/std/src/sys/thread_local/os_local.rs:76
0x4203842f - std::thread::CURRENT::{{closure}}
    at /home/florian/.rustup/toolchains/esp/lib/rustlib/src/rust/library/std/src/sys/thread_local/os_local.rs:31
0x4203b267 - std::thread::local::LocalKey<T>::try_with
    at /home/florian/.rustup/toolchains/esp/lib/rustlib/src/rust/library/std/src/thread/local.rs:282
0x42038917 - std::thread::set_current
    at /home/florian/.rustup/toolchains/esp/lib/rustlib/src/rust/library/std/src/thread/mod.rs:711
0x42011161 - std::thread::Builder::spawn_unchecked_::{{closure}}
    at /home/florian/.rustup/toolchains/esp/lib/rustlib/src/rust/library/std/src/thread/mod.rs:540
0x4202cc4f - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
    at /home/florian/.rustup/toolchains/esp/lib/rustlib/src/rust/library/alloc/src/boxed.rs:2063
0x4202cc6c - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
    at /home/florian/.rustup/toolchains/esp/lib/rustlib/src/rust/library/alloc/src/boxed.rs:2063
0x4202d2d4 - std::sys::pal::unix::thread::Thread::new::thread_start
    at /home/florian/.rustup/toolchains/esp/lib/rustlib/src/rust/library/std/src/sys/pal/unix/thread.rs:108
0x4205e904 - pthread_task_func
    at /home/florian/projects/esp-tokio-bug/.embuild/espressif/esp-idf/v5.2.2/components/pthread/pthread.c:196
```

```log
I (819) edge_http::io::server: Creating 4 handler tasks, memory: 10088B

***ERROR*** A stack overflow in task pthread has been detected.


Backtrace: 0x40376a92:0x3fcc2110 0x4037b5c5:0x3fcc2130 0x4037c23e:0x3fcc2150 0x4037d2c7:0x3fcc21d0 0x4037c370:0x3fcc2200 0x4037c366:0x3fcc21f0 0x0006051d:0x4037c2e6 |<-CORRUPTED
0x40376a92 - panic_abort
    at /home/florian/projects/esp-tokio-bug/.embuild/espressif/esp-idf/v5.2.2/components/esp_system/panic.c:466
0x4037b5c5 - esp_system_abort
    at /home/florian/projects/esp-tokio-bug/.embuild/espressif/esp-idf/v5.2.2/components/esp_system/port/esp_system_chip.c:93
0x4037c23e - vApplicationStackOverflowHook
    at /home/florian/projects/esp-tokio-bug/.embuild/espressif/esp-idf/v5.2.2/components/freertos/FreeRTOS-Kernel/portable/xtensa/port.c:553
0x4037d2c7 - vTaskSwitchContext
    at /home/florian/projects/esp-tokio-bug/.embuild/espressif/esp-idf/v5.2.2/components/freertos/FreeRTOS-Kernel/tasks.c:3664
0x4037c370 - _frxt_dispatch
    at /home/florian/projects/esp-tokio-bug/.embuild/espressif/esp-idf/v5.2.2/components/freertos/FreeRTOS-Kernel/portable/xtensa/portasm.S:451
```