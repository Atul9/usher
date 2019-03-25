//! HTTP routing components based on an Usher `Router`.
//!
//! This module exposes the `HttpRouter` structure, which represents a
//! generic `Router` in a more HTTP appropriate way. Rather than the
//! typical insertion functions, this router exposes HTTP verbs as the
//! names of functions to provide a simple API for mapping HTTP requests.
use http::Method;

use std::collections::HashMap;

use crate::parser::Parser;
use crate::router::Router;

/// A basic HTTP routing structure for generic handlers.
///
/// Almost all internals of this router are controlled by the usual `Router`,
/// with this structure simply providing a more HTTP friendly API for ergonomics.
///
/// To construct a router this way, HTTP verbs must be used as naturally there must
/// be a verb associated with each request. There is currently no way to match any
/// verb, although this will be improved at some point in future.
pub struct HttpRouter<T> {
    router: Router<HashMap<Method, T>>,
}

/// Delegates a HTTP method to the `route` method in a router.
macro_rules! http_delegate {
    ($name:ident, $method:expr) => {
        #[inline(always)]
        pub fn $name(&mut self, path: &str, t: T) {
            self.insert($method, path, t)
        }
    };
}

impl<T> HttpRouter<T> {
    /// Creates a new `Router` with provided matchers.
    pub fn new(parsers: Vec<Box<Parser>>) -> Self {
        Self {
            router: Router::new(parsers),
        }
    }

    // Automatic HTTP method delegates.
    http_delegate!(connect, Method::CONNECT);
    http_delegate!(delete, Method::DELETE);
    http_delegate!(get, Method::GET);
    http_delegate!(head, Method::HEAD);
    http_delegate!(options, Method::OPTIONS);
    http_delegate!(patch, Method::PATCH);
    http_delegate!(post, Method::POST);
    http_delegate!(put, Method::PUT);
    http_delegate!(trace, Method::TRACE);

    /// Inserts a route/handler pair for the provided method and path.
    fn insert(&mut self, method: Method, path: &str, t: T) {
        self.router.update(path, |node| {
            let mut map = node.unwrap_or_else(HashMap::new);
            map.reserve(1);
            map.insert(method, t);
            map
        });
    }

    /// Attempts to route a method/path combination to a handler.
    ///
    /// If a handler exists for the provided method/path combination, it will
    /// be returned - along with any captures found during matching. If the
    /// path does not exist, or the method is not available on the path, a
    /// `None` value will be returned and a handler will not be found.
    pub fn handler<'a>(
        &self,
        method: &Method,
        path: &'a str,
    ) -> Option<(&T, Vec<(&str, &'a str)>)> {
        // look for the node in the router based on the path
        self.router.lookup(path).and_then(|(node, captures)| {
            // unpack the method and map the handler back directly
            node.get(method).map(|handler| (handler, captures))
        })
    }
}
