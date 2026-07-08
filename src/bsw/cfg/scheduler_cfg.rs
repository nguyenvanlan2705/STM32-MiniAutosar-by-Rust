#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::bsw::services::scheduler_type::{SchedulerRunnableConfig, SchedulerRunnableType, SchedulerTasksTable};
use crate::bsw::services::scheduler::{scheduler_runnable_10ms,
     scheduler_runnable_100ms, scheduler_runnable_1ms, scheduler_runnable_500ms};
use core::sync::atomic::AtomicU32;


pub const SCHEDULER_TASKS_TABLE: SchedulerTasksTable = SchedulerTasksTable {
    tasks: &[
    SchedulerRunnableConfig {
        task_type: SchedulerRunnableType::Periodic,
        period_ms: 1, // 1 ms periodic task
        task_fn: scheduler_runnable_1ms,
    },
    SchedulerRunnableConfig {
        task_type: SchedulerRunnableType::Periodic,
        period_ms: 10, // 10 ms periodic task
        task_fn: scheduler_runnable_10ms,
    },
    SchedulerRunnableConfig {
        task_type: SchedulerRunnableType::Periodic,
        period_ms: 100, // 100 ms periodic task
        task_fn: scheduler_runnable_100ms,
    },
    SchedulerRunnableConfig {
        task_type: SchedulerRunnableType::Periodic,
        period_ms: 500, // 500 ms periodic task
        task_fn: scheduler_runnable_500ms,
    },
    ],
};
const TASK_SIZE: usize = SCHEDULER_TASKS_TABLE.tasks.len();
pub static  TASK_LAST_RUN_TICKS: [AtomicU32; TASK_SIZE] = [const { AtomicU32::new(0) }; TASK_SIZE];