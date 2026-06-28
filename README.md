# Smart Home FFI

FFI-слой добавлен поверх исходной модели `SmartSocket` (из smart-home-patterns - прошлого задания)

## Пакеты

- `smart-socket-ffi` — библиотека умного дома из `smart-home-patterns` и C ABI для умной розетки
- `smart-socket-static-app` — пример, который вызывает C ABI через `extern "C"` и статически линкуется с `libsmart_socket_ffi.a`
- `smart-socket-dynamic-app` — пример, который загружает динамическую библиотеку в runtime через `libloading`

## Основной код

- `smart-socket-ffi/src/smart_device/socket.rs` — исходная реализация `PowerSocket`, `SmartSocket`, `Socket` и `SocketState`
- `smart-socket-ffi/src/lib.rs` — публичный Rust API и тонкая C ABI обертка
- `smart-socket-ffi/include/smart_socket_ffi.h` — C-заголовок для ABI

## Скрипты

Полная сборка и проверки

```sh
./build-app.sh
```

Запуск обеих демонстраций

```sh
./run-app.sh
```

Запуск только статической линковки

```sh
./run-app.sh --static
```

Запуск только динамической загрузки

```sh
./run-app.sh --dynamic
```

Сборка и запуск release-версии

```sh
./build-app.sh --release
./run-app.sh --release
```
