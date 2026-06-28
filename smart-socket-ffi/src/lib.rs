#[macro_export]
macro_rules! make_room {
    ( $($device_name:expr => $device:expr),* $(,)? ) => {
        {
            let mut room = $crate::room::SmartRoom::default();
            $(
                room.add_device($device_name, $device);
            )*
            room
        }
    };
}

mod utils;

pub mod builder;
pub mod error;
pub mod home;
pub mod report;
pub mod reporter;
pub mod room;
pub mod smart_device;

pub use builder::HomeBuilder;
pub use reporter::Reporter;
pub use room::Subscriber;
pub use smart_device::{SmartSocket, SmartThermo};

pub type Home = home::SmartHome<f32>;
pub type Room = room::SmartRoom<f32>;
pub type Device = smart_device::Device<f32>;
pub type Socket = smart_device::PowerSocket<f32>;
pub type Thermo = smart_device::CelsiusThermometer<f32>;

pub mod ffi {
    use core::ffi::c_float;
    use std::ptr;

    use crate::smart_device::Socket as SocketOps;
    use crate::smart_device::{SmartSocket, SocketState};

    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SmartSocketStatus {
        Ok = 0,
        NullPointer = 1,
        InvalidArgument = 2,
    }

    #[repr(C)]
    pub struct SmartSocketHandle {
        socket: SmartSocket,
    }

    impl SmartSocketHandle {
        fn new(socket: SmartSocket) -> Self {
            Self { socket }
        }
    }

    fn is_valid_config(default_active_power: c_float, max_power_offset: c_float) -> bool {
        default_active_power.is_finite()
            && max_power_offset.is_finite()
            && max_power_offset.is_sign_positive()
            && max_power_offset > 0.0
    }

    unsafe fn handle_ref<'a>(
        socket: *const SmartSocketHandle,
    ) -> Result<&'a SmartSocketHandle, SmartSocketStatus> {
        if socket.is_null() {
            return Err(SmartSocketStatus::NullPointer);
        }

        // SAFETY: The caller promises that a non-null pointer was created by this
        // library and stays valid for the duration of this call
        Ok(unsafe { &*socket })
    }

    unsafe fn handle_mut<'a>(
        socket: *mut SmartSocketHandle,
    ) -> Result<&'a mut SmartSocketHandle, SmartSocketStatus> {
        if socket.is_null() {
            return Err(SmartSocketStatus::NullPointer);
        }

        // SAFETY: The caller promises that a non-null pointer was created by this
        // library and is uniquely borrowed for the duration of this call
        Ok(unsafe { &mut *socket })
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn smart_socket_create(
        default_active_power: c_float,
        max_power_offset: c_float,
    ) -> *mut SmartSocketHandle {
        if !is_valid_config(default_active_power, max_power_offset) {
            return ptr::null_mut();
        }

        let socket = SmartSocket::new(default_active_power, max_power_offset);

        Box::into_raw(Box::new(SmartSocketHandle::new(socket)))
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn smart_socket_default() -> *mut SmartSocketHandle {
        Box::into_raw(Box::new(SmartSocketHandle::new(SmartSocket::default())))
    }

    /// Destroys a socket handle created by this library
    ///
    /// # Safety
    ///
    /// `socket` must be either null or a pointer returned by `smart_socket_create`
    /// or `smart_socket_default`. A non-null pointer must be passed to this
    /// function at most once and must not be used after destruction
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn smart_socket_destroy(socket: *mut SmartSocketHandle) {
        if socket.is_null() {
            return;
        }

        // SAFETY: The pointer must come from Box::into_raw in this library and
        // must not be used again after destruction
        drop(unsafe { Box::from_raw(socket) });
    }

    /// Turns the socket on
    ///
    /// # Safety
    ///
    /// `socket` must be a valid non-dangling pointer returned by this library
    /// It must be uniquely borrowed for the duration of the call
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn smart_socket_turn_on(
        socket: *mut SmartSocketHandle,
    ) -> SmartSocketStatus {
        match unsafe { handle_mut(socket) } {
            Ok(handle) => {
                handle.socket.turn_on();
                SmartSocketStatus::Ok
            }
            Err(status) => status,
        }
    }

    /// Turns the socket off
    ///
    /// # Safety
    ///
    /// `socket` must be a valid non-dangling pointer returned by this library
    /// It must be uniquely borrowed for the duration of the call
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn smart_socket_turn_off(
        socket: *mut SmartSocketHandle,
    ) -> SmartSocketStatus {
        match unsafe { handle_mut(socket) } {
            Ok(handle) => {
                handle.socket.turn_off();
                SmartSocketStatus::Ok
            }
            Err(status) => status,
        }
    }

    /// Writes the current socket state to `state_out`
    ///
    /// # Safety
    ///
    /// `socket` must be null or a valid non-dangling pointer returned by this
    /// library. `state_out` must be null or point to writable memory for one
    /// `SocketState`
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn smart_socket_get_state(
        socket: *const SmartSocketHandle,
        state_out: *mut SocketState,
    ) -> SmartSocketStatus {
        if state_out.is_null() {
            return SmartSocketStatus::NullPointer;
        }

        match unsafe { handle_ref(socket) } {
            Ok(handle) => {
                // SAFETY: state_out was checked for null and the caller promises
                // it points to writable memory for one SocketState
                unsafe {
                    *state_out = handle.socket.get_state();
                }
                SmartSocketStatus::Ok
            }
            Err(status) => status,
        }
    }

    /// Writes the current power consumption to `power_out`
    ///
    /// # Safety
    ///
    /// `socket` must be null or a valid non-dangling pointer returned by this
    /// library. A non-null socket must be uniquely borrowed for the duration of
    /// the call. `power_out` must be null or point to writable memory for one
    /// `float`
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn smart_socket_get_power(
        socket: *mut SmartSocketHandle,
        power_out: *mut c_float,
    ) -> SmartSocketStatus {
        if power_out.is_null() {
            return SmartSocketStatus::NullPointer;
        }

        match unsafe { handle_mut(socket) } {
            Ok(handle) => {
                // SAFETY: power_out was checked for null and the caller promises
                // it points to writable memory for one float
                unsafe {
                    *power_out = handle.socket.get_power();
                }
                SmartSocketStatus::Ok
            }
            Err(status) => status,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{
            SmartSocketStatus, smart_socket_default, smart_socket_destroy, smart_socket_get_power,
            smart_socket_get_state, smart_socket_turn_on,
        };
        use crate::SmartSocket;
        use crate::smart_device::{Socket as SocketOps, SocketState};

        #[test]
        fn uses_original_smart_socket_model() {
            let mut socket = SmartSocket::default();

            assert_eq!(socket.get_state(), SocketState::Off);
            assert_eq!(socket.get_power(), 0.0);

            socket.turn_on();
            let power = socket.get_power();

            assert!((100.0..=120.0).contains(&power));
        }

        #[test]
        fn ffi_lifecycle_works() {
            let socket = smart_socket_default();
            assert!(!socket.is_null());

            let mut state = SocketState::Off;
            let mut power = 0.0;

            // SAFETY: socket was created by this library and output pointers are
            // valid for writes during these calls
            unsafe {
                assert_eq!(
                    smart_socket_get_state(socket, &mut state),
                    SmartSocketStatus::Ok
                );
                assert_eq!(state, SocketState::Off);
                assert_eq!(smart_socket_turn_on(socket), SmartSocketStatus::Ok);
                assert_eq!(
                    smart_socket_get_power(socket, &mut power),
                    SmartSocketStatus::Ok
                );
                smart_socket_destroy(socket);
            }

            assert!((100.0..=120.0).contains(&power));
        }
    }
}

pub use ffi::{
    SmartSocketHandle, SmartSocketStatus, smart_socket_create, smart_socket_default,
    smart_socket_destroy, smart_socket_get_power, smart_socket_get_state, smart_socket_turn_off,
    smart_socket_turn_on,
};
