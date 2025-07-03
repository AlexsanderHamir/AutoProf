pub const PROMPT: &str = "
INSTRUCTIONS:

You will receive one or more Go pprof profiles in string format.

Your task is to provide an analysis based on all input profiles.

Based on CPU, memory, mutex, etc profiles, what can you tell me about my system ?
Am I spending too much time in benchmark set up ?
Am I forcing the runtime to do too much work ?
Am I doing too many syscalls ?
Am I doing too many context switches when it comes to golang runtime?
Am I being inefficient in my code or system design ?
";
