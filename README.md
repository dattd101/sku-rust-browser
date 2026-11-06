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
