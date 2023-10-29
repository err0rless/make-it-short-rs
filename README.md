# make-it-short-rs
Simple URL shortener written in Rust

## How to use
### Registration
Request POST on `/shorten` with a json body, example: `{ "url": "__full_url__" }`
```shell
❯ curl -v -H "Content-Type: application/json" -d '{ "url": "https://github.com/err0rless/rschat/blob/master/src/server/mod.rs" }' localhost:8080/shorten
```

### Redirection
Request GET on `/` with shortened URL
```shell
❯ curl -v localhost:8080/Wcrryb7uS
# or access to the URL with your web browser
```

## Expected Responses
### Successful case
You're getting `201 Created` for POST and `308 Permanent Redirect` with original location for GET
```shell
❯ curl -v -H "Content-Type: application/json" -d '{ "url": "https://github.com/err0rless/rschat/blob/master/src/server/mod.rs" }' localhost:8080/shorten
...
< HTTP/1.1 201 Created
...
{"short_url":"Wcrryb7uS"}

❯ curl -v localhost:8080/Wcrryb7uS
...
>
< HTTP/1.1 308 Permanent Redirect
< location: https://github.com/err0rless/rschat/blob/master/src/server/mod.rs
< content-length: 0
...
```

### URL is not registered
You're getting `404 Not Found`
```shell
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
- [x] Use our own EPOCH instead of Unix's for even shorter URLs
- [ ] Use persistent DB
