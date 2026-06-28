use std::env;
use std::io;
use std::path::PathBuf;

use libloading::{Library, Symbol};

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

type SocketDefault = unsafe extern "C" fn() -> *mut SmartSocketHandle;
type SocketDestroy = unsafe extern "C" fn(*mut SmartSocketHandle);
type SocketTurnOn = unsafe extern "C" fn(*mut SmartSocketHandle) -> SmartSocketStatus;
type SocketTurnOff = unsafe extern "C" fn(*mut SmartSocketHandle) -> SmartSocketStatus;
type SocketGetState =
    unsafe extern "C" fn(*const SmartSocketHandle, *mut SmartSocketState) -> SmartSocketStatus;
type SocketGetPower = unsafe extern "C" fn(*mut SmartSocketHandle, *mut f32) -> SmartSocketStatus;

fn push_candidate(candidates: &mut Vec<PathBuf>, path: PathBuf) {
    if !candidates.iter().any(|candidate| candidate == &path) {
        candidates.push(path);
    }
}

fn find_library() -> io::Result<PathBuf> {
    let mut candidates = Vec::new();
    let library_file = libloading::library_filename("smart_socket_ffi");

    if let Ok(path) = env::var("SMART_SOCKET_FFI_LIB") {
        push_candidate(&mut candidates, PathBuf::from(path));
    }

    if let Ok(executable) = env::current_exe()
        && let Some(executable_dir) = executable.parent()
    {
        push_candidate(&mut candidates, executable_dir.join(&library_file));
        push_candidate(
            &mut candidates,
            executable_dir.join("deps").join(&library_file),
        );
    }

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if let Some(workspace_dir) = manifest_dir.parent() {
        for profile in ["debug", "release"] {
            push_candidate(
                &mut candidates,
                workspace_dir
                    .join("target")
                    .join(profile)
                    .join(&library_file),
            );
            push_candidate(
                &mut candidates,
                workspace_dir
                    .join("target")
                    .join(profile)
                    .join("deps")
                    .join(&library_file),
            );
        }
    }

    for candidate in &candidates {
        if candidate.is_file() {
            return Ok(candidate.clone());
        }
    }

    let searched = candidates
        .iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>()
        .join(", ");

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("smart_socket_ffi dynamic library was not found; searched: {searched}"),
    ))
}

fn ensure_ok(status: SmartSocketStatus, operation: &str) -> io::Result<()> {
    if status == SmartSocketStatus::Ok {
        return Ok(());
    }

    Err(io::Error::other(format!(
        "{operation} failed with status {status:?}"
    )))
}

fn read_state(
    socket: *const SmartSocketHandle,
    get_state: &Symbol<'_, SocketGetState>,
) -> io::Result<SmartSocketState> {
    let mut state = SmartSocketState::Off;

    // SAFETY: The loaded symbol signature matches the library ABI, the handle is live,
    // and state points to a valid writable stack slot
    ensure_ok(unsafe { get_state(socket, &mut state) }, "get state")?;

    Ok(state)
}

fn read_power(
    socket: *mut SmartSocketHandle,
    get_power: &Symbol<'_, SocketGetPower>,
) -> io::Result<f32> {
    let mut power = 0.0;

    // SAFETY: The loaded symbol signature matches the library ABI, the handle is live,
    // and power points to a valid writable stack slot
    ensure_ok(unsafe { get_power(socket, &mut power) }, "get power")?;

    Ok(power)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let library_path = find_library()?;

    // SAFETY: The selected path is expected to be the cdylib built by smart-socket-ffi
    let library = unsafe { Library::new(&library_path)? };

    // SAFETY: Symbol names and signatures are kept in sync with smart-socket-ffi's C ABI
    unsafe {
        let socket_default: Symbol<'_, SocketDefault> = library.get(b"smart_socket_default")?;
        let socket_destroy: Symbol<'_, SocketDestroy> = library.get(b"smart_socket_destroy")?;
        let socket_turn_on: Symbol<'_, SocketTurnOn> = library.get(b"smart_socket_turn_on")?;
        let socket_turn_off: Symbol<'_, SocketTurnOff> = library.get(b"smart_socket_turn_off")?;
        let socket_get_state: Symbol<'_, SocketGetState> =
            library.get(b"smart_socket_get_state")?;
        let socket_get_power: Symbol<'_, SocketGetPower> =
            library.get(b"smart_socket_get_power")?;

        let socket = socket_default();
        if socket.is_null() {
            return Err(io::Error::other("default socket allocation failed").into());
        }

        println!("dynamic app: loaded {}", library_path.display());
        println!(
            "dynamic app: initial state = {:?}",
            read_state(socket, &socket_get_state)?
        );
        println!(
            "dynamic app: initial power = {:.2} W",
            read_power(socket, &socket_get_power)?
        );

        ensure_ok(socket_turn_on(socket), "turn on")?;
        println!(
            "dynamic app: after turn_on = {:?}",
            read_state(socket, &socket_get_state)?
        );
        println!(
            "dynamic app: active power = {:.2} W",
            read_power(socket, &socket_get_power)?
        );

        ensure_ok(socket_turn_off(socket), "turn off")?;
        println!(
            "dynamic app: after turn_off = {:?}",
            read_state(socket, &socket_get_state)?
        );
        println!(
            "dynamic app: inactive power = {:.2} W",
            read_power(socket, &socket_get_power)?
        );

        socket_destroy(socket);
    }

    Ok(())
}
