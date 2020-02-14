/*
    This file is part of Eruption.

    Eruption is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Eruption is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Eruption.  If not, see <http://www.gnu.org/licenses/>.
*/

use failure::Error;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::sync::Arc;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Event {
    KeyDown(u8),
    KeyUp(u8),
}

#[derive(Debug)]
pub enum EventClass {
    Keyboard,
}

pub type Callback = dyn Fn(&Event) -> Result<bool> + Sync + Send + 'static;

lazy_static! {
    pub static ref KEYBOARD_OBSERVERS: Arc<Mutex<Vec<Box<Callback>>>> =
        Arc::new(Mutex::new(vec![]));
}

pub fn register_observer<C>(event_class: EventClass, callback: C)
where
    C: Fn(&Event) -> Result<bool> + Sync + Send + 'static,
{
    match event_class {
        EventClass::Keyboard => KEYBOARD_OBSERVERS.lock().push(Box::from(callback)),
    }
}

pub fn notify_observers(event: Event) -> Result<()> {
    for callback in KEYBOARD_OBSERVERS.lock().iter() {
        callback(&event)?;
    }

    Ok(())
}
