# <p align="center"><img src="assets/Logo.png" alt="RouterAI Logo" width="220"/></p>

<h1 align="center">RouterAI — Native iOS Client (100% Pure Rust)</h1>

<p align="center">
  <strong>Hệ thống điều khiển & tự động hóa RouterAI tối tân trên thiết bị di động Apple iOS</strong><br>
  <em>Xây dựng 100% bằng ngôn ngữ lập trình Rust nguyên bản với Dioxus Mobile Framework</em>
</p>

<p align="center">
  <a href="https://github.com/zheedevVn/RustRouterAI/actions"><img src="https://img.shields.io/github/actions/workflow/status/zheedevVn/RustRouterAI/build-ipa.yml?branch=main&label=Build%20iOS%20IPA&style=flat-square" alt="CI Build"/></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Language-Rust%202021-DEA584?style=flat-square&logo=rust" alt="Rust 2021"/></a>
  <a href="https://dioxuslabs.com/"><img src="https://img.shields.io/badge/UI%20Framework-Dioxus%200.5-007ACC?style=flat-square" alt="Dioxus"/></a>
  <a href="https://www.apple.com/ios/"><img src="https://img.shields.io/badge/Platform-iOS%20%7C%20iPadOS%20(ARM64)-black?style=flat-square&logo=apple" alt="Apple iOS"/></a>
  <a href="https://ai.mmovn.space"><img src="https://img.shields.io/badge/Backend%20API-ai.mmovn.space-10B981?style=flat-square" alt="Backend API"/></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue?style=flat-square" alt="License MIT"/></a>
</p>

---

## 📖 Giới Thiệu (About RouterAI iOS)

**RouterAI iOS** là ứng dụng di động chính thức phục vụ hệ thống tự động hóa marketing, quét dữ liệu nhóm và rải tin thông minh trên Telegram. Khác biệt hoàn toàn so với các ứng dụng di động thông thường dùng React Native, Flutter hay Swift/Objective-C:

- **100% Pure Rust:** Toàn bộ tầng giao diện người dùng, xử lý logic, quản lý trạng thái, kết nối mạng và bộ mã hóa dữ liệu đều được viết bằng **Rust**, loại bỏ hoàn toàn các lớp thông dịch JavaScript hoặc máy ảo trung gian.
- **Biên dịch trực tiếp ra mã máy ARM64:** Ứng dụng chạy trực tiếp trên vi xử lý Apple Silicon (A-Series / M-Series) với tần số quét màn hình 120Hz ProMotion, độ trễ tiệm cận 0ms và mức tiêu thụ pin tối ưu nhất.
- **An toàn bộ nhớ (Memory Safety & Zero Cost Abstractions):** Loại bỏ triệt để các lỗi rò rỉ bộ nhớ (memory leaks), null pointer và data races trong môi trường đa luồng không đồng bộ (`tokio`).
- **Bảo mật mã nguồn nhị phân:** Mã nguồn được biên dịch dạng tệp thực thi nhị phân Mach-O native, có khả năng chống kỹ thuật dịch ngược (anti-reverse engineering) và bảo vệ an toàn các thuật toán nhạy cảm.

---

## 🌟 Tính Năng Nổi Bật

### 1. Quản Trị Hệ Thống & Bảng Điều Khiển (Dashboard)
- Hiển thị theo thời gian thực trạng thái kết nối máy chủ (`ai.mmovn.space`).
- Thống kê chi tiết: Số lượng tài khoản Telegram đang hoạt động, nhóm đã quét, chiến dịch đang chạy, tốc độ rải tin và hạn ngạch gói VIP.

### 2. Quản Lý Nguồn Tài Khoản Telegram (Accounts Hub)
- Thêm và kiểm soát danh sách tài khoản Telegram thông qua String Session hoặc số điện thoại.
- Giám sát trạng thái hoạt động: `Active`, `SpamBlock`, `FloodWait`, `Expired`.
- Quản trị Proxy gán riêng cho từng tài khoản, giảm thiểu nguy cơ bị giới hạn tài khoản.

### 3. Bộ Máy Quét Nhóm Tự Động (Group Crawler & Router)
- Quét và lọc thông minh danh sách hội nhóm Telegram theo từ khóa mục tiêu.
- Phân loại tự động nhóm cho phép nhắn tin tự do và nhóm có kiểm duyệt (captchas / bot guard).
- Điều hướng luồng tin nhắn tự động theo kịch bản tùy biến.

### 4. Chiến Dịch Rải Tin Thông Minh (Campaign Engine)
- Thiết lập kịch bản rải tin với khoảng cách giãn dòng ngẫu nhiên (Smart Delay Jitter).
- Quay vòng nội dung (Spintax) và tự động thay thế biến thể để vượt qua bộ lọc tin nhắn rác của Telegram.

### 5. Cổng Xác Thực Đa Dạng (Authentication & Security)
- Đăng nhập / Đăng ký truyền thống với thuật toán mã hóa mật khẩu `bcrypt`.
- Hỗ trợ các cổng đăng nhập xã hội qua OAuth2: Google, GitHub, Discord.
- Tích hợp đăng nhập nhanh thông qua Telegram Widget Auth.

### 6. Giao Diện Tối Chuẩn iOS (Fluid Dark Theme)
- Thiết kế riêng cho màn hình OLED với tông nền đen thuần Super Retina.
- Tương thích 100% với Dynamic Island, Notch (tai thỏ) và Home Bar theo tiêu chuẩn Apple Human Interface Guidelines (`safe-area-inset`).

---

## 🏗️ Kiến Trúc Hệ Thống (Architecture)

```
┌────────────────────────────────────────────────────────┐
│                   RouterAI iOS App                     │
├────────────────────────────────────────────────────────┤
│  Presentation Layer: Dioxus 0.5 (RSX & Signals)        │
│  - Views: Login, Dashboard, Accounts, Groups, ...      │
│  - Components: TopHeader, BottomNavBar, SVG Icons      │
│  - Theme: Safe-Area CSS, ProMotion 120Hz Animation     │
├────────────────────────────────────────────────────────┤
│  State & Business Logic: Pure Rust                     │
│  - Models: Strongly-typed Serde Structs                │
│  - Async Runtime: Tokio & Async-Channel                │
├────────────────────────────────────────────────────────┤
│  Networking & Transport: Reqwest + Rustls              │
│  - REST API Client: Bearer Token Auth                  │
│  - TLS 1.3 Encryption: Rustls-native-certs            │
└───────────────────────────┬────────────────────────────┘
                            │ HTTPS / REST API
                            ▼
┌────────────────────────────────────────────────────────┐
│         RouterAI Backend (ai.mmovn.space)              │
│         - Account Verification & Health Checks         │
│         - Group Crawling & Discovery Workers           │
│         - Campaign Dispatcher & Proxy Rotation         │
└────────────────────────────────────────────────────────┘
```

---

## 🛠️ Ngăn Xếp Công Nghệ (Tech Stack)

| Thành phần | Công nghệ / Thư viện | Mục đích |
|---|---|---|
| **Ngôn ngữ** | Rust 2021 Edition | Ngôn ngữ cốt lõi đảm bảo an toàn bộ nhớ và hiệu năng cao nhất |
| **Giao diện** | Dioxus 0.5 Mobile | Framework UI phản ứng (Reactive RSX & Signals) native cho iOS |
| **Mạng HTTP** | Reqwest 0.11 + Rustls | Client HTTP bất đồng bộ hỗ trợ TLS 1.3 không cần OpenSSL |
| **Xử lý bất đồng bộ** | Tokio 1.x + Async-Channel | Runtime điều phối tác vụ đa luồng tốc độ cao |
| **Tuần tự hóa dữ liệu** | Serde + Serde JSON | Chuyển đổi dữ liệu JSON từ API với zero-copy parsing |
| **Định dạng ứng dụng** | Apple Mach-O ARM64 (`.ipa`) | Tệp nhị phân native tương thích iOS 14.0 trở lên |

---

## 📂 Cấu Trúc Thư Mục Dự Án

```
RustRouterAI/
├── .github/
│   └── workflows/
│       └── build-ipa.yml       # Tự động hóa CI/CD biên dịch & đóng gói file .ipa trên macOS runner
├── assets/                     # Tài nguyên hình ảnh, biểu tượng ứng dụng chuẩn 1024x1024
│   ├── Logo.png
│   ├── Logo-Small.png
│   └── icon_1024.png
├── src/
│   ├── main.rs                 # Điểm khởi chạy ứng dụng iOS Dioxus & điều hướng tab
│   ├── api/
│   │   ├── client.rs           # REST API client bất đồng bộ kết nối máy chủ ai.mmovn.space
│   │   └── mod.rs
│   ├── models/
│   │   └── mod.rs              # Structs dữ liệu chuẩn (User, Account, Group, Campaign, Plan)
│   ├── theme/
│   │   └── mod.rs              # Hệ thống CSS Dark Theme chuẩn hóa với iOS Safe Areas
│   ├── components/
│   │   ├── icons.rs            # Bộ Icon SVG vector sắc nét độc lập
│   │   └── mod.rs              # TopHeader, BottomNavBar, thanh công cụ
│   └── views/
│       ├── accounts.rs         # Màn hình quản lý nguồn tài khoản Telegram
│       ├── campaigns.rs        # Màn hình quản trị chiến dịch gửi tin tự động
│       ├── dashboard.rs        # Màn hình tổng quan số liệu thống kê hệ thống
│       ├── groups.rs           # Màn hình quét & duyệt danh sách nhóm Telegram
│       ├── login.rs            # Màn hình đăng nhập, đăng ký và xác thực OAuth
│       ├── plans.rs            # Màn hình nâng cấp gói thành viên VIP
│       ├── settings.rs         # Màn hình cấu hình API và tùy chỉnh ứng dụng
│       └── mod.rs
├── Cargo.toml                  # Khai báo dependency Rust và Bundle Metadata iOS
├── Dioxus.toml                 # Cấu hình đóng gói di động Dioxus Mobile
└── README.md                   # Tài liệu hướng dẫn chi tiết dự án
```

---

## ⚡ Tự Động Xuất File `.ipa` Qua GitHub Actions

Dự án đã được tích hợp sẵn luồng **CI/CD hoàn toàn tự động** trên runner `macos-14`:

1. Mỗi khi có commit mới đẩy lên nhánh `main`, tiến trình **`Build iOS IPA (Rust)`** sẽ tự động kích hoạt.
2. Hệ thống sẽ tự động cài đặt Rust Toolchain, target `aarch64-apple-ios`, biên dịch mã nguồn với cờ tối ưu `--release`.
3. Đóng gói đầy đủ tệp thực thi Mach-O, `Info.plist` và `assets` vào tệp tiêu chuẩn `RouterAI.ipa`.
4. Để tải về:
   - Truy cập tab **[Actions](https://github.com/zheedevVn/RustRouterAI/actions)** trên repository.
   - Nhấp vào phiên chạy workflow gần nhất.
   - Cuộn xuống mục **Artifacts** và tải về gói **`RouterAI-iOS-Package`**.

---

## 📲 Hướng Dẫn Cài Đặt File `.ipa` Lên iPhone / iPad

Sau khi tải file `RouterAI.ipa`, bạn có thể cài đặt dễ dàng lên thiết bị iOS bằng một trong các phương thức sau:

### Cách 1: Cài đặt trực tiếp trên điện thoại (Không cần máy tính)
- **TrollStore (Khuyên dùng):** Dành cho các thiết bị và phiên bản iOS được hỗ trợ (iOS 14.0 - 17.0 tùy dòng máy). Cài đặt vĩnh viễn không bao giờ bị thu hồi chứng chỉ (Never Revoked).
- **Scarlet / Esign / GBox:** Sử dụng chứng chỉ doanh nghiệp (Enterprise Certificate) hoặc DNS chặn thu hồi để ký và cài file `.ipa` trực tiếp trong trình duyệt Safari.

### Cách 2: Ký qua máy tính (PC Windows hoặc Mac)
- **Sideloadly (Miễn phí, dễ dùng nhất):**
  1. Tải và cài đặt [Sideloadly](https://sideloadly.io/) trên máy tính.
  2. Kết nối iPhone/iPad với máy tính qua cáp Lightning/Type-C.
  3. Kéo thả file `RouterAI.ipa` vào giao diện Sideloadly.
  4. Nhập Apple ID của bạn và nhấn **Start**.
  5. Sau khi hoàn tất, trên iPhone vào **Cài đặt** -> **Cài đặt chung** -> **Quản lý VPN & Thiết bị** -> Tin cậy chứng chỉ nhà phát triển Apple ID của bạn.
- **AltStore:** Tương tự như Sideloadly, hỗ trợ tự động gia hạn chứng chỉ 7 ngày qua mạng Wi-Fi cục bộ.

---

## 💻 Hướng Dẫn Biên Dịch Cục Bộ (Local Build)

### Điều kiện tiên quyết:
- Cài đặt **Rust & Cargo**:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- Cài đặt target iOS:
  ```bash
  rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
  ```
- Kiểm tra cú pháp và logic:
  ```bash
  cargo check
  ```
- Chạy thử nghiệm trên máy tính (Desktop/Simulator mode):
  ```bash
  cargo run
  ```

---

## 🔒 Giấy Phép & Bản Quyền (License)

Dự án được phân phối dưới giấy phép [MIT License](LICENSE).  
Copyright © 2021-2026 **RouterAI Team** (`support@mmovn.space`). Mọi quyền được bảo lưu.
