use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ExecutionResult {
    pub exit_code: i32,
    pub exit_sign: i32,
    pub is_killed: bool,
    pub kill_reason : i32,
    pub res_usage: ResourcesUsage
}

impl ExecutionResult {
    pub fn from_limtrac(proc_exec_result: &limtrac::ProcExecResult) -> ExecutionResult
    {
        ExecutionResult {
            exit_code: 0,
            exit_sign: 0,
            is_killed: false,
            kill_reason: 0,
            res_usage: ResourcesUsage {
                real_time: proc_exec_result.res_usage.real_time,
                proc_time: proc_exec_result.res_usage.proc_time,
                proc_wset: proc_exec_result.res_usage.proc_wset,
            },
        }
    }

    pub fn to_json_string(self) -> Result<String, ()>
    {
        return match serde_json::to_string(&self)
        { Ok(res) => Ok(res), Err(_) => Err(()) };
    }
}

#[derive(Serialize, Deserialize)]
pub struct ResourcesUsage {
    pub real_time : u64,
    pub proc_time : u64,
    pub proc_wset : u64
}