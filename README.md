# make-it-short-rs
Simple URL shortener written in Rust

## How to use
### Successful case
You're getting `201 Created` for POST and `308 Permanent Redirect` with original location for GET
```
❯ curl -v -H "Content-Type: application/json" -d '{ "url": "https://github.com/err0rless/rschat/blob/master/src/server/mod.rs" }' localhost:8080/shorten
...
< HTTP/1.1 201 Created
...
{"original_url":"https://github.com/err0rless/rschat/blob/master/src/server/mod.rs","short_url":"Wcrryb7uS"}

❯ curl -v localhost:8080/Wcrryb7uS
...
>
< HTTP/1.1 308 Permanent Redirect
< location: https://github.com/err0rless/rschat/blob/master/src/server/mod.rs
< content-length: 0
...
```

### Duplicate URLs on the system
You're getting `202 Accepted` without response body
```
❯ curl -v -H "Content-Type: application/json" -d '{ "url": "https://github.com/err0rless/rschat/blob/master/src/server/mod.rs" }' localhost:8080/shorten
...
< HTTP/1.1 202 Accepted
< content-length: 0
...
```

### URL is not registered
You're getting `404 Not Found`
```
❯ curl -v localhost:8080/__NOT_FOUND__
...
>
< HTTP/1.1 404 Not Found
< content-length: 0
...
```

### Something went wrong
If system caught unrecognized errors, you're getting `500 Internal Server Error`

## What's next?
- [ ] Take machine ID for distributed system
- [ ] Use our own EPOCH instead of Unix's for even shorter URLs
- [ ] Use persistent DB
