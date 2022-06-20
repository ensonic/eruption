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

    Copyright (c) 2019-2022, The Eruption Development Team
*/

use std::fmt::Write;
use std::{fs, path::Path};

use chrono::Utc;

use crate::mapping::{Action, Event, KeyMappingTable, Source};

pub type Result<T> = std::result::Result<T, eyre::Error>;

#[derive(Debug)]
pub struct LuaBackend {}

impl LuaBackend {
    pub fn new() -> Self {
        Self {}
    }
}

impl super::Backend for LuaBackend {
    fn generate(&self, table: &KeyMappingTable) -> Result<String> {
        let mut text = String::new();

        write!(
            &mut text,
            r"
-- This file is part of Eruption.
--
-- Eruption is free software: you can redistribute it and/or modify
-- it under the terms of the GNU General Public License as published by
-- the Free Software Foundation, either version 3 of the License, or
-- (at your option) any later version.
--
-- Eruption is distributed in the hope that it will be useful,
-- but WITHOUT ANY WARRANTY without even the implied warranty of
-- MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
-- GNU General Public License for more details.
--
-- You should have received a copy of the GNU General Public License
-- along with Eruption.  If not, see <http://www.gnu.org/licenses/>.
--
-- Copyright (c) 2019-2022, The Eruption Development Team
--
-- AUTO GENERATED LUA SOURCE CODE FILE, DO NOT EDIT MANUALLY
--
-- Created by: `eruption-keymap compile {}`
-- Compiled at: {}

",
            table.file_name().display(),
            Utc::now()
        )?;

        for (_index, (source, action)) in table.mappings().iter().enumerate() {
            writeln!(&mut text, "-- {:?} -> {:?}", source, action)?;

            match action {
                Action::Null => {
                    /* do nothing */
                    writeln!(&mut text, "-- ACTION IS DISABLED")?;
                }

                //REMAPPING_TABLE = {} -- level 1 remapping table (No modifier keys applied)
                //MACRO_TABLE = {} -- level 1 macro table (No modifier keys applied)

                //MOUSE_HID_REMAPPING_TABLE = {} -- level 1 remapping table for mouse events (No modifier keys applied)

                //ACTIVE_EASY_SHIFT_LAYER = 1 -- level 4 supports up to 6 sub-layers
                //EASY_SHIFT_REMAPPING_TABLE = { -- level 4 remapping table (Easy Shift+ layer)
                //{}, {}, {}, {}, {}, {}
                //}
                //EASY_SHIFT_MACRO_TABLE = { -- level 4 macro table (Easy Shift+ layer)
                //{}, {}, {}, {}, {}, {}
                //}

                //EASY_SHIFT_MOUSE_DOWN_MACRO_TABLE =
                //{ -- macro tables for mouse button down events (Easy Shift+ layer)
                //{}, {}, {}, {}, {}, {}
                //}
                //EASY_SHIFT_MOUSE_UP_MACRO_TABLE =
                //{ -- macro tables for mouse button up events (Easy Shift+ layer)
                //{}, {}, {}, {}, {}, {}
                //}
                //EASY_SHIFT_MOUSE_HID_DOWN_MACRO_TABLE =
                //{ -- macro tables for mouse (HID) button down events (Easy Shift+ layer)
                //{}, {}, {}, {}, {}, {}
                //}
                //EASY_SHIFT_MOUSE_HID_UP_MACRO_TABLE =
                //{ -- macro tables for mouse (HID) button up events (Easy Shift+ layer)
                //{}, {}, {}, {}, {}, {}
                //}
                //EASY_SHIFT_MOUSE_WHEEL_MACRO_TABLE =
                //{ -- macro tables for mouse wheel events (Easy Shift+ layer)
                //{}, {}, {}, {}, {}, {}
                //}
                //EASY_SHIFT_MOUSE_DPI_MACRO_TABLE =
                //{ -- macro tables for mouse DPI change events (Easy Shift+ layer)
                //{}, {}, {}, {}, {}, {}
                //}
                //
                Action::InjectKey(dest) => {
                    match &source.event {
                        Event::Null => {
                            /* do nothing */
                            writeln!(&mut text, "-- ACTION IS DISABLED")?;
                        }

                        Event::HidKeyDown(_key) => {
                            writeln!(&mut text, "-- ACTION IS NOT IMPLEMENTED")?;

                            //writeln!(&mut text, "{} {}", key.key_index, dest.key_index)?;
                        }

                        Event::HidKeyUp(_key) => {
                            writeln!(&mut text, "-- ACTION IS NOT IMPLEMENTED")?;

                            //writeln!(&mut text, "{} {}", key.key_index, dest.key_index)?;
                        }

                        Event::HidMouseDown(key) => {
                            writeln!(
                                &mut text,
                                "MOUSE_HID_REMAPPING_TABLE[{}] = {}",
                                key.key_index, dest.key_index
                            )?;
                        }

                        Event::HidMouseUp(_key) => {
                            writeln!(&mut text, "-- ACTION IS NOT IMPLEMENTED")?;

                            //writeln!(
                            //&mut text,
                            //"MOUSE_HID_REMAPPING_TABLEi[{}] = {}",
                            //key.key_index, dest.key_index
                            //)?;
                        }

                        Event::EasyShiftKeyDown(key) => {
                            for layer in &source.layers.0 {
                                writeln!(
                                    &mut text,
                                    "EASY_SHIFT_REMAPPING_TABLE[{}][{}] = {}",
                                    layer + 1,
                                    key.key_index,
                                    dest.key_index
                                )?;
                            }
                        }

                        Event::EasyShiftKeyUp(_key) => {
                            writeln!(&mut text, "-- ACTION IS NOT IMPLEMENTED")?;
                        }

                        Event::EasyShiftMouseDown(_key) => {
                            writeln!(&mut text, "-- ACTION IS NOT IMPLEMENTED")?;
                        }

                        Event::EasyShiftMouseUp(_key) => {
                            writeln!(&mut text, "-- ACTION IS NOT IMPLEMENTED")?;
                        }

                        Event::EasyShiftMouseWheel(_direction) => {}

                        Event::SimpleKeyDown(key) => {
                            writeln!(
                                &mut text,
                                "REMAPPING_TABLE[{}] = {}",
                                key.key_index, dest.key_index
                            )?;
                        }

                        Event::SimpleKeyUp(key) => {
                            writeln!(&mut text, "{} {}", key.key_index, dest.key_index)?;
                        }

                        Event::SimpleMouseDown(key) => {
                            writeln!(&mut text, "{} {}", key.key_index, dest.key_index)?;
                        }

                        Event::SimpleMouseUp(key) => {
                            writeln!(&mut text, "{} {}", key.key_index, dest.key_index)?;
                        }

                        Event::SimpleMouseWheel(direction) => {
                            //writeln!(&mut text, "{} {}", key.key_index, dest.key_index)?;
                        }
                    }
                }

                Action::Call(call) => {
                    match &source.event {
                        Event::Null => {
                            /* do nothing */
                            writeln!(&mut text, "-- ACTION IS DISABLED")?;
                        }

                        Event::HidKeyDown(key) => {
                            writeln!(&mut text, "-- ACTION IS NOT IMPLEMENTED")?;

                            //writeln!(&mut text, "{} {}", key.key_index, dest.key_index)?;
                        }

                        Event::HidKeyUp(key) => {
                            writeln!(&mut text, "-- ACTION IS DISABLED")?;

                            //writeln!(&mut text, "{} {}", key.key_index, dest.key_index)?;
                        }

                        Event::HidMouseDown(key) => {
                            //writeln!(
                            //&mut text,
                            //"MOUSE_HID_REMAPPING_TABLEi[{}] = {}",
                            //key.key_index, dest.key_index
                            //)?;
                        }

                        Event::HidMouseUp(_key) => {
                            writeln!(&mut text, "-- ACTION IS NOT IMPLEMENTED")?;

                            //writeln!(
                            //&mut text,
                            //"MOUSE_HID_REMAPPING_TABLEi[{}] = {}",
                            //key.key_index, dest.key_index
                            //)?;
                        }

                        Event::EasyShiftKeyDown(key) => {
                            for layer in &source.layers.0 {
                                writeln!(
                                    &mut text,
                                    "EASY_SHIFT_MACRO_TABLE[{}][{}] = {}",
                                    layer + 1,
                                    key.key_index,
                                    call.function_name
                                )?;
                            }
                        }

                        Event::EasyShiftKeyUp(_key) => {
                            writeln!(&mut text, "-- ACTION IS DISABLED")?;
                        }

                        Event::EasyShiftMouseDown(key) => {
                            for layer in &source.layers.0 {
                                writeln!(
                                    &mut text,
                                    "EASY_SHIFT_MOUSE_DOWN_MACRO_TABLE[{}][{}] = {}",
                                    layer + 1,
                                    key.key_index,
                                    call.function_name
                                )?;
                            }
                        }

                        Event::EasyShiftMouseUp(key) => {
                            for layer in &source.layers.0 {
                                writeln!(
                                    &mut text,
                                    "EASY_SHIFT_MOUSE_UP_MACRO_TABLE[{}][{}] = {}",
                                    layer + 1,
                                    key.key_index,
                                    call.function_name
                                )?;
                            }
                        }

                        Event::EasyShiftMouseWheel(_direction) => {
                            for layer in &source.layers.0 {
                                writeln!(
                                    &mut text,
                                    "EASY_SHIFT_MOUSE_WHEEL_MACRO_TABLE[{}] = {}",
                                    layer + 1,
                                    call.function_name
                                )?;
                            }
                        }

                        Event::SimpleKeyDown(_key) => {
                            writeln!(&mut text, "-- ACTION IS DISABLED")?;
                        }

                        Event::SimpleKeyUp(_key) => {
                            writeln!(&mut text, "-- ACTION IS DISABLED")?;
                        }

                        Event::SimpleMouseDown(key) => {
                            //writeln!(&mut text, "{} {}", key.key_index, dest.key_index)?;
                        }

                        Event::SimpleMouseUp(key) => {
                            //writeln!(&mut text, "{} {}", key.key_index, dest.key_index)?;
                        }

                        Event::SimpleMouseWheel(direction) => {
                            //writeln!(&mut text, "{} {}", key.key_index, dest.key_index)?;
                        }
                    }
                }
            }

            // insert a newline
            writeln!(&mut text)?;
        }

        Ok(text)
    }

    fn write_to_file<P: AsRef<Path>>(&self, path: P, table: &KeyMappingTable) -> Result<()> {
        let text = self.generate(table)?;

        fs::write(&path, &text)?;

        Ok(())
    }
}
