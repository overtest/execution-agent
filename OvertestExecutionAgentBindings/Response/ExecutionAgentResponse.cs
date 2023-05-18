using Newtonsoft.Json;

namespace Sirkadirov.Overtest.Daemon.Submodules.ExecutionAgent.Response;

public sealed class ExecutionAgentResponse
{
    [JsonProperty("exit_code")]
    public int ExitCode { get; set; }
    
    [JsonProperty("exit_sign")]
    public int ExitSignal { get; set; }
    
    [JsonProperty("is_killed")]
    public bool IsKilled { get; set; }
    
    [JsonProperty("kill_reason")]
    public int KillReason { get; set; }
    
    [JsonProperty("res_usage")]
    public ResourcesUsage ResourcesUsage { get; set; }
    
    public static ExecutionAgentResponse FromJson(string json) =>
        JsonConvert.DeserializeObject<ExecutionAgentResponse>(json);
}