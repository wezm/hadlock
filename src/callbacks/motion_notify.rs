use crate::windowmanager::WindowManager;
use crate::xlibwrapper::core::*;
use crate::xlibwrapper::event::*;
use crate::xlibwrapper::util::Position;
use crate::xlibwrapper::masks::*;
use std::rc::Rc;

pub fn motion_notify(_xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {

    let (w, x_root, y_root, state) =
        match event.event_type {
            EventType::MotionNotify => match event.payload {
                Some(EventPayload::MotionNotify(w, x_root, y_root, state)) => (w, x_root, y_root, state),
                _ => { return; }
            },
            _ => { return; }
        };

    if !wm.clients.contains_key(&w) {
        return;
    }

    let drag_pos = Position { x: x_root, y: y_root };
    let (delta_x, delta_y) =  (drag_pos.x - wm.drag_start_pos.0,
                               drag_pos.y - wm.drag_start_pos.1);
    let dest_pos = Position{ x: wm.drag_start_frame_pos.0 + delta_x,
    y: wm.drag_start_frame_pos.1 + delta_y};

    if (state & (Button1Mask | Mod4Mask)) == Button1Mask | Mod4Mask {
        wm.move_window(w, dest_pos.x, dest_pos.y);
    }
}
