pub use self::inner::SteadyTime;

mod inner {
    use time::Duration;
    use ops::Sub;

    pub struct SteadyTime {
        t: u64,
    }

    impl SteadyTime {
        pub fn now() -> SteadyTime {
            // STUB:
            SteadyTime {
                t: 0
            }
        }

        pub fn ns(&self) -> u64 {
            // STUB:
            self.t
        }
    }

    impl<'a> Sub for &'a SteadyTime {
        type Output = Duration;

        fn sub(self, other: &SteadyTime) -> Duration {
            // STUB:
            Duration::nanoseconds(self.t as i64 - other.t as i64)
        }
    }
}
