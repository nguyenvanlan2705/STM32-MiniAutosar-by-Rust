#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SchedulerRunnableType {
    OneShot,
    Periodic,
}

pub type SchedulerRunnableFn = fn();

pub struct SchedulerRunnableConfig {
    pub task_type: SchedulerRunnableType,
    pub period_ms: u32,
    pub task_fn: SchedulerRunnableFn,
}

pub struct SchedulerTasksTable {
    pub tasks: &'static [SchedulerRunnableConfig],
}