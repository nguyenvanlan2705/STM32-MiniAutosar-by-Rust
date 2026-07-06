#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]


use crate ::bsw::management::comm::comm_type::{ComM_NetWorkHandleType, ComMMode, ComMReturnType, ComMRequestedMode };
use crate::bsw::cfg::comm_cfg::{COMM_NETWORK_HANDLE_COUNT, comm_get_network_handle_config, ComM_NetWorkHandleConfig, ComMUser};
use core::sync::atomic::{AtomicU8, Ordering};

static COMM_CURRENTCOMMODE: [AtomicU8; COMM_NETWORK_HANDLE_COUNT] = [const { AtomicU8::new(0) }; COMM_NETWORK_HANDLE_COUNT];
static COMM_REQUESTEDCOMMODE: [AtomicU8; COMM_NETWORK_HANDLE_COUNT] = [const { AtomicU8::new(0) }; COMM_NETWORK_HANDLE_COUNT];

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
fn get_requestedmode(network_handle: ComM_NetWorkHandleType) -> Option<ComMRequestedMode> {
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
fn get_cfg_by_user(user: ComMUser) -> Option<&'static ComM_NetWorkHandleConfig> {
    let network_handle_config = comm_get_network_handle_config();
    for config in network_handle_config {
        if config.user == user{
            return Some(config);
        }
    }
    None
}

// Function to convert a requested communication mode to the corresponding current communication mode
fn comm_requested_to_current_mode(requested: ComMRequestedMode) -> ComMMode {
    match requested {
        ComMRequestedMode::NO_COMMUNICATION => ComMMode::NO_COMMUNICATION,
        ComMRequestedMode::FULL_COMMUNICATION => ComMMode::FULL_COMMUNICATION,
    }
}

// Function to request a communication mode for a given user
pub fn comm_requestcommode(user:ComMUser, mode:ComMRequestedMode) -> ComMReturnType {
    let cfg = get_cfg_by_user(user);
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
    }
}
// Main function to be called periodically to update the communication modes
pub fn comm_mainfunction() {
    let network_handle_config = comm_get_network_handle_config();
    for config in network_handle_config {
        let requested_mode = get_requestedmode(config.network_handle);
        if let Some(requested_mode) = requested_mode {
            let new_mode = comm_requested_to_current_mode(requested_mode);
            let current_mode = comm_getcurrentcommode(config.network_handle);
            if let Some(current_mode) = current_mode {
                if new_mode as u8 != current_mode as u8 {
                    let _ = comm_setcurrentcommode(config.network_handle, new_mode);
                }
            }
        }
    }
}