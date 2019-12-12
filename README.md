# foaas-rs

Rust implementation for the [FOAAS](https://github.com/tomdionysus/foaas) webservice.

## Building and running

Builde the webservice

```
cargo build
```

Start the webservices binary or run

```
cargo run [ip:port]
```

To test if the webservice is running curl an endpoint.

```
curl 127.0.0.1:[your port]/version
```

## Running the tests

```
cargo test
```

## Built With

* [serde](https://github.com/serde-rs/serde) - Serialization crate
* [hyper](https://github.com/hyperium/hyper) - Webservice crate

## Authors

* [**Jil Sahm**](https://github.com/jilsahm) - *Initial work*

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details