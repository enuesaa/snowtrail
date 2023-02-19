use wry::{
    application::{
        dpi::LogicalSize,
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        menu::{MenuBar, MenuItem},
        window::WindowBuilder,
        platform::macos::WindowExtMacOS,
    },
    webview::WebViewBuilder,
};

fn main() -> wry::Result<()> {
    enum UserEvent {
        Navigation(String),
    }

    let event_loop: EventLoop<UserEvent> = EventLoop::with_user_event();
    let proxy = event_loop.create_proxy();
    let mut histories: Vec<String> = Vec::new();

    let mut menu_bar = MenuBar::new();
    let mut utilities = MenuBar::new();
    utilities.add_native_item(MenuItem::Quit);
    utilities.add_native_item(MenuItem::Services);
    menu_bar.add_submenu("&AppName", true, utilities);

    let window = WindowBuilder::new()
        .with_title("snowtrail")
        .with_inner_size(LogicalSize::new(1500, 1000))
        .with_menu(menu_bar)
        // .with_visible(false)
        .build(&event_loop)?;
    // let window2 = WindowBuilder::new()
    //     .with_title("snowtrail2")
    //     .build(&event_loop)?;

    // window.set_tabbing_identifier("a");
    // window2.set_tabbing_identifier("a");

    // To customize titlebar, see https://github.com/tauri-apps/wry/blob/d7c9097256/examples/custom_titlebar.rs
    let webview = WebViewBuilder::new(window)?
        .with_back_forward_navigation_gestures(true)
        .with_navigation_handler(move |uri: String| {
            proxy.send_event(UserEvent::Navigation(uri.clone())).is_ok() || true
        })
        .build()?; 
    // let webview2 = WebViewBuilder::new(window2)?
    //     .with_back_forward_navigation_gestures(true)
    //     .build()?;

    webview.load_url("https://yahoo.co.jp");
    // webview2.load_url("https://yahoo.com");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("{:?}", histories);
                *control_flow = ControlFlow::Exit
            },
            Event::UserEvent(UserEvent::Navigation(uri)) => {
                histories.push(uri.clone())
            },
            _ => (),
        }
    });
}
