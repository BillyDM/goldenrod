use baseview::{Event, Window, WindowHandler, WindowScalePolicy};
use futures::executor::block_on;
use goldenrod::{Renderer, Size};

struct HelloWorldExample {
    renderer: Renderer,
}

impl HelloWorldExample {
    pub fn build(window: &mut Window) -> Self {
        let physical_size = Size::new(
            window.window_info().physical_size().width as f32,
            window.window_info().physical_size().height as f32,
        );

        let renderer = block_on(Renderer::new(
            window,
            physical_size,
            window.window_info().scale(),
        ))
        .unwrap();

        Self { renderer }
    }
}

impl WindowHandler for HelloWorldExample {
    type Message = ();

    fn on_frame(&mut self) {
        self.renderer.render();
    }

    fn on_event(&mut self, _window: &mut Window, event: Event) {
        match event {
            Event::Mouse(e) => {}
            Event::Keyboard(e) => {}
            Event::Window(e) => match e {
                baseview::WindowEvent::Resized(window_info) => {
                    let physical_size = Size::new(
                        window_info.physical_size().width as f32,
                        window_info.physical_size().height as f32,
                    );

                    self.renderer.resize(physical_size, window_info.scale());
                }
                _ => {}
            },
        }
    }

    fn on_message(&mut self, _window: &mut Window, _message: Self::Message) {}
}

fn main() {
    let window_open_options = baseview::WindowOpenOptions {
        title: "baseview".into(),
        size: baseview::Size::new(512.0, 512.0),
        scale: WindowScalePolicy::SystemScaleFactor,
        parent: baseview::Parent::None,
    };

    let handle = Window::open(window_open_options, HelloWorldExample::build);
    handle.app_run_blocking();
}
