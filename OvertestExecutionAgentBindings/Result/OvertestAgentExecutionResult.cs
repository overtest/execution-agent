using Newtonsoft.Json;

namespace Sirkadirov.Overtest.Daemon.Submodules.ExecutionAgent.Result;

public sealed class OvertestAgentExecutionResult
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
    
    public static OvertestAgentExecutionResult FromJson(string json) =>
        JsonConvert.DeserializeObject<OvertestAgentExecutionResult>(json);
}