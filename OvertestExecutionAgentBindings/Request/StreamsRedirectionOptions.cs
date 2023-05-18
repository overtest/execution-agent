using Newtonsoft.Json;

namespace Sirkadirov.Overtest.Daemon.Submodules.ExecutionAgent.Request;

public class StreamsRedirectionOptions
{
    [JsonProperty("input_file_name")]
    public string InputFileName { get; set; }
    
    [JsonProperty("output_file_name")]
    public string OutputFileName { get; set; }
}