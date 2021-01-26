use crate::*;

/// The default state of the game. Where the gameplay happens.
pub struct DefaultState;

impl minigene::State for DefaultState {
    fn update(
        &mut self,
        _world: &mut World,
        _dispatcher: &mut Dispatcher,
        _ctx: &mut BTerm,
    ) -> Trans {
        #[cfg(not(feature = "headless"))]
        {
            _ctx.set_active_console(0);
            _ctx.cls();
            #[cfg(feature = "opengl")]
            {
                _ctx.set_active_console(1);
                _ctx.cls();
            }
            #[cfg(not(feature = "opengl"))]
            {
                _ctx.set_active_console(0);
                render(_ctx);
                render_ascii(
                    _ctx,
                    &*_world.get().unwrap(),
                    &*_world.get().unwrap(),
                    &*_world.get().unwrap(),
                    &*_world.get().unwrap(),
                );
            }
            #[cfg(feature = "opengl")]
            {
                _ctx.set_active_console(1);
                render_sprites(
                    _ctx,
                    &*_world.get().unwrap(),
                    &*_world.get().unwrap(),
                    &*_world.get().unwrap(),
                );
            }
            _ctx.set_active_console(0);
            render_ui(_world, _ctx);
        }
        Trans::None
    }  
}
