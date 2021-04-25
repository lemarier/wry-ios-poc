#[cfg(not(target_os = "android"))]
use cacao::ios::{
    App, AppDelegate, Scene, SceneConfig, SceneConnectionOptions, SceneSession, Window,
    WindowSceneDelegate,
};
use cacao::{layout::{Layout, LayoutConstraint}, webview::InjectAt};
use cacao::view::{View, ViewController, ViewDelegate};
use cacao::webview::{WebView, WebViewConfig, WebViewDelegate};
use mobile_entry_point::mobile_entry_point;
use std::sync::RwLock;

#[derive(Default)]
struct TestApp;

impl AppDelegate for TestApp {
    fn config_for_scene_session(
        &self,
        session: SceneSession,
        _options: SceneConnectionOptions,
    ) -> SceneConfig {
        SceneConfig::new("Default Configuration", session.role())
    }
}

#[derive(Default)]
pub struct WebViewInstance;

impl WebViewDelegate for WebViewInstance {
    fn on_message(&self, name: &str, body: &str) {
        println!("name: {}, body: {}", name, body);
    }

    fn on_custom_protocol_request(&self, path: &str) -> Option<Vec<u8>> {

        let requested_asset_path = path.replace("wry.dev://", "");

        let index_html = r#"
        <!DOCTYPE html>
        <html lang="en">
            <head>
            <meta charset="UTF-8" />
            <meta http-equiv="X-UA-Compatible" content="IE=edge" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            </head>
            <body style="background-color: #7FFFD4">
            <h1>Welcome 🍫</h1>
            <a href="/hello.html">Link</a>
            </body>
        </html>"#; 
        
        let link_html = r#"
        <!DOCTYPE html>
        <html lang="en">
            <head>
            <meta charset="UTF-8" />
            <meta http-equiv="X-UA-Compatible" content="IE=edge" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            </head>
            <body>
            <h1>Hello!</h1>
            <a href="/">Back home</a>
            </body>
        </html>"#; 

        return match requested_asset_path.as_str() {
            "/hello.html" => Some(link_html.as_bytes().into()),
            _ => Some(index_html.as_bytes().into()),
        }
    }
}

pub struct RootView {
    pub webview: WebView<WebViewInstance>,
}

impl RootView {
    pub fn new() -> Self {
        let mut config = WebViewConfig::default();

        config.add_handler("notify");
        config.add_custom_protocol("wry.dev");

        let script = r#"
            window.addEventListener('DOMContentLoaded', (event) => {
                webkit.messageHandlers.notify.postMessage("hello");
            });
        "#;

       config.add_user_script(script, InjectAt::Start, true);
        
        RootView {
            webview: WebView::with(config, WebViewInstance::default()),
        }
    }

    pub fn load_url(&self, url: &str) {
        self.webview.load_url(url);
    }
}

impl ViewDelegate for RootView {
    const NAME: &'static str = "RootView";


    fn did_load(&mut self, view: View) {
        view.add_subview(&self.webview);
        self.load_url("wry.dev://");
        LayoutConstraint::activate(&[
            self.webview.top.constraint_equal_to(&view.top),
            self.webview
                .leading
                .constraint_equal_to(&view.leading),
            self.webview
                .trailing
                .constraint_equal_to(&view.trailing),
            // view height
            self.webview.height.constraint_equal_to_constant(900.),
        ]);
    }
}

#[derive(Default)]
pub struct WindowScene {
    pub window: RwLock<Option<Window>>,
    pub root_view_controller: RwLock<Option<ViewController<RootView>>>,
}

impl WindowSceneDelegate for WindowScene {

    fn will_connect(&self, scene: Scene, session: SceneSession, options: SceneConnectionOptions) {
        let bounds = scene.get_bounds();
        let mut window = Window::new(bounds);
        window.set_window_scene(scene);

        let root_view_controller = ViewController::new(RootView::new());
        window.set_root_view_controller(&root_view_controller);
        window.show();

        {
            let mut w = self.window.write().unwrap();
            *w = Some(window);

            let mut vc = self.root_view_controller.write().unwrap();
            *vc = Some(root_view_controller);
        }
    }
}

#[cfg(target_os = "android")]
fn init_logging() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_min_level(log::Level::Trace)
            .with_tag("test-cargo-mobile"),
    );
}

#[cfg(not(target_os = "android"))]
fn init_logging() {
    simple_logger::SimpleLogger::new().init().unwrap();
}

#[mobile_entry_point]
fn main() {
    init_logging();
    App::new(TestApp::default(), || Box::new(WindowScene::default())).run();
}
