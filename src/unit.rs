use futures::Future;
use tokio::prelude::Async;
use futures::Stream;

pub struct Unit<F> {
    fut: F
}

pub struct UnitErr<F> {
    fut: F,
    tag: &'static str,
}

impl<F: Future> Future for Unit<F>
{
    type Item = ();
    type Error = F::Error;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        try_ready!(self.fut.poll());
        Ok(Async::Ready(()))
    }
}
impl<F: Future> Future for UnitErr<F>
where F::Error: std::fmt::Debug
{
    type Item = F::Item;
    type Error = ();

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        match self.fut.poll() {
            Ok(Async::Ready(value)) => Ok(Async::Ready(value)),
            Ok(Async::NotReady) =>  Ok(Async::NotReady),
            Err(err) => {
                println!("{}: {:?}", self.tag, err);
                Err(())
            }
        }
    }
}


impl<F: Stream> Stream for UnitErr<F>
    where F::Error: std::fmt::Debug
{
    type Item = F::Item;
    type Error = ();

    fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
        match self.fut.poll() {
            Ok(Async::Ready(Some(value))) => Ok(Async::Ready(Some(value))),
            Ok(Async::Ready(None)) => Ok(Async::Ready(None)),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(err) => {
                println!("{}: {:?}", self.tag, err);
                Err(())
            }
        }
    }
}
impl<S: Stream> Stream for Unit<S>
{
    type Item = ();
    type Error = S::Error;

    fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
        match self.fut.poll() {
            Ok(Async::Ready(Some(_))) => Ok(Async::Ready(Some(()))),
            Ok(Async::Ready(None)) => Ok(Async::Ready(None)),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(err) => Err(err),
        }
    }
}

pub trait UnitFutureExt: Future {
    fn unit(self) -> Unit<Self> where Self: Sized {
        Unit { fut: self }
    }

    fn unit_err(self, tag: &'static str) -> UnitErr<Self> where Self: Sized {
        UnitErr { fut: self, tag: tag }
    }
}

impl <F> UnitFutureExt for F where F: Future { }

pub trait UnitStreamExt: Stream {
    fn unit(self) -> Unit<Self> where Self: Sized {
        Unit { fut: self }
    }

    fn unit_err(self, tag: &'static str) -> UnitErr<Self> where Self: Sized {
        UnitErr { fut: self, tag: tag }
    }
}

impl <F> UnitStreamExt for F where F: Stream { }


pub struct UnitDebug {
    tag: &'static str,
}

impl <A: std::fmt::Debug> FnOnce<(A)> for UnitDebug {
    type Output = ();

    extern "rust-call"
    fn call_once(self, args: (A)) -> Self::Output {
        println!("{}: {:?}", self.tag, args);
        ()
    }
}

pub fn unit_debug(tag: &'static str) -> UnitDebug {
    UnitDebug { tag }
}

pub fn unit<A>(a: A) -> () {}