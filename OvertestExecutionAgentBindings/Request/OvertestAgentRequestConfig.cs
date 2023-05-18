using Newtonsoft.Json;

namespace Sirkadirov.Overtest.Daemon.Submodules.ExecutionAgent.Request;

public sealed class OvertestAgentRequestConfig
{
    [JsonProperty("prog_exec_params")]
    public ProgramExecutionParameters ExecutionParameters { get; set; }
    
    [JsonProperty("streams_redirect")]
    public StreamsRedirectionOptions StreamsRedirection { get; set; }
    
    [JsonProperty("runtime_limits")]
    public RuntimeLimitsOptions RuntimeLimits { get; set; }

    public override string ToString() => JsonConvert.SerializeObject(this, Formatting.None);
}