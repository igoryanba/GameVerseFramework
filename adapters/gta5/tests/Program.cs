using System.Diagnostics;
using GameVerse.AdapterProtocol;
using Newtonsoft.Json.Linq;

if (args.Contains("--self-test"))
{
    var message=Wire.Message("adapter_hello");message["version"]=1;message["backend"]="test";
    var bytes=Wire.Encode(message);
    var fixture=Path.Combine(AppContext.BaseDirectory,"hello-v1.frame");
    if(!File.ReadAllBytes(fixture).SequenceEqual(bytes))throw new Exception("Shared Rust/C# golden frame mismatch");
    using var fragmented=new FragmentedStream(bytes);
    if (!JToken.DeepEquals(await Wire.Read(fragmented,CancellationToken.None),message)) throw new Exception("Fragmented decode mismatch");
    foreach(var bad in new[]{new byte[]{0,1,0,1},new byte[]{0,0,0,0},new byte[]{0,0,0,3,123}})
    {
        bool failed=false;try{await Wire.Read(new MemoryStream(bad),CancellationToken.None);}catch(Exception e) when(e is IOException || e is InvalidDataException){failed=true;}
        if(!failed)throw new Exception("Accepted malformed frame");
    }
    var duplicate=System.Text.Encoding.UTF8.GetBytes("{\"type\":\"reset\",\"type\":\"session_begin\"}");
    var prefix=new byte[]{0,0,0,(byte)duplicate.Length};
    bool rejected=false;try{await Wire.Read(new MemoryStream(prefix.Concat(duplicate).ToArray()),CancellationToken.None);}catch(Newtonsoft.Json.JsonReaderException){rejected=true;}
    if(!rejected)throw new Exception("Accepted duplicate property");
    var state=new PlayerState{timestamp_ms=1,position=new[]{0f,0f,0f},rotation=new[]{0f,0f,0f,1f},velocity=new[]{0f,0f,0f},model_hash=1,health=200,armor=0,movement=1,weapon_hash=0};
    var locomotion=new LocomotionController();
    if(locomotion.Update(state)||locomotion.Current!=LocomotionState.Idle)throw new Exception("Bad idle state");
    state.velocity=new[]{3f,0f,0f};state.movement=3;
    if(locomotion.Update(state)||!locomotion.Update(state)||locomotion.Current!=LocomotionState.Run)throw new Exception("Locomotion hysteresis failed");
    state.movement=9;
    if(!locomotion.Update(state)||locomotion.Current!=LocomotionState.Jump)throw new Exception("Immediate jump failed");
    var config=new SessionConfig{spawn=new[]{1f,2f,3f},heading=90f,model_hash=1,instance_id=0};
    if(!config.IsValid())throw new Exception("Valid session config rejected");
    var presenceFixture=JObject.Parse(File.ReadAllText(Path.Combine(AppContext.BaseDirectory,"presence-v2.json")));
    if((ulong)presenceFixture["server_tick"]!=7 || (string)presenceFixture["deltas"][0]["locomotion"]!="run"
        || (uint)presenceFixture["deltas"][0]["appearance"]["model_hash"]!=0x705e61f2)
        throw new Exception("Presence v2 shared fixture mismatch");
    Console.WriteLine("PASS framing, Presence v2 fixture, session config, locomotion state and hysteresis");return;
}

int seconds=int.Parse(Value("--duration","30"));
string output=Value("--report",null), pipeName=Value("--pipe",Wire.PipeName);
int reconnect=int.Parse(Value("--reconnect-after","0"));
int stallAfter=int.Parse(Value("--stall-after","0"));
bool stalled=false;
var timer=Stopwatch.StartNew();
var remotes=new Dictionary<EntityId,RemoteEntity>();
var sessions=new HashSet<ulong>();
ulong creates=0,updates=0,destroys=0,resets=0;
var logGate=new object();
void Log(string line){lock(logGate)Console.WriteLine(line);}
PipeLink MakeLink(){var l=new PipeLink(Log,Wire.GameBuild,"synthetic-adapter-harness",pipeName);l.Start();return l;}
var link=MakeLink();bool reconnected=false;
RemoteEntity last=null;
try
{
    while(timer.Elapsed.TotalSeconds<seconds)
    {
        float t=(float)timer.Elapsed.TotalSeconds;
        if(stallAfter>0 && !stalled && t>=stallAfter){stalled=true;await Task.Delay(4000);continue;}
        // Deliberately identifies itself as synthetic: this cannot prove GTA loaded.
        var state=new PlayerState{timestamp_ms=(ulong)timer.ElapsedMilliseconds,position=new[]{100f+(float)Math.Sin(t*0.1),100f,30f},rotation=new[]{0f,0f,0f,1f},velocity=new[]{0.1f*(float)Math.Cos(t*0.1),0f,0f},model_hash=0x705e61f2,health=200,armor=0,movement=1,weapon_hash=0xa2719263};
        link.Publish(state,true);
        while(link.Commands.TryTake(out var command))
        {
            switch((string)command["type"])
            {
                case "session_begin": remotes.Clear();sessions.Add((ulong)command["session"]);break;
                case "remote_entity_create":
                    var entity=Wire.Entity(command);remotes[entity.id]=entity;last=entity;creates++;break;
                case "remote_entity_update":
                    entity=Wire.Entity(command);if(!remotes.ContainsKey(entity.id))throw new Exception("Update before create");remotes[entity.id]=entity;last=entity;updates++;break;
                case "remote_entity_destroy":
                    if(remotes.Remove(command["id"].ToObject<EntityId>()))destroys++;break;
                case "reset": remotes.Clear();resets++;break;
                default: throw new Exception("Unexpected command");
            }
        }
        if(reconnect>0 && !reconnected && t>=reconnect){link.Dispose();remotes.Clear();await Task.Delay(1500);link=MakeLink();reconnected=true;}
        await Task.Delay(16);
    }
}
finally{link.Dispose();}
var report=new JObject { ["backend"]="synthetic-adapter-harness",["runtime"]=Environment.Version.ToString(),["gta_loaded"]=false,["elapsed_seconds"]=timer.Elapsed.TotalSeconds,["sessions"]=JArray.FromObject(sessions),["creates"]=creates,["updates"]=updates,["destroys"]=destroys,["resets"]=resets,["last_remote"]=last==null?JValue.CreateNull():JToken.FromObject(last),["clean_shutdown"]=true };
if(output!=null)File.WriteAllText(output,report.ToString());Console.WriteLine(report);
if(creates==0||updates==0||(reconnect>0&&sessions.Count<2))throw new Exception("Presence scenario not exercised");
if(stallAfter>0&&sessions.Count<3)throw new Exception("Stalled game callback did not reset the session");

string Value(string option,string fallback){int index=Array.IndexOf(args,option);return index>=0?args[index+1]:fallback;}
sealed class FragmentedStream:MemoryStream
{
    public FragmentedStream(byte[] bytes):base(bytes){}
    public override Task<int> ReadAsync(byte[] buffer,int offset,int count,CancellationToken cancel)=>base.ReadAsync(buffer,offset,Math.Min(count,1),cancel);
}
