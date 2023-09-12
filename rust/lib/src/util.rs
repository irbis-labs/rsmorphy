pub use self::profiler::PausedSimpleTraceProfiler;
pub use self::profiler::SimpleTraceProfiler;

#[cfg(feature = "log_trace_model")]
mod profiler {
    use std::time::Duration;
    use std::time::Instant;

    #[allow(missing_copy_implementations)]
    #[derive(Debug, Clone)]
    pub struct SimpleTraceProfiler {
        tm: Instant,
        prev_elapsed: Duration,
        last_elapsed: Duration,
    }

    #[allow(missing_copy_implementations)]
    #[derive(Debug, Clone)]
    pub struct PausedSimpleTraceProfiler {
        prev_elapsed: Duration,
    }

    impl SimpleTraceProfiler {
        #[inline(always)]
        pub fn start() -> Self {
            SimpleTraceProfiler {
                tm: Instant::now(),
                prev_elapsed: Duration::from_secs(0),
                last_elapsed: Duration::from_secs(0),
            }
        }

        #[inline(always)]
        pub fn waypoint(&mut self, waypoint_name: &str) {
            let elapsed = self.tm.elapsed();
            log::trace!(
                "{} :: {:?}  <--  {:?}",
                waypoint_name,
                elapsed,
                self.last_elapsed
            );
            let waypoint_time = elapsed - self.last_elapsed;
            let total_elapsed = self.prev_elapsed + elapsed;
            log::trace!("{waypoint_name} :: {waypoint_time:?} / {:?}", total_elapsed);
            self.last_elapsed = elapsed;
        }

        pub fn pause(self) -> PausedSimpleTraceProfiler {
            PausedSimpleTraceProfiler {
                prev_elapsed: self.prev_elapsed + self.last_elapsed,
            }
        }
    }

    impl PausedSimpleTraceProfiler {
        pub fn resume(self) -> SimpleTraceProfiler {
            SimpleTraceProfiler {
                tm: Instant::now(),
                prev_elapsed: self.prev_elapsed,
                last_elapsed: Duration::from_secs(0),
            }
        }
    }
}

#[cfg(not(feature = "log_trace_model"))]
mod profiler {
    #[allow(missing_copy_implementations)]
    #[derive(Debug, Clone)]
    pub struct SimpleTraceProfiler {}

    #[allow(missing_copy_implementations)]
    #[derive(Debug, Clone)]
    pub struct PausedSimpleTraceProfiler {}

    impl SimpleTraceProfiler {
        #[inline(always)]
        pub fn start() -> Self {
            SimpleTraceProfiler {}
        }

        #[inline(always)]
        pub fn waypoint(&mut self, _: &str) {}

        #[inline(always)]
        pub fn pause(self) -> PausedSimpleTraceProfiler {
            PausedSimpleTraceProfiler {}
        }
    }

    impl PausedSimpleTraceProfiler {
        #[inline(always)]
        pub fn resume(self) -> SimpleTraceProfiler {
            SimpleTraceProfiler {}
        }
    }
}
