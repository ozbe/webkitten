mod application;
mod webview;
mod window;

use std::fs::File;
use std::io::Read;
use webkitten::ui::ApplicationUI;
use webkitten::Engine;

use cocoa::base::{id,nil};
use block::ConcreteBlock;
use webkit::*;
use runtime::log_error_description;

pub struct CocoaUI {
    pub engine: Engine
}

impl CocoaUI {

    fn compile_content_extensions<F>(&self, completion: F)
        where F: Fn(bool) + 'static {
        if let Some(mut file) = self.content_filter_path().and_then(|p| File::open(p).ok()) {
            let mut contents = String::new();
            if let Some(_) = file.read_to_string(&mut contents).ok() {
                unsafe {
                    let block = ConcreteBlock::new(move |_: id, err: id| {
                        log_error_description(err);
                        completion(err == nil);
                    });
                    let store = _WKUserContentExtensionStore::default_store(nil);
                    store.compile_content_extension("filter", &contents, &block.copy());
                }
            }
        }
    }

    fn content_filter_path(&self) -> Option<&str> {
	    self.engine.config
            .lookup("general.content-filter")
            .and_then(|value| value.as_str())
    }
}

impl ApplicationUI for CocoaUI {

    fn new(engine: Engine) -> Option<Self> {
        Some(CocoaUI { engine: engine })
    }

    fn event_handler(&self) -> &Engine {
       &self.engine
    }

    fn run(&self) {
        self.compile_content_extensions(|_| {});
        self.open_window(self.engine.config.lookup_str("window.start-page"));
        application::start_run_loop();
    }

    fn open_window(&self, uri: Option<&str>) {
        window::open(uri);
    }

    fn close_window(&self, index: u8) {
        window::close(index);
    }

    fn focused_window_index(&self) -> u8 {
        window::focused_index()
    }

    fn focus_window(&self, index: u8) {
        window::focus(index);
    }

    fn window_count(&self) -> u8 {
        window::window_count()
    }

    fn toggle_window(&self, window_index: u8, visible: bool) {
        window::toggle(window_index, visible);
    }

    fn resize_window(&self, window_index: u8, width: u32, height: u32) {
        window::resize(window_index, width, height);
    }

    fn address_field_text(&self, window_index: u8) -> String {
        window::address_field_text(window_index)
    }

    fn set_address_field_text(&self, window_index: u8, text: &str) {
        window::set_address_field_text(window_index, text);
    }

    fn command_field_text(&self, window_index: u8) -> String {
        window::command_field_text(window_index)
    }

    fn set_command_field_text(&self, window_index: u8, text: &str) {
        window::set_command_field_text(window_index, text);
    }

    fn window_title(&self, window_index: u8) -> String {
        window::title(window_index)
    }

    fn set_window_title(&self, window_index: u8, title: &str) {
        window::set_title(window_index, title);
    }

    fn focused_webview_index(&self, window_index: u8) -> u8 {
        window::focused_webview_index(window_index)
    }

    fn webview_count(&self, window_index: u8) -> u8 {
        window::webview_count(window_index)
    }

    fn open_webview(&self, window_index: u8, uri: &str) {
        window::open_webview(window_index, uri);
    }

    fn close_webview(&self, window_index: u8, webview_index: u8) {
        window::close_webview(window_index, webview_index);
    }

    fn focus_webview(&self, window_index: u8, webview_index: u8) {
        window::focus_webview(window_index, webview_index);
    }

    fn set_uri(&self, window_index: u8, webview_index: u8, uri: &str) {
        info!("Setting URI");
        if let Some(webview) = window::webview(window_index, webview_index) {
            info!("Loading URI: {}", uri);
            webview::load_uri(webview, uri);
        }
    }

    fn go_back(&self, window_index: u8, webview_index: u8) -> bool {
        if let Some(webview) = window::webview(window_index, webview_index) {
            webview::go_back(webview)
        } else {
            false
        }
    }

    fn go_forward(&self, window_index: u8, webview_index: u8) -> bool {
        if let Some(webview) = window::webview(window_index, webview_index) {
            webview::go_forward(webview)
        } else {
            false
        }
    }

    fn uri(&self, window_index: u8, webview_index: u8) -> String {
        window::webview(window_index, webview_index)
            .and_then(|webview| Some(webview::uri(webview)))
            .unwrap_or(String::new())
    }

    fn webview_title(&self, window_index: u8, webview_index: u8) -> String {
        window::webview(window_index, webview_index)
            .and_then(|webview| Some(webview::title(webview)))
            .unwrap_or(String::new())
    }

    fn run_javascript(&self, window_index: u8, webview_index: u8, script: &str) {
        if let Some(webview) = window::webview(window_index, webview_index) {
            webview::run_javascript(webview, script)
        }
    }

    fn apply_styles(&self, window_index: u8, webview_index: u8, styles: &str) {
        if let Some(webview) = window::webview(window_index, webview_index) {
            webview::apply_styles(webview, styles);
        }
    }
}
