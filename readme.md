# naro3

なろう第 3 回 第 998244353 回

http://naro3.tqk.trap.show/

- `GET /` : "Hello, World!"
- `GET /ping`: "pong"
- `GET /hello/{name}` : "Hello, {name}!"
- `GET /fizzbuzz?count={count}` : 0 より大きく count までの fizzbuzz
- `POST /post` :
  - json のうちから id: int, name: string, arr: array<int> を受け取り、それらのみの json
- `POST /add` :
  - json から left: int, right: int 受け取って、和 answer: int だけを入れた json
