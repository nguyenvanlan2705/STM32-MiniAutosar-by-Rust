#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]


use crate ::bsw::management::comm::comm_type::{ComM_NetWorkHandleType, ComMMode, ComMReturnType, ComMRequestedMode, ComM_StateType};
use crate::bsw::cfg::comm_cfg::{COMM_NETWORK_HANDLE_COUNT, comm_get_network_handle_config, ComM_NetWorkHandleConfig, ComMUser};
use core::sync::atomic::{AtomicU8, Ordering};

static COMM_CURRENTCOMMODE: [AtomicU8; COMM_NETWORK_HANDLE_COUNT] = [const { AtomicU8::new(0) }; COMM_NETWORK_HANDLE_COUNT];
static COMM_REQUESTEDCOMMODE: [AtomicU8; COMM_NETWORK_HANDLE_COUNT] = [const { AtomicU8::new(0) }; COMM_NETWORK_HANDLE_COUNT];
static COMM_INTERNALSTATE: [AtomicU8; COMM_NETWORK_HANDLE_COUNT] = [const { AtomicU8::new(0) }; COMM_NETWORK_HANDLE_COUNT];

// Function to get the current communication mode for a given network handle
pub fn comm_getcurrentcommode(network_handle: ComM_NetWorkHandleType) -> Option<ComMMode> {
    let index = network_handle as usize;
    if index >= COMM_NETWORK_HANDLE_COUNT {
        return None;
    }
    match COMM_CURRENTCOMMODE[index].load(Ordering::Relaxed) {
        0 => Some(ComMMode::NO_COMMUNICATION),
        1 => Some(ComMMode::SILENT_COMMUNICATION),
        2 => Some(ComMMode::FULL_COMMUNICATION),
        _ => Some(ComMMode::NO_COMMUNICATION),
    }
}
fn comm_get_requestedmode(network_handle: ComM_NetWorkHandleType) -> Option<ComMRequestedMode> {
    let index = network_handle as usize;
    if index >= COMM_NETWORK_HANDLE_COUNT {
        return None;
    }
    match COMM_REQUESTEDCOMMODE[index].load(Ordering::Relaxed) {
        0 => Some(ComMRequestedMode::NO_COMMUNICATION),
        1 => Some(ComMRequestedMode::FULL_COMMUNICATION),
        _ => Some(ComMRequestedMode::NO_COMMUNICATION),
    }
}
// Function to get the requested communication mode for a given network handle
fn comm_setcurrentcommode(network_handle: ComM_NetWorkHandleType, mode: ComMMode) -> ComMReturnType {
    let index = network_handle as usize;
    if index >= COMM_NETWORK_HANDLE_COUNT {
        return ComMReturnType::COMM_E_NOT_OK;
    }
    COMM_CURRENTCOMMODE[index].store(mode as u8, Ordering::Relaxed);
    ComMReturnType::COMM_E_OK
}
// Function to set the requested communication mode for a given network handle
fn comm_set_requestedcommode(network_handle: ComM_NetWorkHandleType, mode: ComMRequestedMode) -> ComMReturnType {
    let index = network_handle as usize;
    if index >= COMM_NETWORK_HANDLE_COUNT {
        return ComMReturnType::COMM_E_NOT_OK;
    }
    COMM_REQUESTEDCOMMODE[index].store(mode as u8, Ordering::Relaxed);
    ComMReturnType::COMM_E_OK
}
// Function to get the configuration for a given user
fn comm_get_cfg_by_user(user: ComMUser) -> Option<&'static ComM_NetWorkHandleConfig> {
    let network_handle_config = comm_get_network_handle_config();
    for config in network_handle_config {
        if config.user == user{
            return Some(config);
        }
    }
    None
}

// Function to get internal state
fn comm_get_internal_state(network_handle: ComM_NetWorkHandleType) -> Option<ComM_StateType> {
    let index = network_handle as usize;
    if index >= COMM_NETWORK_HANDLE_COUNT {
        return None;
    }
    match COMM_INTERNALSTATE[index].load(Ordering::Relaxed) {
        0 => Some(ComM_StateType::COMM_NO_COM_NO_PENDING_REQUEST),
        1 => Some(ComM_StateType::COMM_NO_COM_REQUEST_PENDING),
        2 => Some(ComM_StateType::COMM_FULL_COM_NETWORK_REQUESTED),
        3 => Some(ComM_StateType::COMM_FULL_COM_READY_SLEEP),
        4 => Some(ComM_StateType::COMM_SILENT_COM),
        _ => None,
    }
}
fn comm_set_internal_state(network_handle: ComM_NetWorkHandleType, state: ComM_StateType) -> ComMReturnType {
    let index = network_handle as usize;
    if index >= COMM_NETWORK_HANDLE_COUNT {
        return ComMReturnType::COMM_E_NOT_OK;
    }
    COMM_INTERNALSTATE[index].store(state as u8, Ordering::Relaxed);
    ComMReturnType::COMM_E_OK
}
// Function to transition the internal state based on the current and requested communication modes
fn comm_transition_state( state: ComM_StateType,requested: ComMRequestedMode,) -> ComM_StateType {
    match (state, requested) {
        (
            ComM_StateType::COMM_NO_COM_NO_PENDING_REQUEST,
            ComMRequestedMode::FULL_COMMUNICATION,
        ) => ComM_StateType::COMM_NO_COM_REQUEST_PENDING,

        (
            ComM_StateType::COMM_NO_COM_REQUEST_PENDING,
            ComMRequestedMode::FULL_COMMUNICATION,
        ) => ComM_StateType::COMM_FULL_COM_NETWORK_REQUESTED,

        (
            ComM_StateType::COMM_FULL_COM_NETWORK_REQUESTED,
            ComMRequestedMode::NO_COMMUNICATION,
        ) => ComM_StateType::COMM_FULL_COM_READY_SLEEP,

        (
            ComM_StateType::COMM_FULL_COM_READY_SLEEP,
            ComMRequestedMode::NO_COMMUNICATION,
        ) => ComM_StateType::COMM_NO_COM_NO_PENDING_REQUEST,

        (_, ComMRequestedMode::NO_COMMUNICATION) => {
            ComM_StateType::COMM_NO_COM_NO_PENDING_REQUEST
        }

        (state, _) => state,
    }
}

// Function to convert internal state to current communication mode
fn comm_internal_state_to_current_mode(state: ComM_StateType) -> ComMMode {
    match state {
        ComM_StateType::COMM_NO_COM_NO_PENDING_REQUEST => ComMMode::NO_COMMUNICATION,
        ComM_StateType::COMM_NO_COM_REQUEST_PENDING => ComMMode::NO_COMMUNICATION,
        ComM_StateType::COMM_FULL_COM_NETWORK_REQUESTED => ComMMode::FULL_COMMUNICATION,
        ComM_StateType::COMM_FULL_COM_READY_SLEEP => ComMMode::FULL_COMMUNICATION,
        ComM_StateType::COMM_SILENT_COM => ComMMode::SILENT_COMMUNICATION,
    }
}
// Function to request a communication mode for a given user
pub fn comm_requestcommode(user:ComMUser, mode:ComMRequestedMode) -> ComMReturnType {
    let cfg = comm_get_cfg_by_user(user);
    if let Some(cfg) = cfg {
        return comm_set_requestedcommode(cfg.network_handle, mode)
    }
    ComMReturnType::COMM_E_NOT_OK
}
// Init function to initialize the communication module
pub fn comm_init() {
    let network_handle_config = comm_get_network_handle_config();
    for config in network_handle_config {
        COMM_CURRENTCOMMODE[config.network_handle as usize].store(ComMMode::NO_COMMUNICATION as u8, Ordering::Relaxed);
        COMM_REQUESTEDCOMMODE[config.network_handle as usize].store(ComMRequestedMode::NO_COMMUNICATION as u8, Ordering::Relaxed);
        COMM_INTERNALSTATE[config.network_handle as usize].store(ComM_StateType::COMM_NO_COM_NO_PENDING_REQUEST as u8, Ordering::Relaxed);
    }
}
// Main function to be called periodically to update the communication modes
pub fn comm_mainfunction() {
    // Iterate through all network handles and update their states based on the requested modes
    let network_handle_config = comm_get_network_handle_config();
    for config in network_handle_config {
        // Get the requested mode for the current network handle
        let requested_mode = comm_get_requestedmode(config.network_handle);
        // If a requested mode exists, transition the internal state and update the current communication mode accordingly
        if let Some(requested_mode) = requested_mode {
            // Get the current internal state for the network handle
            let internal_state = comm_get_internal_state(config.network_handle);
            // If an internal state exists, transition it based on the requested mode and update the current communication mode
            if let Some(internal_state) = internal_state {
                // Transition the internal state based on the requested mode
                let new_state = comm_transition_state(internal_state, requested_mode);
                // Update the internal state and current communication mode for the network handle
                let _ = comm_set_internal_state(config.network_handle, new_state);
                // Convert the new internal state to the corresponding current communication mode
                let new_current_mode = comm_internal_state_to_current_mode(new_state);
                // Update the current communication mode for the network handle
                let _ = comm_setcurrentcommode(config.network_handle, new_current_mode);
            }
        }
    }
}
