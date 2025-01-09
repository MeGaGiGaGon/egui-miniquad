use {egui_miniquad as egui_mq, egui_miniquad::miniquad as mq, egui_miniquad::egui};

struct Stage {
    egui_mq: egui_mq::EguiMq,
    mq_ctx: Box<dyn mq::RenderingBackend>,
}

impl Stage {
    fn new() -> Self {
        let mut mq_ctx = mq::window::new_rendering_backend();

        Self {
            egui_mq: egui_mq::EguiMq::new(&mut *mq_ctx),
            mq_ctx,
        }
    }
}

impl mq::EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        self.mq_ctx.clear(Some((1., 1., 1., 1.)), None, None);
        self.mq_ctx
            .begin_default_pass(mq::PassAction::clear_color(0.0, 0.0, 0.0, 1.0));
        self.mq_ctx.end_render_pass();

        // Run the UI code:
        self.egui_mq.run(&mut *self.mq_ctx, |_mq_ctx, ctx| {
            
        });

        // Draw things behind egui here

        self.egui_mq.draw(&mut *self.mq_ctx);

        // Draw things in front of egui here

        self.mq_ctx.commit_frame();
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(x, y);
    }

    fn mouse_wheel_event(&mut self, dx: f32, dy: f32) {
        self.egui_mq.mouse_wheel_event(dx, dy);
    }

    fn mouse_button_down_event(&mut self, mb: mq::MouseButton, x: f32, y: f32) {
        self.egui_mq.mouse_button_down_event(mb, x, y);
    }

    fn mouse_button_up_event(&mut self, mb: mq::MouseButton, x: f32, y: f32) {
        self.egui_mq.mouse_button_up_event(mb, x, y);
    }

    fn char_event(&mut self, character: char, _keymods: mq::KeyMods, _repeat: bool) {
        self.egui_mq.char_event(character);
    }

    fn key_down_event(&mut self, keycode: mq::KeyCode, keymods: mq::KeyMods, _repeat: bool) {
        self.egui_mq.key_down_event(keycode, keymods);
    }

    fn key_up_event(&mut self, keycode: mq::KeyCode, keymods: mq::KeyMods) {
        self.egui_mq.key_up_event(keycode, keymods);
    }
}

fn main() {

    let conf = mq::conf::Conf {
        high_dpi: true,
        window_width: 1200,
        window_height: 1024,
        ..Default::default()
    };
    mq::start(conf, || Box::new(Stage::new()));
}
