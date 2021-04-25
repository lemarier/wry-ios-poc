#[cfg(not(target_os = "android"))]
use cacao::ios::{
    App, AppDelegate, Scene, SceneConfig, SceneConnectionOptions, SceneSession, Window,
    WindowSceneDelegate,
};
use cacao::layout::{Layout, LayoutConstraint};
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

impl WebViewDelegate for WebViewInstance {}

pub struct RootView {
    pub webview: WebView<WebViewInstance>,
}

impl RootView {
    pub fn new() -> Self {
        RootView {
            webview: WebView::with(WebViewConfig::default(), WebViewInstance::default()),
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
        self.load_url("https://tauri.studio/");
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
