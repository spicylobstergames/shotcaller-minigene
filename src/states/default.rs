use crate::*;

pub struct DefaultState;

impl minigene::State for DefaultState {
    fn update(
        &mut self,
        world: &mut World,
        _dispatcher: &mut Dispatcher,
        ctx: &mut BTerm,
    ) -> Trans {
        ctx.set_active_console(0);
        ctx.cls();
        #[cfg(feature = "opengl")]
        {
            ctx.set_active_console(1);
            ctx.cls();
        }
        #[cfg(not(feature = "opengl"))]
        {
            ctx.set_active_console(0);
            render(ctx);
            render_ascii(
                ctx,
                &*world.get().unwrap(),
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
                &*world.get().unwrap(),
                &*world.get().unwrap(),
                &*world.get().unwrap(),
            );
        }
        ctx.set_active_console(0);
        render_ui(world, ctx);
        Trans::None
    }
}
