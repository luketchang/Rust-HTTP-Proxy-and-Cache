fn main() {
    let proxy = http_proxy::proxy::HTTPProxy::new(8000);
    proxy.run();
}
