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
            $($qname: QueryCache<$qtype>),*
        }

        impl $name {
            pub fn new() -> Self {
                $(
                    #[allow(unused_mut)]
                    let mut $qname = QueryCache::new($source);
                    $($qname.add_provider($provider);)*
                )*

                $name {
                    $($qname),*
                }
            }
        }

        $(
            impl Queryable<$qtype> for $name {
                fn query(&mut self) -> $qtype {
                    self.$qname.query()
                }
            }
        )*
    };
}

create_ctx!(pub Ctx {});
