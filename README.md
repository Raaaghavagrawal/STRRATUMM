# 🛡️ STRRATUMM: AI-Overlay Backend

[![CI/CD Pipeline](https://github.com/Raaaghavagrawal/STRRATUMM/actions/workflows/ci.yml/badge.svg)](https://github.com/Raaaghavagrawal/STRRATUMM/actions/workflows/ci.yml)
[![Docker Registry](https://img.shields.io/badge/Container-GHCR-blue)](https://github.com/Raaaghavagrawal/STRRATUMM/pkgs/container/ai-overlay-backend)

**STRRATUMM** is a high-performance, hybrid application that combines a **Tauri-powered Desktop Overlay** with a **Rust/Axum Backend API**. It's designed to provide seamless AI integration (Gemini 1.5 Flash) with advanced "Stealth Mode" capabilities for bypass scenarios.

---

## ✨ Features

- **🚀 Hybrid Architecture:** Runs as a native Desktop App (Tauri) or a standalone Headless API Server (Docker).
- **🤖 Gemini AI Integration:** Leverages Google's Gemini 1.5 Flash for fast, intelligent responses.
- **☁️ AWS Cloud Ready:** Optimized Docker image for seamless deployment on EC2 (specifically Debian-based).
- **🕵️ Stealth Mode:** Includes a browser bypass script and window affinity settings for hidden operation.
- **📡 REST API & Swagger:** Built-in Swagger documentation at `/docs` for testing and integration.
- **🔄 Robust CI/CD:** Automated builds, linting, formatting, and Docker registry (GHCR) integration.

---

## 🛠️ Tech Stack

- **Frontend:** Vanilla HTML, CSS, JavaScript (Vanilla JS for speed and lightweight footprint).
- **Backend Core:** [Rust](https://www.rust-lang.org/)
- **API Framework:** [Axum](https://github.com/tokio-rs/axum) (Tokio/Hyper based)
- **Desktop Framework:** [Tauri 2.0](https://tauri.app/)
- **AI Engine:** Google Gemini SDK (via `reqwest`).
- **Deployment:** [Docker](https://www.docker.com/) via GitHub Container Registry (GHCR).

---

## 🚀 Getting Started

### Prerequisites

1.  **Rust:** `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2.  **Node.js:** (Recommended for Tauri CLI management)
3.  **GEMINI_API_KEY:** Obtain from [Google AI Studio](https://aistudio.google.com/).

### 1. Local Development (Desktop Mode)

Clone the repository and install dependencies:

```bash
cd STRRATUMM
npm install
```

Start the application:

```bash
npm run tauri dev
```

### 2. Run API Server Only (Docker)

If you strictly want the backend API running (e.g., on a VPS or EC2):

1. **Pull the latest image:**
   ```bash
   docker pull ghcr.io/raaaghavagrawal/ai-overlay-backend:latest
   ```

2. **Run it:**
   ```bash
   docker run -d \
     -p 3001:3001 \
     -e GEMINI_API_KEY="your_api_key_here" \
     -e SERVER_ONLY="true" \
     --name ai-backend \
     ghcr.io/raaaghavagrawal/ai-overlay-backend:latest
   ```

3. **Verify:**
   Visit `http://localhost:3001/health` or `http://localhost:3001/docs`.

---

## ☁️ Deployment & CI/CD

This project uses **GitHub Actions** (`ci.yml`) to:

- ✅ **Lint & Format:** Check Rust code quality.
- 🧪 **Unit Test:** Run core library tests.
- 🏗️ **Multi-Stage Build:** Compile the binary on Debian Bookworm for guaranteed GLIBC compatibility.
- 📤 **Push to GHCR:** Automatically push versioned tags (`latest`, `v2`, `git-sha`) to GitHub PKGS.

### Docker Cache-Busting
We've implemented a custom `CACHE_BUST` build-arg in the pipeline. If your binary isn't updating on EC2, the CI/CD will force a fresh rebuild from source code to ensure compatibility.

---

## 📁 Project Structure

```text
STRRATUMM/
├── .github/workflows/  # CI/CD Pipeline
├── src/                # Frontend (HTML/CSS/JS)
├── src-tauri/          # Rust Backend & Tauri config
│   ├── src/
│   │   ├── main.rs     # Entry point & Server logic
│   │   └── lib.rs      # AI interaction & Tauri commands
│   └── Dockerfile      # Production-grade Docker config (Multi-stage)
├── test-pipeline.ps1   # Local CI verification script
└── README.md           # This file!
```

---

## 🛡️ Important Security Note

- **API Security:** The backend currently binds to `0.0.0.0` for Docker accessibility. Ensure you use an AWS Security Group or Firewall to restrict port `3001` to your own IP address if deploying publicly.
- **Stealth:** The application uses `SetWindowDisplayAffinity` on Windows to prevent standard screen-sharing software from capturing the overlay window.

---

## 📝 License

This project is private and intended for internal use.
