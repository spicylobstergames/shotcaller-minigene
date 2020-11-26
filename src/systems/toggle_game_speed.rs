use crate::*;

event_reader_res!(ToggleGameSpeedRes, InputEvent);
system!(
    ToggleGameSpeedSystem,
    |events: Read<'a, EventChannel<InputEvent>>,
     res: WriteExpect<'a, ToggleGameSpeedRes>,
     speed: Write<'a, GameSpeed>| {
        for k in events.read(&mut res.0) {
            if k == &InputEvent::SpeedToggle {
                if speed.0 == 1.0 {
                    speed.0 = 2.0;
                } else {
                    speed.0 = 1.0;
                }
            }
        }
    }
);
