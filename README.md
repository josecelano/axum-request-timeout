# Axum request timeout

I'm trying to set a request timeout for an Axum server.

This is a test repo to reproduce the problem with a minimal amount of code.

The original issue: <https://github.com/torrust/torrust-index-backend/issues/204>

I think there are two different timeouts:

## Timeout for the handlers

I think this is the discussion opened in the Axum repo:

<https://github.com/tokio-rs/axum/discussions/1383>

With the solution provided in the discussion, I can set a timeout for the handlers.
If the handler takes more than 5 seconds, the server will return a 500 error.

It's the current implementation in this repo. You can test it with:

```s
$ telnet localhost 3001
Trying 127.0.0.1...
Connected to localhost.
Escape character is '^]'.
GET / HTTP/1.0

HTTP/1.0 500 Internal Server Error
content-type: text/plain; charset=utf-8
content-length: 37
date: Mon, 19 Jun 2023 16:00:56 GMT

`GET /` failed with request timed outConnection closed by foreign host.
```

After 5 seconds, the server returns a 500 error. You have to press "return" twice to send the `GET / HTTP/1.0` request.

## Timeout for the client

ActixWeb provides a client request timeout. If you run a similar example but with ActixWeb you will see:

```s
$ telnet localhost 3001
Trying 127.0.0.1...
Connected to localhost.
Escape character is '^]'.
HTTP/1.1 408 Request Timeout
content-length: 0
connection: close
date: Mon, 19 Jun 2023 15:02:54 GMT

Connection closed by foreign host.
```

It will send a `408 Request Timeout` response if you do not send any request after 5 seconds.
It's the [default timeout](https://docs.rs/actix-web/latest/actix_web/struct.HttpServer.html#method.client_request_timeout) for the ActixWeb server.

I'm trying to do the same for Axum. The following could be a solution: <https://github.com/torrust/torrust-tracker/issues/324#issuecomment-1548360076>.

## Links

- [How to set timeouts? (GitHub discussion)](https://github.com/tokio-rs/axum/discussions/1383)
- <https://docs.rs/axum/latest/axum/error_handling/index.html#running-extractors-for-error-handling>
- <https://stackoverflow.com/questions/73758789/how-to-set-http-timeouts-using-axum-based-on-hyper>
- <https://github.com/torrust/torrust-tracker/issues/324#issuecomment-1548360076>
- [Tower issue - Should TimeoutLayer middleware allow returning other status codes?](https://github.com/tower-rs/tower-http/issues/300)
- [ActixWeb implementation](https://docs.rs/actix-web/latest/actix_web/struct.HttpServer.html#method.client_request_timeout)
