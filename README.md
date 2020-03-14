# homochecker-rs
Rust implementation of [chitoku-k/HomoChecker](https://github.com/chitoku-k/HomoChecker) API.

* 🚀 Blazingly Fast
* 📦 Easy to Use
* 👨🏻‍🔧 High Torelance

## Implemented APIs Overview
* Check API
    - `GET /check`
    - `GET /check/:user`
    - Query parameter
        - `format`: `sse` or `json` (optional, default to `sse`)
* List API
    - `GET /list`
    - `GET /list/:user`
    - Query parameteqr
        - `format`: `json` or `sql` (optional, default to `json`)
* Badge API
    - `GET /badge`
