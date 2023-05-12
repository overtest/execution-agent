mod request_config;
mod execution_result;

use std::io;
use crate::execution_result::ExecutionResult;
use crate::request_config::RequestConfig;

fn main() {
    // Get a new STDIN handle to read from it
    let stdin: io::Stdin = io::stdin();

    // Read line containing JSON string from STDIN
    let mut input_data: String = String::new();
    stdin.read_line(&mut input_data).unwrap();

    // Deserialize JSON string into request config object
    let request_config: RequestConfig = RequestConfig::from_json_string(input_data).unwrap();

    // Create and fill LIMTRAC request structs
    let exec_prog_info  : limtrac::ExecProgInfo   = request_config.prog_exec_params.to_limtrac();
    let exec_prog_io    : limtrac::ExecProgIO     = request_config.streams_redirect.to_limtrac();
    let exec_prog_limits: limtrac::ExecProgLimits = request_config.proc_limits_info.to_limtrac();
    let exec_prog_guard : limtrac::ExecProgGuard  = request_config.get_exec_prog_guard();

    // Invoke LIMTRAC and wait till it returns the execution resut details
    let limtrac_result: limtrac::ProcExecResult = limtrac::limtrac_execute(
        exec_prog_info, exec_prog_io, exec_prog_limits, exec_prog_guard);

    // Change execution results format and serialize a newly-created object into JSON string
    let execution_result = ExecutionResult::from_limtrac(&limtrac_result);
    let execution_result_str = execution_result.to_json_string().unwrap();

    // Print JSON string generated during previous step to STDOUT
    println!("{}", execution_result_str.as_str());
}