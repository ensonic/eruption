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

use lazy_static::lazy_static;

// type Result<T> = std::result::Result<T, eyre::Error>;

// #[derive(Debug, thiserror::Error)]
// pub enum DeviceError {
//     #[error("Unknown device")]
//     UnknownDevice {},
// }

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    make: &'static str,
    model: &'static str,
    usb_vid: u16,
    usb_pid: u16,
}

lazy_static! {
    #[rustfmt::skip]
    pub static ref DEVICE_INFO: &'static [DeviceInfo; 15] = &[
        DeviceInfo { make: "ROCCAT", model: "Vulcan 100/12x",       usb_vid: 0x1e7d, usb_pid: 0x3098, },
        DeviceInfo { make: "ROCCAT", model: "Vulcan 100/12x",       usb_vid: 0x1e7d, usb_pid: 0x307a, },

        DeviceInfo { make: "ROCCAT", model: "Vulcan Pro",           usb_vid: 0x1e7d, usb_pid: 0x30f7, },

        DeviceInfo { make: "ROCCAT", model: "Vulcan TKL",           usb_vid: 0x1e7d, usb_pid: 0x2fee, },

        DeviceInfo { make: "ROCCAT", model: "Vulcan Pro TKL",       usb_vid: 0x1e7d, usb_pid: 0x311a, },

        DeviceInfo { make: "Corsair", model: "Corsair STRAFE Gaming Keyboard", usb_vid: 0x1b1c, usb_pid: 0x1b15, },

        DeviceInfo { make: "ROCCAT", model: "Kone Aimo",            usb_vid: 0x1e7d, usb_pid: 0x2e27, },

        DeviceInfo { make: "ROCCAT", model: "Kone Aimo Remastered", usb_vid: 0x1e7d, usb_pid: 0x2e2c, },

        DeviceInfo { make: "ROCCAT", model: "Kone XTD Mouse",       usb_vid: 0x1e7d, usb_pid: 0x2e22, },

        DeviceInfo { make: "ROCCAT", model: "Kone Pure Ultra",      usb_vid: 0x1e7d, usb_pid: 0x2dd2, },

        DeviceInfo { make: "ROCCAT", model: "Burst Pro",            usb_vid: 0x1e7d, usb_pid: 0x2de1, },

        DeviceInfo { make: "ROCCAT", model: "Kova AIMO",            usb_vid: 0x1e7d, usb_pid: 0x2cf1, },
        DeviceInfo { make: "ROCCAT", model: "Kova AIMO",            usb_vid: 0x1e7d, usb_pid: 0x2cf3, },

        DeviceInfo { make: "ROCCAT", model: "Nyth",                 usb_vid: 0x1e7d, usb_pid: 0x2e7c, },
        DeviceInfo { make: "ROCCAT", model: "Nyth",                 usb_vid: 0x1e7d, usb_pid: 0x2e7d, },
    ];
}

pub fn get_device_make(usb_vid: u16, usb_pid: u16) -> Option<&'static str> {
    Some(get_device_info(usb_vid, usb_pid)?.make)
}

pub fn get_device_model(usb_vid: u16, usb_pid: u16) -> Option<&'static str> {
    Some(get_device_info(usb_vid, usb_pid)?.model)
}

pub fn get_device_info(usb_vid: u16, usb_pid: u16) -> Option<&'static DeviceInfo> {
    DEVICE_INFO.iter().find(|e| {
        if e.usb_vid == usb_vid && e.usb_pid == usb_pid {
            true
        } else {
            false
        }
    })
}