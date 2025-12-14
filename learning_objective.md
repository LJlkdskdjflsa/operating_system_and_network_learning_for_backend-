**用「學習 + 實作」的方式，把 OS / Network 打成「可直接用在後端」的硬功**，而且偏好用 **Rust** 來寫。下面我幫你整理一份「分階段、分主題、每個主題都有：學什麼 ➜ 做什麼（Rust）」的詳細路線圖。

你可以把這當成一份長期 roadmap，照順序走也可以挑主題插隊。

---

## 🧭 全局架構：4 大階段

1. **基礎打底：Rust + Linux 基礎 + 整體觀念**
2. **OS 向：process / thread / memory / I/O（偏後端實務）**
3. **Network 向：TCP / HTTP / TLS + 實戰工具**
4. **綜合專案：高效 HTTP 服務 + 觀測 + 調優**

每一階段都會用「學習 ➜ 實作」兩條線來說。

---

## 1️⃣ 基礎打底：Rust、Linux、整體觀念

### 1.1 Rust 強化（偏後端會用到的能力）

**學習重點**

- 所需語言特性：
    - 所有權 / 借用 / lifetime
    - `Result` / `?` / error handling
    - trait / generics
    - `Arc` / `Mutex` / `RwLock` / `mpsc`
- async 基礎：
    - `Future` / `async` / `await`
    - 基本 runtime（Tokio）概念

**實作任務（Rust）**

1. **小 CLI 工具 – 模擬簡易版 `cat` / `grep`**
    - 功能：
        - 從檔案讀資料
        - 過濾關鍵字輸出
    - 練習點：
        - `std::fs::File`、buffered I/O
        - error handling (`Result`, `thiserror`/`anyhow`)
2. **小多執行緒程式 – 平行計數**
    - 做一個：給一個大範圍（1..N），用 thread pool 去加總。
    - 練習點：
        - `std::thread::spawn`
        - `Arc<Mutex<T>>` / channel 傳任務

---

### 1.2 Linux & 環境熟悉

**學習重點**

- 會用基本指令：
    - `ps`, `top`, `htop`, `ls`, `cat`, `less`, `grep`, `sed`, `awk`
- 跑 Rust 程式時會用：
    - `strace`：看 system call
    - `lsof`：看開了哪些 fd

**實作任務**

1. **寫個 Rust 程式，然後用 `strace` 看 system call**
    - 程式內容：讀檔案 + sleep + print
    - 用 `strace ./your_program`
    - 觀察：`open`, `read`, `write`, `nanosleep` 等 syscall
2. **寫個 Rust 版的小 `ps`（列出目前執行中的 process）**
    - 在 Linux 上讀 `/proc` 目錄：
        - 列出 PID、cmd line
    - 練習點：
        - 讀目錄、解析文字
        - 了解 Linux `/proc` 介面

---

## 2️⃣ OS 向：Process / Thread / Memory / I/O

這部分完全站在「後端會用到的 OS 知識」角度設計。

---

### 2.1 Process / Thread / Scheduling

**學習重點**

- 名詞與概念：
    - process vs thread vs coroutine（async task）
    - context switch 是什麼、為什麼會貴（cache / TLB）
    - kernel thread vs user thread
- Linux 相關：
    - `fork` / `exec`（概念上理解即可）
    - `top` / `htop` 中的 CPU 利用率 / load average

**實作任務（Rust）**

1. **多 process vs 多 thread 的簡單比較**
    
    （需在 Linux 上，用 `nix` crate 會比較方便）
    
    - 任務：大量計算，例如計算 1..N 的平方和，拆成多段並行。
    - 實作兩版：
        1. 多 process 版本：
            - 用 `nix::unistd::fork` 建幾個 process，各算一段，結果用 pipe 回 parent。
        2. 多 thread 版本：
            - 用 `std::thread` + `mpsc` 或 `Arc<Mutex<_>>`.
    - 觀察：
        - 用 `time` 看執行時間
        - 用 `htop` 看 process / thread 結構
        - 體會 process 與 thread 的差別
2. **簡易 Thread Pool**
    - 寫一個通用 thread pool：
        - 啟動固定數量 worker threads
        - 使用 channel（`std::sync::mpsc` 或 `crossbeam_channel`）投遞任務
    - 附帶一個 demo：用 thread pool 處理一堆「耗時任務」（sleep + 模擬計算）

---

### 2.2 Memory / 虛擬記憶體

**學習重點**

- 基本概念：
    - stack vs heap
    - virtual memory / page / page fault
    - memory layout（text / data / heap / stack）
- Rust 向：
    - 所有權 / 生存期 與記憶體釋放的關係
    - `Box`, `Vec`, `Rc` vs `Arc`

**實作任務（Rust）**

1. **模擬記憶體爆炸 / page fault 感覺**
    - 寫兩個版本的程式：
        1. **局部性好**：頻繁存取同一小段 array（例如 [0..4096]）
        2. **局部性差**：在超大 array（幾百 MB）中用大步長亂跳存取
    - 用 `time` 測 benchmark，觀察效能差異
    - 用 `perf stat`（或類似工具）看 cache miss / page fault（若可）
2. **簡單 memory pool**
    - 寫一個非常簡單的記憶體池管理：
        - 事先分配一大塊 `Vec<u8>`
        - 手動管理「區塊已用 / 未用」的 bitmap 或 free list
    - 讓某些小物件的分配改用 memory pool
    → 主要是練「思考記憶體配置」的能力

---

### 2.3 I/O & File Descriptor & Non-blocking

**學習重點**

- I/O 模型：
    - blocking I/O
    - non-blocking + `select` / `poll` / `epoll`
    - asynchronous I/O（Rust async runtime 本質）
- Linux 中的 fd：
    - socket、pipe、file 都是 fd
- 後端相關概念：
    - 大量連線 vs 大量資料量
    - 為什麼 Nginx 要用 epoll 而不是一連線一 thread

**實作任務（Rust）**

1. **Blocking echo server + client**
    - 使用 `std::net::{TcpListener, TcpStream}`：
        - server：每 accept 一個連線，就開一個 thread 處理（簡單 echo）
        - client：開多個 client 同時連線、送資料
    - 觀察：
        - thread 數量（`htop`）
        - 當連線數上去後，CPU 等資源使用情況
2. **Non-blocking + epoll 版本（進階）**
    - 使用 `mio` crate（低階事件驅動 I/O，Tokio 的底層之一）：
        - 將 socket 設成 non-blocking
        - 用 `Poll` + event loop 處理多連線
    - 目標不是寫超完整 server，而是體會：
        - 沒 thread per connection 也能處理大量連線
        - event-driven 程式結構
3. **Async 版本（Tokio）**
    - 用 `tokio::net::TcpListener` + `tokio::spawn` 重寫 echo server
    - 模擬高併發連線，對比 CPU / 記憶體 / latency

---

## 3️⃣ Network 向：TCP / HTTP / TLS + 工具實戰

---

### 3.1 TCP / UDP / 基礎網路

**學習重點**

- TCP：
    - 三次握手、四次揮手
    - RTT、超時 retransmission、流量控制
    - TIME_WAIT / CLOSE_WAIT 等狀態
- UDP：
    - 沒有連線、沒有重傳保證
- Linux 工具：
    - `ss -tulpn`, `netstat`, `ping`, `traceroute`

**實作任務（Rust）**

1. **簡單 TCP chat server**
    - 使用 `tokio`（或標準 blocking 也可）：
        - 支援多 client 連線
        - client 送訊息 → server 廣播給其他 client
    - 練習：
        - 維護連線列表
        - 處理連線中斷、錯誤
2. **簡單 UDP echo / 「猜數字」小遊戲**
    - 使用 `std::net::UdpSocket`
    - server：接到資料就回覆
    - client：發送請求 + 接收回應
    - 讓你感受「UDP 沒有連線」的行為
3. **用 `tcpdump` / `Wireshark` 看你自己的封包**
    - 跑剛才的 TCP / UDP 程式
    - 用 `sudo tcpdump -i lo port <port>` 抓本機流量
    - 在 Wireshark 裡：
        - 看到 TCP handshake / data / FIN
        - 在 UDP case 看到簡單 datagram

---

### 3.2 HTTP / HTTPS / REST 基礎

**學習重點**

- HTTP：
    - request / response 格式
    - method / status code / headers
    - keep-alive / connection pool
- HTTP/1.1 vs HTTP/2（概念即可）
- HTTPS / TLS：
    - 握手大致流程、cert 角色
    - 第一次連線 vs 之後連線延遲差異

**實作任務（Rust）**

1. **自己手刻一個超簡單 HTTP/1.1 server**
    - 不用框架，只用 `TcpListener`：
        - 解析 request line + header
        - 回固定的 response（例如回一個簡單 HTML 或 JSON）
    - 重點：親手 parse HTTP（不用太完整，先支援 GET）
2. **用現成 crate 重寫 – `hyper` 或 `axum`**
    - 用 `hyper` 或 `axum` 實作同樣的 API：
        - 支援 path param / query param
    - 對比：
        - 手刻 vs 用框架的開發效率
        - 但你會因為手刻過，比較懂框架底下在做什麼
3. **加上 HTTPS（TLS）**
    - 使用 `hyper` + `rustls` 或 `axum` + `tower`：
        - 建立一個 HTTPS server（可以用自簽憑證）
    - 用 `curl -vk <https://localhost>:port` 測試：
        - 觀察 TLS handshake / cert info

---

### 3.3 DNS / Routing / Proxy / Load Balancer（概念 + 小實作）

**學習重點**

- DNS：domain → IP，resolver / authoritative server
- Routing：IP forwarding的大致概念
- Reverse Proxy / Load Balancer：
    - L4 vs L7
    - 為什麼要用 Nginx / HAProxy / Envoy

**實作任務（Rust）**

1. **簡易 Reverse Proxy**
    - 寫一個小 server：
        - 接收 client 的 HTTP 請求
        - 轉送到後端真實 server（例如另一個簡單服務）
        - 把 response 原封不動回給 client
    - 可用 `hyper` 的 client + server API
    - 這會讓你理解：
        - 反向代理 / gateway 的實作本質
2. **簡易 Load Balancer（round-robin）**
    - 在 reverse proxy 基礎上支援多個 backend：
        - 用簡單的 round-robin 或 random 策略
    - 額外可以：
        - 實作 health check（定期請求 backend，看是否存活）

---

## 4️⃣ 綜合專案：實戰後端服務 + 觀測 + 調優

這一段是把你前面學的 OS + Network + Rust 整合起來，做一個「可展示」的專案。

---

### 4.1 實作一個「小型 REST 服務」(Rust)

**功能大致**

- 提供以下 API：
    - `POST /items`：創建資源
    - `GET /items/:id`：查詢
    - `GET /items`：列表（支援分頁）
- 資料存放：
    - 先用 in-memory（`Arc<RwLock<Vec<_>>>`）
    - 之後可接 PostgreSQL / SQLite（用 `sqlx`）

**技術重點**

- 使用 **Tokio + Axum 或 Actix-web**
- 支援：
    - JSON request/response
    - 結構化 error handling
    - logging（`tracing`）與基本 metrics（請求數、latency）

---

### 4.2 加入觀測 & 調優

**學習重點**

- logging：
    - 結構化 log（JSON log）
    - log level（info/debug/warn/error）
- metrics：
    - QPS
    - latency（平均 / p95 / p99）
    - 連線數、thread 數
- tracing：
    - request 的 trace id / span

**實作任務（Rust）**

1. **加入 `tracing` / `tracing-subscriber`**
    - 為每個 request 建立一個 span
    - log：
        - path
        - status code
        - latency
2. **加入 Prometheus metrics**
    - 用 `prometheus` 或 `metrics` crate
    - 暴露 `/metrics` endpoint
    - 指標：
        - `http_requests_total`
        - `http_request_duration_seconds_bucket`
        - `active_connections`
3. **壓測 + 分析**
    - 用 `wrk` / `hey` / `ab` 對你的服務壓測
    - 同時用：
        - `htop` 看 CPU / thread 數
        - `ss -s` 看 socket 情況
        - `strace -p <pid>` 掃一眼 system call 型態（`epoll_wait` / `read` / `write` 等）
    - 嘗試：
        - 調整 tokio worker 數量
        - 改變連線數 / keep-alive 設定
    - 觀察效能變化，寫下自己的結論（這會非常有價值）

---

### 4.3 延伸挑戰（可選）

如果你還有力氣往 OS / network 深一點走，可以挑一兩個：

1. **在你的服務裡實作簡單的「限流（rate limiting）」**
    - IP-based / token bucket
    - 理解背後跟 OS socket queue 的關聯
2. **簡單 Service Mesh / Sidecar prototype**
    - 做一個代理：
        - 負責加 tracing header / metrics
        - 真實服務只處理業務邏輯
3. **從 Blog OS 延伸出一點東西**
    - 在你寫的 Rust toy kernel 中：
        - 加一個簡單的 cooperative multitasking（tasks + scheduler）
    - 把「你在 Linux userland 學到的東西」投射回 kernel 視角