use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll}
};
use futures::lock::Mutex;
use super::core::{Pack, Error};

struct SharedState<'a> {
    pack: &'a mut Pack,
}

pub struct RunFuture<'a> {
    shared_state: Mutex<SharedState<'a>>,
}

impl<'a> Future for RunFuture<'a> {
    type Output = Result<(), Error>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(mut shared_state) = self.shared_state.try_lock() {
            match shared_state.pack.one_step() {
                Ok(true) => {
                    cx.waker().wake_by_ref();
                    Poll::Pending
                },
                Ok(false) => Poll::Ready(Ok(())),
                Err(e) => Poll::Ready(Err(e)),
            }
        }
        else {
            Poll::Pending   
        }
    }
}

impl<'a> RunFuture<'a> {
    pub fn new(pack: &'a mut Pack) -> Self {
        let shared_state = Mutex::new(SharedState {
            pack,
        });
        RunFuture { shared_state }
    }
}
