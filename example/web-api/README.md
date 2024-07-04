### How to run server?
```sh
$ cargo run
```

```sh
curl --location 'http://127.0.0.1:3000/users' \
--header 'Content-Type: application/json' \
--data '{
    "username": "sahil"
}'
```