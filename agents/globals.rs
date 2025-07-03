pub const PROMPT: &str = r#"
INSTRUCTIONS:
You will receive one or more Go pprof profiling outputs in raw string format, including CPU, memory allocation, mutex contention, blocking profiles, and goroutine traces.
Your task is to analyze these profiles collectively and provide a detailed performance report covering:
1. CPU usage hotspots — which functions or goroutines consume the most CPU time?
2. Memory allocation patterns — are there excessive allocations or leaks?
3. Mutex contention and blocking — is there significant contention or blocking causing slowdowns?
4. Runtime inefficiencies — excessive syscalls, context switches, or GC pauses?
5. Benchmark overhead — are benchmarks spending unnecessary time in setup or teardown?
6. Potential code or design inefficiencies causing performance bottlenecks.
7. Suggestions for improvements in code, runtime usage, or system configuration.
Please provide your analysis as if you are a senior Go performance engineer reviewing production profiling data. Be concise but thorough, and reference specific profiling terms and metrics where appropriate.
End your response with a summary of the top 3 action items to improve performance.
"#;
