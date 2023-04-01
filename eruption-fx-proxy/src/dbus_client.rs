/*  SPDX-License-Identifier: GPL-3.0-or-later  */

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

#![allow(dead_code)]

use std::time::Duration;

use dbus::blocking::Connection;

use crate::constants;

type Result<T> = std::result::Result<T, eyre::Error>;

#[derive(Debug, thiserror::Error)]
pub enum DbusClientError {
    #[error("Unknown error: {description}")]
    UnknownError { description: String },

    #[error("Authentication failed: {description}")]
    AuthError { description: String },

    #[error("Method call failed: {description}")]
    MethodFailed { description: String },
}

pub fn ping() -> Result<()> {
    use self::config::OrgEruptionConfig;

    let conn = Connection::new_system()?;
    let proxy = conn.with_proxy(
        "org.eruption",
        "/org/eruption/config",
        Duration::from_secs(35),
    );

    if let Err(e) = proxy.ping() {
        log::error!("{}", e);

        Err(DbusClientError::MethodFailed {
            description: format!("{}", e),
        }
        .into())
    } else {
        Ok(())
    }
}

/// Get managed devices USB IDs from the eruption daemon
pub fn get_managed_devices() -> Result<(Vec<(u16, u16)>, Vec<(u16, u16)>, Vec<(u16, u16)>)> {
    use status::OrgEruptionStatus;

    let conn = Connection::new_system()?;
    let status_proxy = conn.with_proxy(
        "org.eruption",
        "/org/eruption/status",
        Duration::from_secs(constants::DBUS_TIMEOUT_MILLIS),
    );

    let result = status_proxy.get_managed_devices()?;

    Ok(result)
}

#[allow(clippy::all)]
mod config {
    // This code was autogenerated with `dbus-codegen-rust -s -d org.eruption -p /org/eruption/config -m None`, see https://github.com/diwic/dbus-rs
    #[allow(unused_imports)]
    use dbus::arg;
    use dbus::blocking;

    pub trait OrgEruptionConfig {
        fn ping(&self) -> Result<bool, dbus::Error>;
        fn ping_privileged(&self) -> Result<bool, dbus::Error>;
        fn write_file(&self, filename: &str, data: &str) -> Result<bool, dbus::Error>;
        fn brightness(&self) -> Result<i64, dbus::Error>;
        fn set_brightness(&self, value: i64) -> Result<(), dbus::Error>;
        fn enable_sfx(&self) -> Result<bool, dbus::Error>;
        fn set_enable_sfx(&self, value: bool) -> Result<(), dbus::Error>;
    }

    #[derive(Debug)]
    pub struct OrgEruptionConfigBrightnessChanged {
        pub brightness: i64,
    }

    impl arg::AppendAll for OrgEruptionConfigBrightnessChanged {
        fn append(&self, i: &mut arg::IterAppend) {
            arg::RefArg::append(&self.brightness, i);
        }
    }

    impl arg::ReadAll for OrgEruptionConfigBrightnessChanged {
        fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
            Ok(OrgEruptionConfigBrightnessChanged {
                brightness: i.read()?,
            })
        }
    }

    impl dbus::message::SignalArgs for OrgEruptionConfigBrightnessChanged {
        const NAME: &'static str = "BrightnessChanged";
        const INTERFACE: &'static str = "org.eruption.Config";
    }

    impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>> OrgEruptionConfig
        for blocking::Proxy<'a, C>
    {
        fn ping(&self) -> Result<bool, dbus::Error> {
            self.method_call("org.eruption.Config", "Ping", ())
                .and_then(|r: (bool,)| Ok(r.0))
        }

        fn ping_privileged(&self) -> Result<bool, dbus::Error> {
            self.method_call("org.eruption.Config", "PingPrivileged", ())
                .and_then(|r: (bool,)| Ok(r.0))
        }

        fn write_file(&self, filename: &str, data: &str) -> Result<bool, dbus::Error> {
            self.method_call("org.eruption.Config", "WriteFile", (filename, data))
                .and_then(|r: (bool,)| Ok(r.0))
        }

        fn brightness(&self) -> Result<i64, dbus::Error> {
            <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
                &self,
                "org.eruption.Config",
                "Brightness",
            )
        }

        fn enable_sfx(&self) -> Result<bool, dbus::Error> {
            <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
                &self,
                "org.eruption.Config",
                "EnableSfx",
            )
        }

        fn set_brightness(&self, value: i64) -> Result<(), dbus::Error> {
            <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
                &self,
                "org.eruption.Config",
                "Brightness",
                value,
            )
        }

        fn set_enable_sfx(&self, value: bool) -> Result<(), dbus::Error> {
            <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(
                &self,
                "org.eruption.Config",
                "EnableSfx",
                value,
            )
        }
    }

    pub trait OrgFreedesktopDBusIntrospectable {
        fn introspect(&self) -> Result<String, dbus::Error>;
    }

    impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>>
        OrgFreedesktopDBusIntrospectable for blocking::Proxy<'a, C>
    {
        fn introspect(&self) -> Result<String, dbus::Error> {
            self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
                .and_then(|r: (String,)| Ok(r.0))
        }
    }

    pub trait OrgFreedesktopDBusProperties {
        fn get(
            &self,
            interface_name: &str,
            property_name: &str,
        ) -> Result<arg::Variant<Box<dyn arg::RefArg + 'static>>, dbus::Error>;
        fn get_all(&self, interface_name: &str) -> Result<arg::PropMap, dbus::Error>;
        fn set(
            &self,
            interface_name: &str,
            property_name: &str,
            value: arg::Variant<Box<dyn arg::RefArg>>,
        ) -> Result<(), dbus::Error>;
    }

    #[derive(Debug)]
    pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
        pub interface_name: String,
        pub changed_properties: arg::PropMap,
        pub invalidated_properties: Vec<String>,
    }

    impl arg::AppendAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
        fn append(&self, i: &mut arg::IterAppend) {
            arg::RefArg::append(&self.interface_name, i);
            arg::RefArg::append(&self.changed_properties, i);
            arg::RefArg::append(&self.invalidated_properties, i);
        }
    }

    impl arg::ReadAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
        fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
            Ok(OrgFreedesktopDBusPropertiesPropertiesChanged {
                interface_name: i.read()?,
                changed_properties: i.read()?,
                invalidated_properties: i.read()?,
            })
        }
    }

    impl dbus::message::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
        const NAME: &'static str = "PropertiesChanged";
        const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
    }

    impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>>
        OrgFreedesktopDBusProperties for blocking::Proxy<'a, C>
    {
        fn get(
            &self,
            interface_name: &str,
            property_name: &str,
        ) -> Result<arg::Variant<Box<dyn arg::RefArg + 'static>>, dbus::Error> {
            self.method_call(
                "org.freedesktop.DBus.Properties",
                "Get",
                (interface_name, property_name),
            )
            .and_then(|r: (arg::Variant<Box<dyn arg::RefArg + 'static>>,)| Ok(r.0))
        }

        fn get_all(&self, interface_name: &str) -> Result<arg::PropMap, dbus::Error> {
            self.method_call(
                "org.freedesktop.DBus.Properties",
                "GetAll",
                (interface_name,),
            )
            .and_then(|r: (arg::PropMap,)| Ok(r.0))
        }

        fn set(
            &self,
            interface_name: &str,
            property_name: &str,
            value: arg::Variant<Box<dyn arg::RefArg>>,
        ) -> Result<(), dbus::Error> {
            self.method_call(
                "org.freedesktop.DBus.Properties",
                "Set",
                (interface_name, property_name, value),
            )
        }
    }
}

#[allow(clippy::all)]
mod status {
    // This code was autogenerated with `dbus-codegen-rust -s -d org.eruption -p /org/eruption/status -m None`, see https://github.com/diwic/dbus-rs
    #[allow(unused_imports)]
    use dbus::arg;
    use dbus::blocking;

    pub trait OrgEruptionStatus {
        fn get_led_colors(&self) -> Result<Vec<(u8, u8, u8, u8)>, dbus::Error>;
        fn get_managed_devices(
            &self,
        ) -> Result<(Vec<(u16, u16)>, Vec<(u16, u16)>, Vec<(u16, u16)>), dbus::Error>;
        fn running(&self) -> Result<bool, dbus::Error>;
    }

    impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>> OrgEruptionStatus
        for blocking::Proxy<'a, C>
    {
        fn get_led_colors(&self) -> Result<Vec<(u8, u8, u8, u8)>, dbus::Error> {
            self.method_call("org.eruption.Status", "GetLedColors", ())
                .and_then(|r: (Vec<(u8, u8, u8, u8)>,)| Ok(r.0))
        }

        fn get_managed_devices(
            &self,
        ) -> Result<(Vec<(u16, u16)>, Vec<(u16, u16)>, Vec<(u16, u16)>), dbus::Error> {
            self.method_call("org.eruption.Status", "GetManagedDevices", ())
                .and_then(|r: ((Vec<(u16, u16)>, Vec<(u16, u16)>, Vec<(u16, u16)>),)| Ok(r.0))
        }

        fn running(&self) -> Result<bool, dbus::Error> {
            <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
                &self,
                "org.eruption.Status",
                "Running",
            )
        }
    }

    pub trait OrgFreedesktopDBusIntrospectable {
        fn introspect(&self) -> Result<String, dbus::Error>;
    }

    impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>>
        OrgFreedesktopDBusIntrospectable for blocking::Proxy<'a, C>
    {
        fn introspect(&self) -> Result<String, dbus::Error> {
            self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
                .and_then(|r: (String,)| Ok(r.0))
        }
    }

    pub trait OrgFreedesktopDBusProperties {
        fn get(
            &self,
            interface_name: &str,
            property_name: &str,
        ) -> Result<arg::Variant<Box<dyn arg::RefArg + 'static>>, dbus::Error>;
        fn get_all(&self, interface_name: &str) -> Result<arg::PropMap, dbus::Error>;
        fn set(
            &self,
            interface_name: &str,
            property_name: &str,
            value: arg::Variant<Box<dyn arg::RefArg>>,
        ) -> Result<(), dbus::Error>;
    }

    #[derive(Debug)]
    pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
        pub interface_name: String,
        pub changed_properties: arg::PropMap,
        pub invalidated_properties: Vec<String>,
    }

    impl arg::AppendAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
        fn append(&self, i: &mut arg::IterAppend) {
            arg::RefArg::append(&self.interface_name, i);
            arg::RefArg::append(&self.changed_properties, i);
            arg::RefArg::append(&self.invalidated_properties, i);
        }
    }

    impl arg::ReadAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
        fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
            Ok(OrgFreedesktopDBusPropertiesPropertiesChanged {
                interface_name: i.read()?,
                changed_properties: i.read()?,
                invalidated_properties: i.read()?,
            })
        }
    }

    impl dbus::message::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
        const NAME: &'static str = "PropertiesChanged";
        const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
    }

    impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>>
        OrgFreedesktopDBusProperties for blocking::Proxy<'a, C>
    {
        fn get(
            &self,
            interface_name: &str,
            property_name: &str,
        ) -> Result<arg::Variant<Box<dyn arg::RefArg + 'static>>, dbus::Error> {
            self.method_call(
                "org.freedesktop.DBus.Properties",
                "Get",
                (interface_name, property_name),
            )
            .and_then(|r: (arg::Variant<Box<dyn arg::RefArg + 'static>>,)| Ok(r.0))
        }

        fn get_all(&self, interface_name: &str) -> Result<arg::PropMap, dbus::Error> {
            self.method_call(
                "org.freedesktop.DBus.Properties",
                "GetAll",
                (interface_name,),
            )
            .and_then(|r: (arg::PropMap,)| Ok(r.0))
        }

        fn set(
            &self,
            interface_name: &str,
            property_name: &str,
            value: arg::Variant<Box<dyn arg::RefArg>>,
        ) -> Result<(), dbus::Error> {
            self.method_call(
                "org.freedesktop.DBus.Properties",
                "Set",
                (interface_name, property_name, value),
            )
        }
    }
}

#[allow(clippy::all)]
mod devices {
    // This code was autogenerated with `dbus-codegen-rust -s -d org.eruption -p /org/eruption/devices -m None`, see https://github.com/diwic/dbus-rs
    #[allow(unused_imports)]
    use dbus::arg;
    use dbus::blocking;

    pub trait OrgEruptionDevice {
        fn get_device_config(&self, device: u64, param: &str) -> Result<String, dbus::Error>;
        fn get_device_status(&self, device: u64) -> Result<String, dbus::Error>;
        fn get_managed_devices(
            &self,
        ) -> Result<(Vec<(u16, u16)>, Vec<(u16, u16)>, Vec<(u16, u16)>), dbus::Error>;
        fn set_device_config(
            &self,
            device: u64,
            param: &str,
            value: &str,
        ) -> Result<bool, dbus::Error>;
        fn device_status(&self) -> Result<String, dbus::Error>;
    }

    #[derive(Debug)]
    pub struct OrgEruptionDeviceDeviceHotplug {
        pub device_info: (u16, u16, bool),
    }

    impl arg::AppendAll for OrgEruptionDeviceDeviceHotplug {
        fn append(&self, i: &mut arg::IterAppend) {
            arg::RefArg::append(&self.device_info, i);
        }
    }

    impl arg::ReadAll for OrgEruptionDeviceDeviceHotplug {
        fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
            Ok(OrgEruptionDeviceDeviceHotplug {
                device_info: i.read()?,
            })
        }
    }

    impl dbus::message::SignalArgs for OrgEruptionDeviceDeviceHotplug {
        const NAME: &'static str = "DeviceHotplug";
        const INTERFACE: &'static str = "org.eruption.Device";
    }

    #[derive(Debug)]
    pub struct OrgEruptionDeviceDeviceStatusChanged {
        pub status: String,
    }

    impl arg::AppendAll for OrgEruptionDeviceDeviceStatusChanged {
        fn append(&self, i: &mut arg::IterAppend) {
            arg::RefArg::append(&self.status, i);
        }
    }

    impl arg::ReadAll for OrgEruptionDeviceDeviceStatusChanged {
        fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
            Ok(OrgEruptionDeviceDeviceStatusChanged { status: i.read()? })
        }
    }

    impl dbus::message::SignalArgs for OrgEruptionDeviceDeviceStatusChanged {
        const NAME: &'static str = "DeviceStatusChanged";
        const INTERFACE: &'static str = "org.eruption.Device";
    }

    impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>> OrgEruptionDevice
        for blocking::Proxy<'a, C>
    {
        fn get_device_config(&self, device: u64, param: &str) -> Result<String, dbus::Error> {
            self.method_call("org.eruption.Device", "GetDeviceConfig", (device, param))
                .and_then(|r: (String,)| Ok(r.0))
        }

        fn get_device_status(&self, device: u64) -> Result<String, dbus::Error> {
            self.method_call("org.eruption.Device", "GetDeviceStatus", (device,))
                .and_then(|r: (String,)| Ok(r.0))
        }

        fn get_managed_devices(
            &self,
        ) -> Result<(Vec<(u16, u16)>, Vec<(u16, u16)>, Vec<(u16, u16)>), dbus::Error> {
            self.method_call("org.eruption.Device", "GetManagedDevices", ())
                .and_then(|r: ((Vec<(u16, u16)>, Vec<(u16, u16)>, Vec<(u16, u16)>),)| Ok(r.0))
        }

        fn set_device_config(
            &self,
            device: u64,
            param: &str,
            value: &str,
        ) -> Result<bool, dbus::Error> {
            self.method_call(
                "org.eruption.Device",
                "SetDeviceConfig",
                (device, param, value),
            )
            .and_then(|r: (bool,)| Ok(r.0))
        }

        fn device_status(&self) -> Result<String, dbus::Error> {
            <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
                &self,
                "org.eruption.Device",
                "DeviceStatus",
            )
        }
    }

    pub trait OrgFreedesktopDBusIntrospectable {
        fn introspect(&self) -> Result<String, dbus::Error>;
    }

    impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>>
        OrgFreedesktopDBusIntrospectable for blocking::Proxy<'a, C>
    {
        fn introspect(&self) -> Result<String, dbus::Error> {
            self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
                .and_then(|r: (String,)| Ok(r.0))
        }
    }

    pub trait OrgFreedesktopDBusProperties {
        fn get(
            &self,
            interface_name: &str,
            property_name: &str,
        ) -> Result<arg::Variant<Box<dyn arg::RefArg + 'static>>, dbus::Error>;
        fn get_all(&self, interface_name: &str) -> Result<arg::PropMap, dbus::Error>;
        fn set(
            &self,
            interface_name: &str,
            property_name: &str,
            value: arg::Variant<Box<dyn arg::RefArg>>,
        ) -> Result<(), dbus::Error>;
    }

    #[derive(Debug)]
    pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
        pub interface_name: String,
        pub changed_properties: arg::PropMap,
        pub invalidated_properties: Vec<String>,
    }

    impl arg::AppendAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
        fn append(&self, i: &mut arg::IterAppend) {
            arg::RefArg::append(&self.interface_name, i);
            arg::RefArg::append(&self.changed_properties, i);
            arg::RefArg::append(&self.invalidated_properties, i);
        }
    }

    impl arg::ReadAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
        fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
            Ok(OrgFreedesktopDBusPropertiesPropertiesChanged {
                interface_name: i.read()?,
                changed_properties: i.read()?,
                invalidated_properties: i.read()?,
            })
        }
    }

    impl dbus::message::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
        const NAME: &'static str = "PropertiesChanged";
        const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
    }

    impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>>
        OrgFreedesktopDBusProperties for blocking::Proxy<'a, C>
    {
        fn get(
            &self,
            interface_name: &str,
            property_name: &str,
        ) -> Result<arg::Variant<Box<dyn arg::RefArg + 'static>>, dbus::Error> {
            self.method_call(
                "org.freedesktop.DBus.Properties",
                "Get",
                (interface_name, property_name),
            )
            .and_then(|r: (arg::Variant<Box<dyn arg::RefArg + 'static>>,)| Ok(r.0))
        }

        fn get_all(&self, interface_name: &str) -> Result<arg::PropMap, dbus::Error> {
            self.method_call(
                "org.freedesktop.DBus.Properties",
                "GetAll",
                (interface_name,),
            )
            .and_then(|r: (arg::PropMap,)| Ok(r.0))
        }

        fn set(
            &self,
            interface_name: &str,
            property_name: &str,
            value: arg::Variant<Box<dyn arg::RefArg>>,
        ) -> Result<(), dbus::Error> {
            self.method_call(
                "org.freedesktop.DBus.Properties",
                "Set",
                (interface_name, property_name, value),
            )
        }
    }
}

#[allow(clippy::all)]
mod process_monitor {
    // This code was autogenerated with `dbus-codegen-rust -d org.eruption.process_monitor -p /org/eruption/process_monitor/rules -m None`, see https://github.com/diwic/dbus-rs
    #[allow(unused_imports)]
    use dbus::arg;
    use dbus::blocking;

    pub trait OrgEruptionProcessMonitorRules {
        fn enum_rules(&self) -> Result<Vec<(String, String, String, String)>, dbus::Error>;
        fn set_rules(&self, rules: Vec<(&str, &str, &str, &str)>) -> Result<(), dbus::Error>;
    }

    impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>>
        OrgEruptionProcessMonitorRules for blocking::Proxy<'a, C>
    {
        fn enum_rules(&self) -> Result<Vec<(String, String, String, String)>, dbus::Error> {
            self.method_call("org.eruption.process_monitor.Rules", "EnumRules", ())
                .and_then(|r: (Vec<(String, String, String, String)>,)| Ok(r.0))
        }

        fn set_rules(&self, rules: Vec<(&str, &str, &str, &str)>) -> Result<(), dbus::Error> {
            self.method_call("org.eruption.process_monitor.Rules", "SetRules", (rules,))
        }
    }

    #[derive(Debug)]
    pub struct OrgEruptionProcessMonitorRulesRulesChanged {
        pub rules: Vec<(String, String, String, String)>,
    }

    impl arg::AppendAll for OrgEruptionProcessMonitorRulesRulesChanged {
        fn append(&self, i: &mut arg::IterAppend) {
            arg::RefArg::append(&self.rules, i);
        }
    }

    impl arg::ReadAll for OrgEruptionProcessMonitorRulesRulesChanged {
        fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
            Ok(OrgEruptionProcessMonitorRulesRulesChanged { rules: i.read()? })
        }
    }

    impl dbus::message::SignalArgs for OrgEruptionProcessMonitorRulesRulesChanged {
        const NAME: &'static str = "RulesChanged";
        const INTERFACE: &'static str = "org.eruption.process_monitor.Rules";
    }

    pub trait OrgFreedesktopDBusIntrospectable {
        fn introspect(&self) -> Result<String, dbus::Error>;
    }

    impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>>
        OrgFreedesktopDBusIntrospectable for blocking::Proxy<'a, C>
    {
        fn introspect(&self) -> Result<String, dbus::Error> {
            self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
                .and_then(|r: (String,)| Ok(r.0))
        }
    }
}