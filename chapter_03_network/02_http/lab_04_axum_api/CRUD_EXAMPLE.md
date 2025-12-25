# CRUD Examples (Axum Items API)

Base URL:
`http://localhost:8080`

## Read (List)
```bash
curl http://localhost:8080/items
```

## Create
```bash
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{"name":"Widget","price":9.99}' \
  http://localhost:8080/items
```



## Read (Single)
```bash
curl http://localhost:8080/items/1
```

## Update
```bash
curl -X PUT \
  -H "Content-Type: application/json" \
  -d '{"name":"Super Widget","price":19.99}' \
  http://localhost:8080/items/1
```

## Delete
```bash
curl -X DELETE http://localhost:8080/items/1
```
