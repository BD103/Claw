mod cache;

pub use self::cache::QueryCache;

pub trait Queryable<T> {
    fn query(&mut self) -> T;
}

macro_rules! create_ctx {
    ($scope:vis $name:ident {
        $(query $qname:ident: $qtype:ty {
            $source:expr $(=> $provider:expr)*
        }),*
    }) => {
        $scope struct $name {
            $($qname: ::crate::query::QueryCache<$qtype>),*
        }

        impl $name {
            pub fn new() -> Self {
                $(
                    #[allow(unused_mut)]
                    let mut $qname = ::crate::query::QueryCache::new($source);
                    $($qname.add_provider($provider);)*
                )*

                $name {
                    $($qname),*
                }
            }
        }

        $(
            impl ::crate::query::Queryable<$qtype> for $name {
                fn query(&mut self) -> $qtype {
                    self.$qname.query()
                }
            }
        )*
    };
}

create_ctx!(
    pub Ctx {}
);
