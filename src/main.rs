fn main() {
    let proxy = proxy::HTTPProxy::new(8000);
    proxy.run();
}
