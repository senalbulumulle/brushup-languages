use ui_framework::render;
use ui_framework::window::AppWindow;

fn main() {
    let mut app = AppWindow::new("ui-framework", 800, 600).expect("failed to open window");

    while !app.should_close() {
        app.poll_events();
        if app.drain_events() {
            app.request_close();
        }

        render::clear(0.10, 0.10, 0.10, 1.0);
        app.swap_buffers();
    }
}
