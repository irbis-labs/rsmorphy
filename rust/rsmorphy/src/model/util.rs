pub use self::profiler::PausedSimpleTraceProfiler;
pub use self::profiler::SimpleTraceProfiler;

#[cfg(feature = "trace_opencorpora")]
mod profiler {
    use std::time::Duration;
    use std::time::Instant;

    #[allow(missing_copy_implementations)]
    #[derive(Debug, Clone)]
    pub struct SimpleTraceProfiler {
        tm: Instant,
        last_elapsed: Duration,
    }

    #[allow(missing_copy_implementations)]
    #[derive(Debug, Clone)]
    pub struct PausedSimpleTraceProfiler {
        last_elapsed: Duration,
    }

    impl SimpleTraceProfiler {
        #[inline(always)]
        pub fn start() -> Self {
            let tm = Instant::now();
            let last_elapsed = Duration::from_secs(0);
            SimpleTraceProfiler { tm, last_elapsed }
        }

        #[inline(always)]
        pub fn waypoint(&mut self, waypoint_name: &str) {
            let elapsed = self.tm.elapsed();
            let waypoint_time = elapsed - self.last_elapsed;
            log::trace!("{waypoint_name} :: {waypoint_time:?} / {elapsed:?}");
            self.last_elapsed = elapsed;
        }

        pub fn pause(self) -> PausedSimpleTraceProfiler {
            PausedSimpleTraceProfiler {
                last_elapsed: self.last_elapsed,
            }
        }
    }

    impl PausedSimpleTraceProfiler {
        pub fn resume(self) -> SimpleTraceProfiler {
            let tm = Instant::now();
            let last_elapsed = self.last_elapsed;
            SimpleTraceProfiler { tm, last_elapsed }
        }
    }
}

#[cfg(not(feature = "trace_opencorpora"))]
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
