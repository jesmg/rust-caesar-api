# Caesar encrypt/decrypt API

Microservice exposing API endpoints for doing Caesar encryption and decryption.

This is my first program written in Rust :)

## How to run it

Set the Rust nightly build for this repo: `rustup override set nightly`.
Launch with `cargo run` or compile with `cargo build`.

## Endpoints

* **POST /caesar/decrypt** expecting a body like:
    ```
    {
        "secret": "abyz",
        "shift": 25
    }
    ```
* **POST /caesar/encrypt** expecting a body like:
    ```
    {
        "secret": "abyz",
        "shift": 25
    }
    ```
*  **GET /liveness**