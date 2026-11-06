# Minimal Browser (Rust + WebView2)

Windows x64 minimal browser using `tao` and `wry`. Includes a simple HTML toolbar, navigation, and an experimental Cloudflare DoH Chromium flag configuration.

## Build on GitHub

Upload the **contents of this folder** to the repository root. GitHub Actions builds the EXE, runs Rust tests, attempts a Microsoft Defender scan, and generates a SHA-256 checksum. Download the artifact from the Actions run.

## Security and Chrome warnings

A successful Cargo build or Defender scan does not mean Chrome Safe Browsing will accept an unsigned, unfamiliar executable. The EXE is not code-signed. For public distribution, use an established code-signing identity and build reputation over time. Do not disable Chrome Safe Browsing or Windows Defender.

Defender PowerShell cmdlets and signature updates may be unavailable on some GitHub-hosted runners; in that case the scan step deliberately fails rather than claiming the file is clean.

## Cloudflare DNS

The application sets `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS` before initializing WebView2 with Chromium DoH flags. Whether those flags are honored depends on WebView2 Runtime and policy; this is **not** a guarantee that all DNS requests use Cloudflare. Test at https://1.1.1.1/help.

## Notes

Requires WebView2 Runtime on the target Windows machine. Source was reviewed, but this package was not compiled or scanned on Windows in this environment.
