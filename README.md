# Rust HTTP Proxy and Cache

### Summary
- Intercepts client HTTP requests
- Modifies request headers, checks cache of responses, and checks strikeset for blocked domains
- If response not cached, makes request to upstream host, caches response, returns content to client
- *courtesy of reberhardt7 for custom request and response code

### Todo
- Finish cache implementation (specifically storing requests in cache files in serializable/deserializable way)
- Strikeset implementation
- More comprehensive and better organized testing