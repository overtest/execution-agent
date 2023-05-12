use std::ffi::{c_ulonglong, CString};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestConfig
{
    pub prog_exec_params: ProgramExecutionParameters,
    pub streams_redirect: StreamsRedirectionOptions,
    pub proc_limits_info: RuntimeLimitsOptions
}

impl RequestConfig {
    pub fn from_json_string(str: String) -> Result<RequestConfig, ()>
    {
        return match serde_json::from_str(str.as_str())
        { Ok(obj) => Ok(obj), Err(_) => Err(()) };
    }

    pub fn get_exec_prog_guard(self) -> limtrac::ExecProgGuard
    {
        limtrac::ExecProgGuard {
            scmp_enabled: true,
            scmp_deny_common: true,
            unshare_common: true,
            unshare_network: true
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProgramExecutionParameters
{
    pub command_name: String,
    pub command_args: Vec<String>,
    pub working_path: String,
    pub run_as_uname: String
}

impl ProgramExecutionParameters
{
    pub fn to_limtrac(&self) -> limtrac::ExecProgInfo
    {
        limtrac::ExecProgInfo
        {
            program_path: CString::new(self.command_name.clone()).unwrap().into_raw(),
            program_args: CString::new(self.command_args.clone().join(" ")).unwrap().into_raw(),
            working_path: CString::new(self.working_path.clone()).unwrap().into_raw(),
            exec_as_user: CString::new(self.run_as_uname.clone()).unwrap().into_raw(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct StreamsRedirectionOptions
{
    pub input_file_name: String,
    pub output_file_name: String
}

impl StreamsRedirectionOptions
{
    pub fn to_limtrac(&self) -> limtrac::ExecProgIO
    {
        let io_redirected: bool = self.input_file_name.eq("")
            && self.output_file_name.eq("");

        limtrac::ExecProgIO
        {
            io_redirected,
            io_path_stdin:  CString::new(self.input_file_name.clone()).unwrap().into_raw(),
            io_path_stdout: CString::new(self.output_file_name.clone()).unwrap().into_raw(),
            io_path_stderr: CString::new("".to_string()).unwrap().into_raw(),
            io_dup_err_out: true,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RuntimeLimitsOptions
{
    pub max_exec_time: u64,
    pub max_proc_time: u64,
    pub max_wset_size: u64
}

impl RuntimeLimitsOptions {
    pub fn to_limtrac(&self) -> limtrac::ExecProgLimits
    {
        limtrac::ExecProgLimits
        {
            limit_real_time: self.max_exec_time as c_ulonglong,
            limit_proc_time: self.max_proc_time as c_ulonglong,
            limit_proc_wset: self.max_wset_size as c_ulonglong,
            rlimit_enabled: false,
            rlimit_core: 0, rlimit_npoc: 0, rlimit_nofile: 0
        }
    }
}