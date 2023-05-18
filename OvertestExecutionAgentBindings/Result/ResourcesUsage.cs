using Newtonsoft.Json;

namespace Sirkadirov.Overtest.Daemon.Submodules.ExecutionAgent.Result;

public sealed class ResourcesUsage
{
    [JsonProperty("real_time")]
    public ulong RealTime { get; set; }
    
    [JsonProperty("proc_time")]
    public ulong ProcessorTime { get; set; }
    
    [JsonProperty("proc_wset")]
    public ulong ProcessWorkingSet { get; set; }
}