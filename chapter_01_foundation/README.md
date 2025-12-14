# 第一章：基礎打底 — Rust、Linux、整體觀念

## 本章目標

完成本章後，你將能夠：

1. **Rust 方面**
   - 理解並運用所有權、借用、生命週期來管理記憶體
   - 使用 `Result` 和 `?` 優雅處理錯誤
   - 用 `Arc`、`Mutex`、`mpsc` 撰寫多執行緒程式
   - 理解 async/await 的基本概念

2. **Linux 方面**
   - 熟練使用 `ps`、`top`、`htop`、`grep` 等基本指令
   - 使用 `strace` 觀察程式的 system call
   - 理解 `/proc` 虛擬檔案系統的結構

3. **整合能力**
   - 能從「系統層面」觀察你寫的程式行為
   - 建立「程式碼 ↔ OS 行為」的連結思維

---

## 章節結構

```
chapter_01_foundation/
├── README.md                 # 本文件
├── 01_rust_fundamentals/     # Rust 核心概念
│   ├── theory.md            # 理論講解
│   ├── lab_01_mini_cat/     # 實作：mini cat/grep
│   └── lab_02_parallel_sum/ # 實作：平行計算
├── 02_linux_basics/          # Linux 環境
│   ├── theory.md            # 理論講解
│   ├── lab_03_strace/       # 實作：strace 觀察
│   └── lab_04_mini_ps/      # 實作：mini ps
└── checkpoint.md             # 階段檢核點
```

---

## 學習路線圖

```
Week 1-2: Rust 強化
┌─────────────────────────────────────────────────────────┐
│  Day 1-3: 所有權 / 借用 / 生命週期                        │
│  Day 4-5: 錯誤處理 (Result / ?)                          │
│  Day 6-7: Lab 1 - 實作 mini cat/grep                    │
│  Day 8-10: Arc / Mutex / mpsc                           │
│  Day 11-12: Lab 2 - 平行計算                            │
│  Day 13-14: async 基礎概念                              │
└─────────────────────────────────────────────────────────┘

Week 3: Linux 環境熟悉
┌─────────────────────────────────────────────────────────┐
│  Day 15-16: 基本指令 + strace 概念                       │
│  Day 17-18: Lab 3 - 用 strace 觀察程式                  │
│  Day 19-20: /proc 檔案系統                              │
│  Day 21: Lab 4 - 實作 mini ps                          │
└─────────────────────────────────────────────────────────┘
```

---

## 快速開始

1. 先閱讀 `01_rust_fundamentals/theory.md`
2. 完成 Lab 1 和 Lab 2
3. 再閱讀 `02_linux_basics/theory.md`
4. 完成 Lab 3 和 Lab 4
5. 用 `checkpoint.md` 檢核自己的學習成果

---

## 環境需求

- Rust 1.70+（建議用 rustup 安裝）
- Linux 環境（原生 Linux、WSL2、或 Docker）
- 編輯器：VS Code + rust-analyzer 推薦
