#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]


use crate ::bsw::management::comm::comm_type::{ComM_NetWorkHandleType, ComMMode, ComMReturnType, ComMRequestedMode };
use crate::bsw::cfg::comm_cfg::{COMM_NETWORK_HANDLE_COUNT, comm_get_network_handle_config, ComM_NetWorkHandleConfig, ComMUser};
use core::sync::atomic::{AtomicU8, Ordering};

static COMM_CURRENTCOMMODE: [AtomicU8; COMM_NETWORK_HANDLE_COUNT] = [const { AtomicU8::new(0) }; COMM_NETWORK_HANDLE_COUNT];
static COMM_REQUESTEDCOMMODE: [AtomicU8; COMM_NETWORK_HANDLE_COUNT] = [const { AtomicU8::new(0) }; COMM_NETWORK_HANDLE_COUNT];

// Function to get the current communication mode for a given network handle
pub fn comm_getcurrentcommode(network_handle: ComM_NetWorkHandleType) -> ComMMode {
    let index = network_handle as usize;
    if index >= COMM_NETWORK_HANDLE_COUNT {
        return ComMMode::NO_COMMUNICATION;
    }
    match COMM_CURRENTCOMMODE[index].load(Ordering::Relaxed) {
        0 => ComMMode::NO_COMMUNICATION,
        1 => ComMMode::SILENT_COMMUNICATION,
        2 => ComMMode::FULL_COMMUNICATION,
        _ => ComMMode::NO_COMMUNICATION,
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
        let requested_mode = COMM_REQUESTEDCOMMODE[config.network_handle as usize].load(Ordering::Relaxed);
        let current_mode = COMM_CURRENTCOMMODE[config.network_handle as usize].load(Ordering::Relaxed);
        if requested_mode != current_mode {
            COMM_CURRENTCOMMODE[config.network_handle as usize].store(requested_mode, Ordering::Relaxed);
        }
    }
}