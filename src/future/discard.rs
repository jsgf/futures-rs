use {Future, Poll, Async};

/// `Discard` waits for an underlying Future to complete, but discards
/// its results or error.
#[derive(Debug)]
pub struct Discard<F>(F);

pub fn new<F>(f: F) -> Discard<F> {
    Discard(f)
}

impl<F> Future for Discard<F> where F: Future {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.0.poll() {
            Err(_) => Err(()),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Ok(Async::Ready(_)) => Ok(Async::Ready(())),
        }
    }
}