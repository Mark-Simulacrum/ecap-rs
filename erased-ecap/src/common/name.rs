use ecap;

use std::borrow::Borrow;

pub trait Name {}

impl<N> Name for N
where
    N: ecap::common::name::NameT,
{
}

impl<N> Borrow<N> for dyn Name
where
    N: ecap::common::name::NameT,
{
    fn borrow(&self) -> &N {
        self
    }
}
