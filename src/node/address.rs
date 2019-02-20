use std::fmt;

use multiaddr::{self, Multiaddr, ToMultiaddr};

/// the address of any given nodes
///
/// The underlying object is, for now, a [`Multiaddr`] to allow
/// compatibility with the different type of network and identification.
///
/// [`Multiaddr`]: https://docs.rs/multiaddr/latest/multiaddr/struct.Multiaddr.html
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Address(Multiaddr);

impl Address {
    /// create a new address from any given [`ToMultiaddr`] implementors
    ///
    /// # Examples
    ///
    /// TODO: simple example with SockAddr from standard lib
    /// TODO: simple example with a multiaddr object and a string
    ///
    /// TODO: [`ToMultiaddr`]
    pub fn new<Addr>(addr: Addr) -> Result<Self, multiaddr::Error>
    where
        Addr: ToMultiaddr,
    {
        addr.to_multiaddr().map(Address)
    }
}

impl std::str::FromStr for Address {
    type Err = <Multiaddr as std::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Address)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::{Arbitrary, Gen};

    impl Arbitrary for Address {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let ip: (u8, u8, u8, u8) = Arbitrary::arbitrary(g);
            let port: u16 = Arbitrary::arbitrary(g);
            let address = format!("/ip4/{}.{}.{}.{}/tcp/{}", ip.0, ip.1, ip.2, ip.3, port);
            address.parse().unwrap()
        }
    }
}
