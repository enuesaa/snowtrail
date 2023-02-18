use wry::{
    application::{
        dpi::LogicalSize,
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        menu::{MenuBar, MenuItem},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();

    let mut menu_bar = MenuBar::new();
    let mut utilities = MenuBar::new();
    utilities.add_native_item(MenuItem::Quit);
    utilities.add_native_item(MenuItem::Services);
    menu_bar.add_submenu("&AppName", true, utilities);

    let window = WindowBuilder::new()
        .with_title("snowtrail")
        .with_inner_size(LogicalSize::new(1500, 1000))
        .with_menu(menu_bar)
        .build(&event_loop)?;

    // To customize titlebar, see https://github.com/tauri-apps/wry/blob/d7c9097256/examples/custom_titlebar.rs
    let webview = WebViewBuilder::new(window)?
        .build()?;

    webview.load_url("https://yahoo.co.jp");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
