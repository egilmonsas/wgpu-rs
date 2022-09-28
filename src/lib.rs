#[cfg(target_arch = "wasm32")]
use tracing_subscriber::fmt::format::Pretty;
#[cfg(target_arch = "wasm32")]
use tracing_subscriber::fmt::time::UtcTime;
#[cfg(target_arch = "wasm32")]
use tracing_subscriber::prelude::*;
#[cfg(target_arch = "wasm32")]
use tracing_web::{performance_layer, MakeConsoleWriter};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
                std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            let fmt_layer= tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_timer(UtcTime::rfc_3339())
                .with_writer(MakeConsoleWriter);
            let perf_layer = performance_layer()
                .with_details_from_fields(Pretty::default());
            tracing_subscriber::registry()
                .with(fmt_layer)
                .with(perf_layer)
                .init();

        }
    }
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("WGPU Demo")
        .build(&event_loop)
        .unwrap();
    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(450, 400));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.body()?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    });
}
