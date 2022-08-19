# naro3

web エンジニアになろう！(n 回目)
server practice
Showcase Rust actix-web sqlx

http://naro3.tqk.trap.show/

- `GET /` : "Hello, World!"
- `GET /ping`: "pong"
- `GET /hello/{name}` : "Hello, {name}!"
- `POST /signup`: 登録
```
{
    "username": "tqk",
    "password": "wMQnbeznUn8N"
}
```
- `POST /login`: ログイン
```
{
    "username": "tqk",
    "password": "wMQnbeznUn8N"
}
```
- `GET /hello`: "Hello, {username}!", ログインしていればそのユーザーネーム、していなければゲスト
- `GET /fizzbuzz?count={count}` : [1, count] の fizzbuzz
- `GET /city/{name}`: 名前が name の都市の情報
- `POST /post` :
  - 一部切り取ってそのまま返す 意味はない arr: [int]
```
{
    "id": 123,
    "name": "ddd",
		"arr": [998244353,998244353,998244353,998244353,-1]
}
```
- `POST /add` :
  - 足して返す
```
{
    "left": 123,
    "right": 678
}
```
```
```
{
    "answer": 801
}
```
```
- `POST /postcity`:
  - json から name: string, countryCode: string, district,: string, population: int 受け取って、データベースに追加する
```
{
  	"name": "Ooookayama",
    "country_code": "JPN",
    "district": "Tokyo",
    "population": 998244353
}
```
