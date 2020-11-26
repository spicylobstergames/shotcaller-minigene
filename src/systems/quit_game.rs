use crate::*;

event_reader_res!(QuitGameRes, InputEvent);
system!(
    QuitGameSystem,
    |events: Read<'a, EventChannel<InputEvent>>,
     res: WriteExpect<'a, QuitGameRes>,
     quit: Write<'a, QuitGame>| {
        for k in events.read(&mut res.0) {
            if k == &InputEvent::Quit {
                quit.0 = true;
            }
        }
    }
);
