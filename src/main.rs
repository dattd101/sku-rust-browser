#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod navigation;

use navigation::{resolve, HOME_URL};
use tao::{
    dpi::{LogicalPosition, LogicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};
use wry::{Rect, WebViewBuilder};

const TOOLBAR_HEIGHT: f64 = 48.0;

#[derive(Debug)]
enum BrowserEvent {
    Command(String),
    UrlChanged(String),
}

fn bounds(x: f64, y: f64, width: f64, height: f64) -> Rect {
    Rect {
        position: LogicalPosition::new(x, y).into(),
        size: LogicalSize::new(width.max(1.0), height.max(1.0)).into(),
    }
}

fn configure_cloudflare_dns() {
    // WebView2 reads this before its browser process is initialized.
    // Preserve any flags supplied by the launcher or the user.
    const DOH_FLAGS: &str = "--dns-over-https-mode=secure --dns-over-https-templates=https://cloudflare-dns.com/dns-query";
    let existing = std::env::var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS")
        .unwrap_or_default();
    let flags = if existing.trim().is_empty() {
        DOH_FLAGS.to_owned()
    } else {
        format!("{existing} {DOH_FLAGS}")
    };
    std::env::set_var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS", flags);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    configure_cloudflare_dns();
    let event_loop = EventLoopBuilder::<BrowserEvent>::with_user_event().build();
    let window = WindowBuilder::new()
        .with_title("Minimal Browser")
        .with_inner_size(LogicalSize::new(1280.0, 800.0))
        .build(&event_loop)?;

    let size = window.inner_size().to_logical::<f64>(window.scale_factor());

    let command_proxy = event_loop.create_proxy();
    let toolbar = WebViewBuilder::new()
        .with_html(include_str!("../assets/toolbar.html"))
        .with_bounds(bounds(0.0, 0.0, size.width, TOOLBAR_HEIGHT))
        .with_ipc_handler(move |request| {
            let _ = command_proxy.send_event(BrowserEvent::Command(request.body().to_owned()));
        })
        .build_as_child(&window)?;

    let page_proxy = event_loop.create_proxy();
    let browser = WebViewBuilder::new()
        .with_url(HOME_URL)
        .with_bounds(bounds(
            0.0,
            TOOLBAR_HEIGHT,
            size.width,
            size.height - TOOLBAR_HEIGHT,
        ))
        .with_on_page_load_handler(move |_, url| {
            let _ = page_proxy.send_event(BrowserEvent::UrlChanged(url));
        })
        .build_as_child(&window)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::UserEvent(BrowserEvent::Command(command)) => {
                let result = match command.as_str() {
                    "back" => browser.evaluate_script("window.history.back();"),
                    "forward" => browser.evaluate_script("window.history.forward();"),
                    "reload" => browser.reload(),
                    "home" => browser.load_url(HOME_URL),
                    _ if command.starts_with("navigate:") => {
                        browser.load_url(&resolve(&command["navigate:".len()..]))
                    }
                    _ => Ok(()),
                };
                if let Err(error) = result {
                    eprintln!("Navigation failed: {error}");
                }
            }
            Event::UserEvent(BrowserEvent::UrlChanged(url)) => {
                if let Ok(encoded) = serde_json::to_string(&url) {
                    let _ = toolbar.evaluate_script(&format!("window.setAddress({encoded});"));
                }
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                let size = new_size.to_logical::<f64>(window.scale_factor());
                let _ = toolbar.set_bounds(bounds(0.0, 0.0, size.width, TOOLBAR_HEIGHT));
                let _ = browser.set_bounds(bounds(
                    0.0,
                    TOOLBAR_HEIGHT,
                    size.width,
                    size.height - TOOLBAR_HEIGHT,
                ));
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}
