# Lab 5: Streaming HTTP (Rust)

## 實作目標

> 用 Rust 實作 HTTP 串流回應，觀察文字逐段輸出

完成後你將學會：
- HTTP 串流回應的概念（chunked）
- 使用 Axum 回傳 streaming body
- 用 SSE（Server-Sent Events）傳送事件流

---

## 預期功能

```bash
# 逐段輸出（chunked）
$ curl -N http://localhost:8080/stream
chunk 0
chunk 1
chunk 2
...

# SSE（事件流）
$ curl -N http://localhost:8080/sse
data: token 0

data: token 1

data: token 2
...
```

---

## 階段一：Chunked Streaming

### 任務
1. 建立 `/stream` 路由
2. 用 interval 產生 10 段資料（每 200ms 一段）
3. 回傳 streaming body（不要設定 Content-Length）

### 提示
- `StreamBody` 會自動使用 `Transfer-Encoding: chunked`
- `curl -N` 可以避免客戶端緩衝

---

## 階段二：SSE

### 任務
1. 建立 `/sse` 路由
2. 回傳 `text/event-stream`
3. 每 400ms 推送一個事件（共 10 個）

### 提示
- 使用 `axum::response::sse::{Sse, Event}`
- SSE 每筆資料格式為 `data: ...\n\n`

---

## 驗收標準

- [ ] `/stream` 可以看到逐段輸出（非一次回傳）
- [ ] `/sse` 事件可以持續推送
- [ ] 使用 `curl -N` 時能即時看到資料

---

## 起始程式碼

查看 `src/main.rs` 取得起始程式碼框架。
