pub trait Duplex {
    type Protocol;
    type Effect<T>;
    type SendErr;

    fn send(&self, msg: Self::Protocol) -> Result<(), Self::SendErr>;

    fn recv(&self) -> Self::Effect<Self::Protocol>;
}
