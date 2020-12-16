mod proxy;
mod scheduler;
mod handler;
mod strikeset;
mod cache;

use proxy::HTTPProxy;

fn main() {
    let proxy = HTTPProxy::new(8000);
    proxy.run();
}
