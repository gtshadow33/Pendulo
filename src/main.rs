use ggez::{
    event, graphics, glam::*, input::keyboard::{KeyCode, KeyInput}, Context, GameResult,
};

struct Pendulo {
    longitud: f32,
    angulo: f32,
    velocidad: f32,
    gravedad: f32,
}

impl Pendulo {
    fn new() -> Self {
        Self {
            longitud: 400.0,
            angulo: std::f32::consts::FRAC_PI_4, // 45° en radianes
            velocidad: 0.0,
            gravedad: 9.81,
        }
    }

    fn actualizar(&mut self, dt: f32) {
        let aceleracion = -(self.gravedad / self.longitud) * self.angulo.sin();
        self.velocidad += aceleracion * dt;
        self.angulo += self.velocidad * dt;
    }
}

struct MainState {
    pendulo: Pendulo,
    angulo_inicial: f32,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        Ok(MainState {
            pendulo: Pendulo::new(),
            angulo_inicial: std::f32::consts::FRAC_PI_4,
        })
    }
}

impl event::EventHandler for MainState {
   
fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    let dt = ctx.time.delta().as_secs_f32(); // correcto en ggez 0.11+
    self.pendulo.actualizar(dt);
    Ok(())
}

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult<()> {
        let paso = std::f32::consts::PI / 180.0; // 1 grado en radianes

        if let Some(keycode) = input.keycode {
            match keycode {
                KeyCode::Equals | KeyCode::Plus => {
                    self.angulo_inicial += paso;
                    self.pendulo.angulo = self.angulo_inicial;
                    self.pendulo.velocidad = 0.0;
                }
                KeyCode::Minus => {
                    self.angulo_inicial -= paso;
                    self.pendulo.angulo = self.angulo_inicial;
                    self.pendulo.velocidad = 0.0;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from_rgb(20, 30, 30));

        let (cx, cy) = (600.0, 100.0);
        let x = cx + self.pendulo.longitud * self.pendulo.angulo.sin();
        let y = cy + self.pendulo.longitud * self.pendulo.angulo.cos();

        // Dibujar línea del péndulo
        let line = graphics::Mesh::new_line(
            ctx,
            &[Vec2::new(cx, cy), Vec2::new(x, y)],
            3.0,
            graphics::Color::WHITE,
        )?;
        canvas.draw(&line, graphics::DrawParam::default());

        // Dibujar bola
        let bola = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(x, y),
            20.0,
            0.1,
            graphics::Color::from_rgb(0, 150, 255),
        )?;
        canvas.draw(&bola, graphics::DrawParam::default());

        // Mostrar información de ángulo y velocidad
        let texto = graphics::Text::new(format!(
            "Ángulo inicial: {:.2} rad ({:.1}°)\nÁngulo: {:.2} rad ({:.1}°)\nVelocidad: {:.2} rad/s",
            self.angulo_inicial,
            self.angulo_inicial.to_degrees(),
            self.pendulo.angulo,
            self.pendulo.angulo.to_degrees(),
            self.pendulo.velocidad,
        ));
        canvas.draw(&texto, Vec2::new(20.0, 20.0));

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult<()> {
    let (ctx, event_loop) = ggez::ContextBuilder::new("pendulo", "gts")
        .window_setup(ggez::conf::WindowSetup::default().title("Simulador de Péndulo"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1200.0, 800.0))
        .build()?;

    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
