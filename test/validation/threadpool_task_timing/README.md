# Task timing without empty-poll instrumentation

`ThreadPool::RunTask` now measures each successfully dequeued task group: worker
notification when appropriate, execution, rescheduling or queue-entry release.
It excludes unsuccessful queue searching. Previously one scope covered an entire
`DoTask` call, including empty polls and potentially several dequeued groups.
This is an explicit profiler-metric correction, rather than a scheduler change.

The existing per-group wait/execution statistics, `ExecuteLoop` timestamps,
worker spin/sleep deadlines, main-thread wait deadlines, queue precedence,
notifications, rescheduling and publication/release operations are unchanged.
Neither `CTimeProfiler` enablement nor the timers used for engine CPU-share
reporting are changed. A disabled `ScopedMtTimer` still reads three clocks under
the current profiler implementation; removing its construction on empty polls
avoids that work without suppressing actual task timing.

Run the standalone check against an unchanged baseline:

```sh
python3 test/validation/threadpool_task_timing/check.py \
  --baseline /path/to/baseline --candidate . --out /tmp/task-timing-check
```

The check compiles each tree's actual `DoTask` body against a deterministic queue
and timer fixture. It covers 32 combinations of four thread IDs, both async
lanes, both synced-context queue-precedence branches, and empty/populated queues.
Populated cases include global and thread-specific queues, background work,
rescheduling, slice entry release, and self-deleting task flags. It requires
identical dequeue/notification/execution/reschedule/release event order, task
statistics and task timestamps. The fixture counts the existing profiler's three
clock reads per scope: all empty candidate polls must construct zero scopes,
while every dequeued group must retain exactly one scope around its execution.

This isolates control-flow and instrumentation behavior; it does not replace the
normal thread-pool tests or native workload comparison. Measure the same ordinary
idle workload after rebuilding, retaining its per-frame behavior and diagnostic
settings, and compare the fresh native CPU profile with baseline.
