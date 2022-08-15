# naro3

web エンジニアになろう！(n 回目)
server practice
Showcase Rust actix-web sqlx

http://naro3.tqk.trap.show/

- `GET /` : "Hello, World!"
- `GET /ping`: "pong"
- `GET /hello/{name}` : "Hello, {name}!"
- `GET /fizzbuzz?count={count}` : 0 より大きく count までの fizzbuzz
- `GET /city/{name}`: 名前が name の都市の情報
- `POST /post` :
  - json のうちから id: int, name: string, arr: array<int> を受け取り、それらのみの json
- `POST /add` :
  - json から left: int, right: int 受け取って、和 answer: int だけを入れた json
- `POST /postcity`:
  - json から name: string, countryCode: string, district,: string, population: int 受け取って、データベースに追加する
