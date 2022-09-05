Dones:
* status code with Result wrapping
* message builder for help and code boilerplating
* logger with log + env_logger
* example for diamond tasks
* refactoring all the procedure

* builder with macro support, and modify example usage
* make asynchonously with tokio runtime
* optimize tokio runtime with tracing + tracing-subscriber + tracing-chrome

epic
* scheduling: a naive dynamic list scheduling = active_registry (state machine)
  * system on_receive change into a event_loop, copy all senders, one for on_receive, called by parent; others are send to actors for callback-msg-pass
  * actors can re-register into availables, that poll_ready_actor can get first

WIPs:
* scheduling: a naive dynamic list scheduling = active_registry (state machine)
  * make poll_ready_actor async, that dispatch msg can become a easy-life
  * make 100 workloads workable all right
  * <then try to use crt workflow_frontend to call a session embedded actorsystem, with fake executor>
  * session create/own actorsystem
  * run a single instruction with actorsystem + fake executor
  * achieve exec dep/data dep, real data on vm. actor take so called id/symbol/signal of data and only simulate behaviours(future may move data since device handle need ownership; then use arc to manage data in further future)
* abstract out trait as pub interface = impl a simulator for paper (current executor is not integrated with workload) + adapt with chopper
  * define executor trait and (current: executor struct, into trait executor + simulator struct, types use trait bound)
  * make device_context/functor to impl executor trait and provide that fake one
  * bridge the real function to there
* make Workload trait to packaging real load, and make real workflow work

Backlogs:
* perf tuning with RUST PERFORMANCE BOOK IN WEBSITES' BOOKMARK
