use crate::base::{ResolveWireEvent, ResolveWireGeEvent};
use crate::ext::{EventExtensionData, Extension};
use crate::ffi::*;
use crate::x;

#[cfg(feature = "damage")]
use crate::damage;

#[cfg(feature = "dri2")]
use crate::dri2;

#[cfg(feature = "glx")]
use crate::glx;

#[cfg(feature = "present")]
use crate::present;

#[cfg(feature = "randr")]
use crate::randr;

#[cfg(feature = "screensaver")]
use crate::screensaver;

#[cfg(feature = "shape")]
use crate::shape;

#[cfg(feature = "shm")]
use crate::shm;

#[cfg(feature = "sync")]
use crate::sync;

#[cfg(feature = "xfixes")]
use crate::xfixes;

#[cfg(feature = "xinput")]
use crate::xinput;

#[cfg(feature = "xkb")]
use crate::xkb;

#[cfg(feature = "xprint")]
use crate::xprint;

#[cfg(feature = "xv")]
use crate::xv;

/// Unified Event type from the X server.
#[derive(Debug)]
pub enum Event {
    /// The event is issued from the X core protocol.
    X(x::Event),

    #[cfg(feature = "damage")]
    /// The event is issued from the `DAMAGE` extension.
    Damage(damage::Event),

    #[cfg(feature = "dri2")]
    /// The event is issued from the `DRI2` extension.
    Dri2(dri2::Event),

    #[cfg(feature = "glx")]
    /// The event is issued from the `GLX` extension.
    Glx(glx::Event),

    #[cfg(feature = "present")]
    /// The event is issued from the `Present` extension.
    Present(present::Event),

    #[cfg(feature = "randr")]
    /// The event is issued from the `RANDR` extension.
    RandR(randr::Event),

    #[cfg(feature = "screensaver")]
    /// The event is issued from the `MIT-SCREEN-SAVER` extension.
    ScreenSaver(screensaver::Event),

    #[cfg(feature = "shape")]
    /// The event is issued from the `SHAPE` extension.
    Shape(shape::Event),

    #[cfg(feature = "shm")]
    /// The event is issued from the `MIT-SHM` extension.
    Shm(shm::Event),

    #[cfg(feature = "sync")]
    /// The event is issued from the `SYNC` extension.
    Sync(sync::Event),

    #[cfg(feature = "xfixes")]
    /// The event is issued from the `XFIXES` extension.
    XFixes(xfixes::Event),

    #[cfg(feature = "xinput")]
    /// The event is issued from the `XInputExtension` extension.
    Input(xinput::Event),

    #[cfg(feature = "xkb")]
    /// The event is issued from the `XKEYBOARD` extension.
    Xkb(xkb::Event),

    #[cfg(feature = "xprint")]
    /// The event is issued from the `XpExtension` extension.
    XPrint(xprint::Event),

    #[cfg(feature = "xv")]
    /// The event is issued from the `XVideo` extension.
    Xv(xv::Event),
}

pub(crate) unsafe fn resolve_event(
    event: *mut xcb_generic_event_t,
    extension_data: &[EventExtensionData],
) -> Event {
    let response_type = (*event).response_type & 0x7F;

    if response_type == XCB_GE_GENERIC {
        let event = event as *mut xcb_ge_generic_event_t;
        let extension = (*event).extension;
        for ext in extension_data {
            if ext.major_opcode == extension {
                match ext.ext {
                    #[cfg(feature = "present")]
                    Extension::Present => {
                        return Event::Present(present::Event::resolve_wire_ge_event(event));
                    }
                    #[cfg(feature = "xinput")]
                    Extension::Input => {
                        return Event::Input(xinput::Event::resolve_wire_ge_event(event));
                    }
                    _ => panic!("could not resolve Generic Event extension"),
                }
            }
        }
    }

    for data in extension_data {
        if response_type >= data.first_event {
            match data.ext {
                #[cfg(feature = "damage")]
                Extension::Damage => {
                    return Event::Damage(damage::Event::resolve_wire_event(
                        data.first_event,
                        event,
                    ));
                }

                #[cfg(feature = "dri2")]
                Extension::Dri2 => {
                    return Event::Dri2(dri2::Event::resolve_wire_event(data.first_event, event));
                }

                #[cfg(feature = "glx")]
                Extension::Glx => {
                    return Event::Glx(glx::Event::resolve_wire_event(data.first_event, event));
                }

                #[cfg(feature = "present")]
                Extension::Present => {
                    return Event::Present(present::Event::resolve_wire_event(
                        data.first_event,
                        event,
                    ));
                }

                #[cfg(feature = "randr")]
                Extension::RandR => {
                    return Event::RandR(randr::Event::resolve_wire_event(data.first_event, event));
                }

                #[cfg(feature = "screensaver")]
                Extension::ScreenSaver => {
                    return Event::ScreenSaver(screensaver::Event::resolve_wire_event(
                        data.first_event,
                        event,
                    ));
                }

                #[cfg(feature = "shape")]
                Extension::Shape => {
                    return Event::Shape(shape::Event::resolve_wire_event(data.first_event, event));
                }

                #[cfg(feature = "shm")]
                Extension::Shm => {
                    return Event::Shm(shm::Event::resolve_wire_event(data.first_event, event));
                }

                #[cfg(feature = "sync")]
                Extension::Sync => {
                    return Event::Sync(sync::Event::resolve_wire_event(data.first_event, event));
                }

                #[cfg(feature = "xfixes")]
                Extension::XFixes => {
                    return Event::XFixes(xfixes::Event::resolve_wire_event(
                        data.first_event,
                        event,
                    ));
                }

                #[cfg(feature = "xinput")]
                Extension::Input => {
                    return Event::Input(xinput::Event::resolve_wire_event(
                        data.first_event,
                        event,
                    ));
                }

                #[cfg(feature = "xkb")]
                Extension::Xkb => {
                    return Event::Xkb(xkb::Event::resolve_wire_event(data.first_event, event));
                }

                #[cfg(feature = "xprint")]
                Extension::XPrint => {
                    return Event::XPrint(xprint::Event::resolve_wire_event(
                        data.first_event,
                        event,
                    ));
                }

                #[cfg(feature = "xv")]
                Extension::Xv => {
                    return Event::Xv(xv::Event::resolve_wire_event(data.first_event, event));
                }

                _ => {}
            }
        }
    }

    Event::X(x::Event::resolve_wire_event(0, event))
}