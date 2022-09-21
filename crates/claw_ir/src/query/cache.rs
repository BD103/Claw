use std::collections::VecDeque;

pub struct QueryCache<T: Clone> {
    /// The cached output of [`provider`].
    cache: Option<T>,

    /// Function that computes the value of [`source`].
    provider: VecDeque<fn(T) -> T>,

    /// The original data in the function.
    source: T,
}

impl<T: Clone> QueryCache<T> {
    pub fn new(source: T) -> Self {
        QueryCache {
            cache: None,
            provider: VecDeque::new(),
            source,
        }
    }

    /// Queries the value, using the cache if possible.
    pub fn query(&mut self) -> T {
        if let Some(res) = &self.cache {
            res.clone()
        } else {
            let res = self.query_no_cache();
            self.cache = Some(res.clone());
            res
        }
    }

    /// Queries the value without accessing or updating the cache.
    pub fn query_no_cache(&self) -> T {
        self.provider
            .iter()
            .fold(self.source.clone(), |res, provider| provider(res))
    }

    pub fn clear_cache(&mut self) {
        self.cache = None;
    }

    pub(crate) fn add_provider(&mut self, provider: fn(T) -> T) {
        self.clear_cache();
        self.provider.push_back(provider);
    }

    #[allow(dead_code)]
    pub(crate) fn add_provider_front(&mut self, provider: fn(T) -> T) {
        self.clear_cache();
        self.provider.push_front(provider);
    }

    #[allow(dead_code)]
    pub(crate) fn clear_providers(&mut self) {
        self.provider.clear();
    }
}

impl<T: Clone + Default> Default for QueryCache<T> {
    fn default() -> Self {
        QueryCache {
            cache: None,
            provider: VecDeque::new(),
            source: T::default(),
        }
    }
}
