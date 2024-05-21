const RAND_MAX: u64 = 2_147_483_647;
use core::hash::BuildHasher;
use crate::time;
use spinlock::SpinNoIrq;
static PARK_MILLER_LEHMER_SEED: SpinNoIrq<u32> = SpinNoIrq::new(0);
use core::hash::SipHasher13;
use core::hash::Hasher;

pub fn random() -> u128 {
    let mut seed = PARK_MILLER_LEHMER_SEED.lock();
    if *seed == 0 {
        *seed = time::current_ticks() as u32;
    }

    let mut ret: u128 = 0;
    for _ in 0..4 {
        *seed = ((u64::from(*seed) * 48271) % RAND_MAX) as u32;
        ret = (ret << 32) | (*seed as u128);
    }
    ret
}



/// The default [`Hasher`] used by [`RandomState`].
///
/// The internal algorithm is not specified, and so it and its hashes should
/// not be relied upon over releases.
#[allow(deprecated)]
#[derive(Clone, Debug)]
pub struct DefaultHasher(SipHasher13);

impl DefaultHasher {
    /// Creates a new `DefaultHasher`.
    ///
    /// This hasher is not guaranteed to be the same as all other
    /// `DefaultHasher` instances, but is the same as all other `DefaultHasher`
    /// instances created through `new` or `default`.
    #[inline]
    #[allow(deprecated)]
    #[must_use]
    pub const fn new() -> DefaultHasher {
        DefaultHasher(SipHasher13::new_with_keys(0, 0))
    }
}

impl Default for DefaultHasher {
    /// Creates a new `DefaultHasher` using [`new`].
    /// See its documentation for more.
    ///
    /// [`new`]: DefaultHasher::new
    #[inline]
    fn default() -> DefaultHasher {
        DefaultHasher::new()
    }
}

impl Hasher for DefaultHasher {
    // The underlying `SipHasher13` doesn't override the other
    // `write_*` methods, so it's ok not to forward them here.

    #[inline]
    fn write(&mut self, msg: &[u8]) {
        self.0.write(msg)
    }

    #[inline]
    fn write_str(&mut self, s: &str) {
        self.0.write_str(s);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0.finish()
    }
}


impl BuildHasher for RandomState {
    type Hasher = DefaultHasher;
    #[inline]
    #[allow(deprecated)]
    fn build_hasher(&self) -> DefaultHasher {
        DefaultHasher(SipHasher13::new_with_keys(self.k0, self.k1))
    }
}

#[derive(Default)]
pub struct RandomState {
    k0: u64,
    k1: u64,
}


impl RandomState {
    pub fn new() -> RandomState {
        let x1  = random();
        let x2 =  random();
        RandomState{ k0 : x1 as u64 , k1 : x2 as u64 }
    }
}