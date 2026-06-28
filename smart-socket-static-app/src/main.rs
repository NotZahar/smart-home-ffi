#[repr(C)]
pub struct SmartSocketHandle {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SmartSocketState {
    On = 1,
    Off = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SmartSocketStatus {
    Ok = 0,
    NullPointer = 1,
    InvalidArgument = 2,
}

unsafe extern "C" {
    fn smart_socket_default() -> *mut SmartSocketHandle;
    fn smart_socket_destroy(socket: *mut SmartSocketHandle);
    fn smart_socket_turn_on(socket: *mut SmartSocketHandle) -> SmartSocketStatus;
    fn smart_socket_turn_off(socket: *mut SmartSocketHandle) -> SmartSocketStatus;
    fn smart_socket_get_state(
        socket: *const SmartSocketHandle,
        state_out: *mut SmartSocketState,
    ) -> SmartSocketStatus;
    fn smart_socket_get_power(
        socket: *mut SmartSocketHandle,
        power_out: *mut f32,
    ) -> SmartSocketStatus;
}

fn ensure_ok(status: SmartSocketStatus, operation: &str) {
    assert_eq!(status, SmartSocketStatus::Ok, "{operation} failed");
}

fn read_state(socket: *const SmartSocketHandle) -> SmartSocketState {
    let mut state = SmartSocketState::Off;

    // SAFETY: The socket handle is created by the library and state points to a valid
    // writable stack slot
    let status = unsafe { smart_socket_get_state(socket, &mut state) };
    ensure_ok(status, "get state");

    state
}

fn read_power(socket: *mut SmartSocketHandle) -> f32 {
    let mut power = 0.0;

    // SAFETY: The socket handle is created by the library and power points to a valid
    // writable stack slot
    let status = unsafe { smart_socket_get_power(socket, &mut power) };
    ensure_ok(status, "get power");

    power
}

fn main() {
    // SAFETY: The symbol is linked from the smart-socket-ffi static library and
    // returns either null or a handle owned by that library
    let socket = unsafe { smart_socket_default() };
    assert!(!socket.is_null(), "default socket allocation failed");

    println!("static app: initial state = {:?}", read_state(socket));
    println!("static app: initial power = {:.2} W", read_power(socket));

    // SAFETY: socket was created by the library and is still alive
    let status = unsafe { smart_socket_turn_on(socket) };
    ensure_ok(status, "turn on");

    println!("static app: after turn_on = {:?}", read_state(socket));
    println!("static app: active power = {:.2} W", read_power(socket));

    // SAFETY: socket was created by the library and is still alive
    let status = unsafe { smart_socket_turn_off(socket) };
    ensure_ok(status, "turn off");

    println!("static app: after turn_off = {:?}", read_state(socket));
    println!("static app: inactive power = {:.2} W", read_power(socket));

    // SAFETY: socket was created by the library and is not used after this call
    unsafe {
        smart_socket_destroy(socket);
    }
}
