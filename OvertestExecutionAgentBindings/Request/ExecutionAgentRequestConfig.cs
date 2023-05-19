using Newtonsoft.Json;

namespace Sirkadirov.Overtest.Daemon.Submodules.ExecutionAgent.Request;

public sealed class ExecutionAgentRequestConfig
{
    [JsonProperty("prog_exec_params")]
    public ProgramExecutionParameters ExecutionParameters { get; set; }
    
    [JsonProperty("streams_redirect")]
    public StreamsRedirectionOptions StreamsRedirection { get; set; }
    
    [JsonProperty("proc_limits_info")]
    public RuntimeLimitsOptions RuntimeLimits { get; set; }

    public override string ToString() => JsonConvert.SerializeObject(this, Formatting.None);
}