using Newtonsoft.Json;

namespace Sirkadirov.Overtest.Daemon.Submodules.ExecutionAgent.Request;

public sealed class ProgramExecutionParameters
{
    [JsonProperty("command_name")]
    public string CommandName { get; set; }
    
    [JsonProperty("command_args")]
    public string[] CommandArgs { get; set; }
    
    [JsonProperty("working_path")]
    public string WorkingPath { get; set; }
    
    [JsonProperty("run_as_uname")]
    public string RunAsUserName { get; set; }
}