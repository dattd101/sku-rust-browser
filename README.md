# Minimal Browser (Rust + WebView2)

Minimal Windows desktop browser with a native window and two embedded WebView2 instances: a compact toolbar and a website view.

## Features

- Address bar with domain normalization and Google search fallback
- Back, Forward, Reload, Home
- Resizable window
- Embedded toolbar HTML (no external asset needed alongside EXE)
- GitHub Actions Windows x64 build with downloadable EXE artifact

## Build on GitHub Actions

1. Upload the project files to the root of a GitHub repository.
2. Open **Actions → Build Windows EXE → Run workflow** (or push to `main`/`master`).
3. After the job succeeds, download artifact **MinimalBrowser-Windows-x64**.
4. Extract `minimal-browser.exe` and run it on Windows with Microsoft Edge WebView2 Runtime installed.

## Notes

- This is an MVP, not a hardened browser for untrusted browsing. No tabs, downloads UI, or profile management.
- The toolbar's Ctrl+L shortcut works while the toolbar has focus; global shortcuts are not implemented.
- WebView2 Runtime is required on the target Windows machine.
- Source has not been compiled in this environment; GitHub Actions performs the actual Windows validation.

## Fix v0.1.1

Back/Forward use `evaluate_script("window.history.back();")` and `evaluate_script("window.history.forward();")` instead of unsupported Wry methods.


## Cloudflare DNS over HTTPS (experimental)

`src/main.rs` sets `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS` before creating WebView2 with Chromium flags `--dns-over-https-mode=secure` and `--dns-over-https-templates=https://cloudflare-dns.com/dns-query`. The flags are best-effort: the installed WebView2 Runtime, policy, and an already-running shared browser process may ignore them. This does not configure Windows system DNS, and it does not guarantee all DNS traffic uses Cloudflare. Test resolution and failure behavior on the target machine.
