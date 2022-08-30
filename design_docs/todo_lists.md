Dones:
* status code with Result wrapping
* message builder for help and code boilerplating
* logger with log + env_logger
* example for diamond tasks
* builder with macro support, and modify example usage
* make asynchonously with tokio runtime
* refactoring all the procedure

WIPs:
* optimize tokio runtime with tracing + tracing-subscriber + tracing-chrome
* impl tensorview abstraction
* bridge tensorview + op to form calculation.
* a naive dynamic list scheduling = active_registry (state machine)
* abstract out trait as pub interface = impl a simulator for paper (current executor is not integrated with workload) + adapt with chopper

Todos:
* perf tuning with RUST PERFORMANCE BOOK IN WEBSITES' BOOKMARK
