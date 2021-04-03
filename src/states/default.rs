use crate::*;

/// The default state of the game. Where the gameplay happens.
pub struct DefaultState;

#[allow(unused_variables)]
impl minigene::State for DefaultState {
    fn update(&mut self, world: &mut World, dispatcher: &mut Dispatcher, ctx: &mut BTerm) -> Trans {
        #[cfg(not(feature = "headless"))]
        {
            ctx.set_active_console(0);
            ctx.cls();
            #[cfg(feature = "opengl")]
            {
                ctx.set_active_console(1);
                ctx.cls();
            }

            // create fake smaller camera to avoid rendering through the
            // UI at the right side of the screen.
            let cur_cam = world.get::<Camera>().unwrap();
            let mut tweaked_cam = cur_cam.clone();
            tweaked_cam.size.x -= cur_cam.position.x;
            tweaked_cam.size.y -= cur_cam.position.y;

            #[cfg(not(feature = "opengl"))]
            {
                ctx.set_active_console(0);
                render(ctx);
                render_ascii(
                    ctx,
                    &tweaked_cam,
                    &*world.get().unwrap(),
                    &*world.get().unwrap(),
                    &*world.get().unwrap(),
                );
            }
            #[cfg(feature = "opengl")]
            {
                ctx.set_active_console(1);
                render_sprites(
                    ctx,
                    &tweaked_cam,
                    &*world.get().unwrap(),
                    &*world.get().unwrap(),
                    Some(&*world.get().unwrap()),
                );
            }
            ctx.set_active_console(0);
            render_ui(world, ctx);
        }

        // Update mouse info
        {
            if let Ok(mut m) = world.get_mut::<Mouse>() {
                m.pos = ctx.mouse_pos();
                m.left_click = ctx.left_click;
            }
        }

        Trans::None
    }
}
