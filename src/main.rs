#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod navigation;
mod storage;
use navigation::{resolve, HOME_URL};
use storage::Data;
use tao::{dpi::{LogicalPosition, LogicalSize}, event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoopBuilder, EventLoopProxy}, window::WindowBuilder};
use wry::{Rect, WebView, WebViewBuilder};

const TOOLBAR_HEIGHT: f64 = 82.0;
const MAX_TABS: usize = 10;
#[derive(Debug)]
enum BrowserEvent { Command(String), UrlChanged(usize, String) }
struct Tab { id: usize, url: String, view: WebView }
fn bounds(x:f64,y:f64,w:f64,h:f64)->Rect { Rect {position:LogicalPosition::new(x,y).into(),size:LogicalSize::new(w.max(1.0),h.max(1.0)).into()} }
fn configure_cloudflare_dns() {
    const FLAGS: &str = "--dns-over-https-mode=secure --dns-over-https-templates=https://cloudflare-dns.com/dns-query";
    let current = std::env::var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS").unwrap_or_default();
    std::env::set_var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",format!("{current} {FLAGS}"));
}
fn new_tab(window:&tao::window::Window, proxy:&EventLoopProxy<BrowserEvent>, id:usize, url:&str, adblock:bool, width:f64,height:f64)->Result<Tab, Box<dyn std::error::Error>> {
    let p = proxy.clone();
    let mut builder = WebViewBuilder::new()
        .with_url(url)
        .with_bounds(bounds(0.0,TOOLBAR_HEIGHT,width,height-TOOLBAR_HEIGHT))
        .with_on_page_load_handler(move |_,url| { let _ = p.send_event(BrowserEvent::UrlChanged(id,url)); });
    if adblock { builder = builder.with_initialization_script(include_str!("../assets/adblock.js")); }
    let view = builder.build_as_child(window)?;
    Ok(Tab {id,url:url.into(),view})
}
fn refresh(toolbar:&WebView,tabs:&[Tab],active:usize,data:&Data) {
    let tablist:Vec<_> = tabs.iter().map(|t|serde_json::json!({"id":t.id,"url":t.url})).collect();
    let state = serde_json::json!({"tabs":tablist,"active":active,"history":data.history,"bookmarks":data.bookmarks,"adblock":data.adblock,"limit":MAX_TABS});
    let _=toolbar.evaluate_script(&format!("window.renderState({state});"));
}
fn layout(tabs:&[Tab],active:usize,w:f64,h:f64) {
    for t in tabs {
        // Hide inactive child WebViews outside the window bounds; preserve their page state.
        let r=if t.id==active {bounds(0.0,TOOLBAR_HEIGHT,w,h-TOOLBAR_HEIGHT)} else {bounds(-20000.0,-20000.0,1.0,1.0)};
        let _=t.view.set_bounds(r);
    }
}
fn main()->Result<(),Box<dyn std::error::Error>> {
    configure_cloudflare_dns();
    let event_loop=EventLoopBuilder::<BrowserEvent>::with_user_event().build();
    let window=WindowBuilder::new().with_title("Minimal Browser").with_inner_size(LogicalSize::new(1280.0,800.0)).build(&event_loop)?;
    let proxy=event_loop.create_proxy();
    let sz=window.inner_size().to_logical::<f64>(window.scale_factor());
    let toolbar_proxy=proxy.clone();
    let toolbar=WebViewBuilder::new().with_html(include_str!("../assets/toolbar.html"))
        .with_bounds(bounds(0.0,0.0,sz.width,TOOLBAR_HEIGHT))
        .with_ipc_handler(move |req| {let _=toolbar_proxy.send_event(BrowserEvent::Command(req.body().to_owned()));})
        .build_as_child(&window)?;
    let mut data=Data::load();
    let mut next_id=1usize;
    let mut active=0usize;
    let mut tabs=vec![new_tab(&window,&proxy,active,HOME_URL,data.adblock,sz.width,sz.height)?];
    refresh(&toolbar,&tabs,active,&data);
    event_loop.run(move |event,_,flow| {
        *flow=ControlFlow::Wait;
        match event {
            Event::UserEvent(BrowserEvent::Command(cmd)) => {
                let current=tabs.iter().position(|t|t.id==active);
                let mut rebuild=false;
                match cmd.as_str() {
                    "new" if tabs.len()<MAX_TABS => {
                        let sz=window.inner_size().to_logical::<f64>(window.scale_factor());
                        let id=next_id; next_id+=1;
                        match new_tab(&window,&proxy,id,HOME_URL,data.adblock,sz.width,sz.height) {
                            Ok(t)=>{tabs.push(t);active=id;rebuild=true;}, Err(e)=>eprintln!("New tab failed: {e}")
                        }
                    }
                    "back"=>{if let Some(i)=current {let _=tabs[i].view.evaluate_script("history.back()");}}
                    "forward"=>{if let Some(i)=current {let _=tabs[i].view.evaluate_script("history.forward()");}}
                    "reload"=>{if let Some(i)=current {let _=tabs[i].view.reload();}}
                    "home"=>{if let Some(i)=current {let _=tabs[i].view.load_url(HOME_URL);}}
                    "bookmark"=>{if let Some(i)=current {data.toggle_bookmark(&tabs[i].url);}}
                    "clear-history"=>{data.history.clear();data.save();}
                    "adblock-toggle"=>{
                        data.adblock=!data.adblock;data.save();
                        // Initialization scripts are configured per WebView; recreate tabs to apply.
                        let urls:Vec<_>=tabs.iter().map(|t|(t.id,t.url.clone())).collect();
                        tabs.clear();
                        let sz=window.inner_size().to_logical::<f64>(window.scale_factor());
                        for (id,url) in urls {if let Ok(t)=new_tab(&window,&proxy,id,&url,data.adblock,sz.width,sz.height){tabs.push(t);}}
                        if !tabs.iter().any(|t|t.id==active) {if let Some(t)=tabs.first(){active=t.id;}}
                        rebuild=true;
                    }
                    _ if cmd.starts_with("switch:")=>{
                        if let Ok(id)=cmd[7..].parse::<usize>() {if tabs.iter().any(|t|t.id==id){active=id;rebuild=true;}}
                    }
                    _ if cmd.starts_with("close:")=>{
                        if let Ok(id)=cmd[6..].parse::<usize>() {
                            if tabs.len()>1 {tabs.retain(|t|t.id!=id);if active==id {active=tabs.last().map(|t|t.id).unwrap_or(active);}rebuild=true;}
                        }
                    }
                    _ if cmd.starts_with("navigate:")=>{if let Some(i)=current {let _=tabs[i].view.load_url(&resolve(&cmd[9..]));}}
                    _=>{}
                }
                if rebuild {let sz=window.inner_size().to_logical::<f64>(window.scale_factor());layout(&tabs,active,sz.width,sz.height);}
                refresh(&toolbar,&tabs,active,&data);
            }
            Event::UserEvent(BrowserEvent::UrlChanged(id,url))=>{
                if let Some(t)=tabs.iter_mut().find(|t|t.id==id) {
                    t.url=url.clone();data.visit(&url);refresh(&toolbar,&tabs,active,&data);
                }
            }
            Event::WindowEvent{event:WindowEvent::Resized(sz),..}=>{
                let sz=sz.to_logical::<f64>(window.scale_factor());
                let _=toolbar.set_bounds(bounds(0.0,0.0,sz.width,TOOLBAR_HEIGHT));
                layout(&tabs,active,sz.width,sz.height);
            }
            Event::WindowEvent{event:WindowEvent::CloseRequested,..}=>*flow=ControlFlow::Exit,
            _=>{}
        }
    });
}
