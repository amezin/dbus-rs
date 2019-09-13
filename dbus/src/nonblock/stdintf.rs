//! This module contains some standard interfaces and an easy way to call them.
//!
//! See the [D-Bus specification](https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces) for more information about these standard interfaces.
//! 
//! The code here was originally created by dbus-codegen.


// cargo run -- -d org.freedesktop.Notifications -p /org/freedesktop/Notifications -m none -c nonblock -g -i "org.freedesktop.DBus"
// ...and slightly hand edited afterwards to unwrap the variants

pub mod org_freedesktop_dbus {

#![allow(missing_docs)]

// This code was autogenerated with dbus-codegen-rust, see https://github.com/diwic/dbus-rs

use crate as dbus;
use crate::arg;
use crate::nonblock;

pub trait Properties {
    fn get<R0: 'static + for<'b> arg::Get<'b>>(&self, interface_name: &str, property_name: &str) -> nonblock::MethodReply<R0>;
    fn get_all(&self, interface_name: &str) -> nonblock::MethodReply<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>>;
    fn set<I2: arg::Arg + arg::Append>(&self, interface_name: &str, property_name: &str, value: I2) -> nonblock::MethodReply<()>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target=T>> Properties for nonblock::Proxy<'a, C> {

    fn get<R0: 'static + for<'b> arg::Get<'b>>(&self, interface_name: &str, property_name: &str) -> nonblock::MethodReply<R0> {
        self.method_call("org.freedesktop.DBus.Properties", "Get", (interface_name, property_name, ))
            .and_then(|r: (arg::Variant<R0>,)| Ok((r.0).0))
    }

    fn get_all(&self, interface_name: &str) -> nonblock::MethodReply<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>> {
        self.method_call("org.freedesktop.DBus.Properties", "GetAll", (interface_name, ))
            .and_then(|r: (::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,)| Ok(r.0))
    }

    fn set<I2: arg::Arg + arg::Append>(&self, interface_name: &str, property_name: &str, value: I2) -> nonblock::MethodReply<()> {
        self.method_call("org.freedesktop.DBus.Properties", "Set", (interface_name, property_name, arg::Variant(value), ))
    }
}

#[derive(Debug)]
pub struct PropertiesPropertiesChanged {
    pub interface_name: String,
    pub changed_properties: ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
    pub invalidated_properties: Vec<String>,
}

impl arg::AppendAll for PropertiesPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface_name, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
}

impl arg::ReadAll for PropertiesPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(PropertiesPropertiesChanged {
            interface_name: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for PropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}

pub trait Introspectable {
    fn introspect(&self) -> nonblock::MethodReply<String>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target=T>> Introspectable for nonblock::Proxy<'a, C> {

    fn introspect(&self) -> nonblock::MethodReply<String> {
        self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .and_then(|r: (String,)| Ok(r.0))
    }
}

pub trait Peer {
    fn ping(&self) -> nonblock::MethodReply<()>;
    fn get_machine_id(&self) -> nonblock::MethodReply<String>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target=T>> Peer for nonblock::Proxy<'a, C> {

    fn ping(&self) -> nonblock::MethodReply<()> {
        self.method_call("org.freedesktop.DBus.Peer", "Ping", ())
    }

    fn get_machine_id(&self) -> nonblock::MethodReply<String> {
        self.method_call("org.freedesktop.DBus.Peer", "GetMachineId", ())
            .and_then(|r: (String,)| Ok(r.0))
    }
}




}

