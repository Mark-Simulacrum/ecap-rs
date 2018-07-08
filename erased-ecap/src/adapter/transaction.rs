use ecap;
use ecap::common::{Area, Name, NamedValueVisitor};

use common;
use host::Host as ErasedHost;
use host::Transaction as ErasedTransaction;

pub trait Transaction: common::Options {
    fn start<'a>(&mut self, host: &'a mut (dyn ErasedTransaction<dyn ErasedHost> + 'static));
    fn stop<'a>(&mut self, host: &'a mut (dyn ErasedTransaction<dyn ErasedHost> + 'static));
    fn resume<'a>(&mut self, host: &'a mut (dyn ErasedTransaction<dyn ErasedHost> + 'static));
    fn adapted_body_discard<'a>(
        &mut self,
        host: &'a mut (dyn ErasedTransaction<dyn ErasedHost> + 'static),
    );
    fn adapted_body_make<'a>(
        &mut self,
        host: &'a mut (dyn ErasedTransaction<dyn ErasedHost> + 'static),
    );
    fn adapted_body_make_more<'a>(
        &mut self,
        host: &'a mut (dyn ErasedTransaction<dyn ErasedHost> + 'static),
    );
    fn adapted_body_stop_making<'a>(
        &mut self,
        host: &'a mut (dyn ErasedTransaction<dyn ErasedHost> + 'static),
    );
    fn adapted_body_pause<'a>(
        &mut self,
        host: &'a mut (dyn ErasedTransaction<dyn ErasedHost> + 'static),
    );
    fn adapted_body_resume<'a>(
        &mut self,
        host: &'a mut (dyn ErasedTransaction<dyn ErasedHost> + 'static),
    );
    fn adapted_body_content<'a>(
        &mut self,
        host: &'a mut (dyn ErasedTransaction<dyn ErasedHost> + 'static),
        offset: usize,
        size: usize,
    ) -> Area;
    fn adapted_body_content_shift<'a>(
        &mut self,
        host: &'a mut (dyn ErasedTransaction<dyn ErasedHost> + 'static),
        size: usize,
    );
    fn virgin_body_content_done<'a>(
        &mut self,
        host: &'a mut (dyn ErasedTransaction<dyn ErasedHost> + 'static),
        at_end: bool,
    );
    fn virgin_body_content_available<'a>(
        &mut self,
        host: &'a mut (dyn ErasedTransaction<dyn ErasedHost> + 'static),
    );
}

macro_rules! generate_method_transaction {
    ($name:ident) => {
        fn $name<'a>(&mut self, h: &'a mut (dyn ErasedTransaction<dyn ErasedHost> + 'static)) {
            U::$name(self, h)
        }
    }
}

impl<U> Transaction for U
where
    U: ecap::adapter::Transaction<dyn ErasedHost> + ?Sized,
{
    generate_method_transaction!(start);
    generate_method_transaction!(stop);
    generate_method_transaction!(resume);
    generate_method_transaction!(adapted_body_discard);
    generate_method_transaction!(adapted_body_make);
    generate_method_transaction!(adapted_body_make_more);
    generate_method_transaction!(adapted_body_stop_making);
    generate_method_transaction!(adapted_body_pause);
    generate_method_transaction!(adapted_body_resume);

    fn adapted_body_content<'a>(
        &mut self,
        host: &'a mut (dyn ErasedTransaction<dyn ErasedHost> + 'static),
        offset: usize,
        size: usize,
    ) -> Area {
        U::adapted_body_content(self, host, offset, size)
    }
    fn adapted_body_content_shift<'a>(
        &mut self,
        host: &'a mut (dyn ErasedTransaction<dyn ErasedHost> + 'static),
        size: usize,
    ) {
        U::adapted_body_content_shift(self, host, size)
    }
    fn virgin_body_content_done<'a>(
        &mut self,
        host: &'a mut (dyn ErasedTransaction<dyn ErasedHost> + 'static),
        at_end: bool,
    ) {
        U::virgin_body_content_done(self, host, at_end)
    }

    generate_method_transaction!(virgin_body_content_available);
}

macro_rules! generate_method_transaction_1 {
    ($name:ident) => {
        fn $name<'a>(&mut self, host: &'a mut (dyn ErasedTransaction<dyn ErasedHost> + 'static))
        where
            (dyn ErasedTransaction<dyn ErasedHost> + 'static): 'a,
        {
            Self::$name(self, host)
        }
    }
}

impl ecap::adapter::Transaction<dyn ErasedHost> for dyn Transaction {
    generate_method_transaction_1!(start);
    generate_method_transaction_1!(stop);
    generate_method_transaction_1!(resume);
    generate_method_transaction_1!(adapted_body_discard);
    generate_method_transaction_1!(adapted_body_make);
    generate_method_transaction_1!(adapted_body_make_more);
    generate_method_transaction_1!(adapted_body_stop_making);
    generate_method_transaction_1!(adapted_body_pause);
    generate_method_transaction_1!(adapted_body_resume);

    fn adapted_body_content<'a>(
        &mut self,
        host: &'a mut (dyn ErasedTransaction<dyn ErasedHost> + 'static),
        offset: usize,
        size: usize,
    ) -> Area
    where
        (dyn ErasedTransaction<dyn ErasedHost> + 'static): 'a,
    {
        Self::adapted_body_content(self, host, offset, size)
    }
    fn adapted_body_content_shift<'a>(
        &mut self,
        host: &'a mut (dyn ErasedTransaction<dyn ErasedHost> + 'static),
        size: usize,
    ) where
        (dyn ErasedTransaction<dyn ErasedHost> + 'static): 'a,
    {
        Self::adapted_body_content_shift(self, host, size)
    }
    fn virgin_body_content_done<'a>(
        &mut self,
        host: &'a mut (dyn ErasedTransaction<dyn ErasedHost> + 'static),
        at_end: bool,
    ) where
        (dyn ErasedTransaction<dyn ErasedHost> + 'static): 'a,
    {
        Self::virgin_body_content_done(self, host, at_end)
    }

    generate_method_transaction_1!(virgin_body_content_available);
}

impl<'a> ecap::common::Options for dyn Transaction + 'a {
    fn option(&self, name: &Name) -> Option<Area> {
        self.option(name)
    }

    fn visit_each<V: NamedValueVisitor>(&self, mut visitor: V) {
        self.visit_each(&mut visitor)
    }
}
