using Newtonsoft.Json;

namespace Sirkadirov.Overtest.Daemon.Submodules.ExecutionAgent.Request;

public sealed class RuntimeLimitsOptions
{
    [JsonProperty("max_exec_time")]
    public ulong MaxExecutionTime { get; set; }
    
    [JsonProperty("max_proc_time")]
    public ulong MaxProcessorTime { get; set; }
    
    [JsonProperty("max_wset_size")]
    public ulong MaxWorkingSetSize { get; set; }
}