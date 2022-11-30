use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker}
};
use runpack::{self, Pack};

pub struct RunFuture<'a> {
    shared_state: Arc<Mutex<SharedState<'a>>>,
}

struct SharedState<'a> {
    pack: &'a mut Pack,
    waker: Option<Waker>,
}

impl<'a> Future for RunFuture<'a> {
    type Output = Result<(), runpack::Error>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        match shared_state.pack.one_step() {
            Ok(true) => {
                shared_state.waker = Some(cx.waker().clone());
                if let Some(waker) = shared_state.waker.take() {
                    waker.wake()
                }
                Poll::Pending
            },
            Ok(false) => Poll::Ready(Ok(())),
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

//TODO: run a word, an async version of "exec"

impl<'a> RunFuture<'a> {
    pub fn new(pack: &'a mut Pack) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            pack,
            waker: None,
        }));
        RunFuture { shared_state }
    }
}