# naro3

なろう第 3 回 第 998244353 回

http://naro3.tqk.trap.show/

- `GET /` : "Hello, World!" を返す
- `GET /hello/{name}` : "Hello, {name}!" を返す
- `POST /post` :
  - json のうちから以下のものを受け取り、それらのみの json を返す
    - id: number
    - name: string
    - arr: array
  - 1 個以上のフィールドが足りないと 400
