use sys::mutex::Mutex;
use time::Duration;

pub struct Condvar;

pub const CONDVAR_INIT: Condvar = Condvar;

impl Condvar {
    #[inline]
    pub fn new() -> Condvar {
        Condvar
    }

    #[inline]
    pub fn notify_one(&self) {
        // STUB:
    }

    #[inline]
    pub fn notify_all(&self) {
        // STUB:
    }

    #[inline]
    pub fn wait(&self, mutex: &Mutex) {
        // STUB:
        let _ = mutex;
    }

    pub fn wait_timeout(&self, mutex: &Mutex, dur: Duration) -> bool {
        // STUB:
        let _ = mutex;
        let _ = dur;
        return true
    }

    #[inline]
    pub fn destroy(&self) {
        // STUB:
    }
}
