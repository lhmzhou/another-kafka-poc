# another-kafka-poc

### Install Dependencies
`cargo install`
<br/><br/>

### Run the POC
`cargo run`
<br/><br/>

### Cargo Setup Behind Network Proxy
`vim ~/.cargo/config`
<br/><br/>


Copy and paste the following proxy configuration into your cargo/config file to enable your rust installation to download dependencies behind a proxy.

```
[http]
proxy = "http://localhost:3128"   # HTTP proxy to use for HTTP requests (defaults to none)
                                  # in libcurl format, e.g. "socks5h://host:port"
timeout = 60000                   # Timeout for each HTTP request, in milliseconds
```
<br/><br/>


### Rust Installation
https://www.rust-lang.org/en-US/install.html

### Rust Official Docker Image
https://github.com/rust-lang-nursery/docker-rust <br/>
https://hub.docker.com/_/rust/

### Docker Commands
```
docker build -t my-rust-app .
docker run -it --rm --name my-running-app my-rust-app
```

### Package for Offline Use
```
cargo fetch
```
