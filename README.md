# http-server in Rust

[Rustで簡易HTTPサーバーを作成してみる](https://qiita.com/sogrnwil/items/42fd032999b39f595324)を参考にhttpサーバーを書いてみる。

## How to Use

サーバを起動

```sh
❯ ./main
```

アクセスする

```
❯ curl localhost:8080/index.html
HTTP1.1 200 OK
Content-Type: text/html; charset=UTF-8
Content-Length: 43

<html><body>
  Hello world!!
</body></html>
```

## References
* [Rustで簡易HTTPサーバーを作成してみる](https://qiita.com/sogrnwil/items/42fd032999b39f595324)