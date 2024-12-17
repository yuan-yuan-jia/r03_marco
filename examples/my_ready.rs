
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use futures::task::noop_waker_ref;


#[macro_export]
macro_rules! my_ready {
    ($expr: expr) => {
        match $expr {
            std::task::Poll::Ready(t) => std::task::Poll::Ready(t),
            std::task::Poll::Pending => std::task::Poll::Pending,

        }
    };
}

#[tokio::main]
async fn main() {
    // let mut cx = Context::from_waker(noop_waker_ref());
    // let ret = poll_fut(&mut cx);
    //println!("Final result: {:?}", ret);
    let fut = MyFut::new(42);
    println!("Final result: {:?}", fut.await);
}

fn poll_fut(cx: &mut Context<'_>) -> Poll<usize> {
    let mut fut = MyFut::new(42);
    let fut = Pin::new(&mut fut);
    my_ready!(fut.poll(cx))
}

struct MyFut {
    polled: bool,
    v: usize,
}

impl MyFut {
    fn new(v: usize) -> Self {
        Self {
            polled: false,
            v
        }
    }
}

impl Future for MyFut {
    type Output = usize;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.polled {
            Poll::Ready(self.v)
        } else {
            self.polled = true;
            // wake up the waker
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

