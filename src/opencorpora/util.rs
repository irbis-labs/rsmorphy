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
    }
}

#[cfg(not(feature = "trace_opencorpora"))]
mod profiler {
    #[allow(missing_copy_implementations)]
    #[derive(Debug, Clone)]
    pub struct SimpleTraceProfiler {}

    impl SimpleTraceProfiler {
        #[inline(always)]
        pub fn start() -> Self {
            SimpleTraceProfiler {}
        }

        #[inline(always)]
        pub fn waypoint(&mut self, _: &str) {}
    }
}
