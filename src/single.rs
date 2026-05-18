pub trait Receiver<T> {
    type F<A>;
    type Err;
    
    fn recv(&mut self) -> Self::F<Result<T, Self::Err>>;
}

pub trait Sender<T> {
    type F<A>;
    type Err;
    
    fn send(&mut self, value: T) -> Self::F<Result<(), Self::Err>>;
}
