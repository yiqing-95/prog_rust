use std::collections::HashMap;

struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

struct BasicRouter<C> where C: Fn(&Request) -> Response {
    routes: HashMap<String, C>,
}


impl<C> BasicRouter<C> where C: Fn(&Request) -> Response {
    /// Create an empty router.
    fn new() -> BasicRouter<C> {
        BasicRouter { routes: HashMap::new() }
    }
    /// Add a route to the router.
    fn add_route(&mut self, url: &str, callback: C) {
        self.routes.insert(url.to_string(), callback);
    }
}

mod v2 {
    use super::*;

    type BoxedCallback = Box<dyn Fn(&Request) -> Response>;

    struct BasicRouter {
        routes: HashMap<String, BoxedCallback>,
    }

    impl BasicRouter {
        // Create an empty router.
        fn new() -> BasicRouter {
            BasicRouter {
                routes: HashMap::new()
            }
        }

        // Add a route to the router.
        fn add_route<C>(&mut self, url: &str, callback: C) where C: Fn(&Request) -> Response + 'static
        {
            self.routes.insert(url.to_string(), Box::new(callback));
        }
    }

    impl BasicRouter {
        fn handle_request(&self, request: &Request) -> Response {
            fn not_found_response() -> Response {
                Response {
                    code: 404,
                    headers: Default::default(),
                    body: vec![],

                }
            }

            match self.routes.get(&request.url) {
                None => not_found_response(),
                Some(callback) => callback(request)
            }
        }
    }
}

#[test]
fn test_fn_ptr() {
    // In fact, closures that don’t capture anything from their environment are identical to function pointers,
    // since they don’t need to hold any extra information about captured variables.
    let closure_ptr: fn(u32) -> u32 = |x| x + 1;
    let two = closure_ptr(1); // 2
}

struct FnPointerRouter {
    routes: HashMap<String, fn(&Request) -> Response>
}

impl FnPointerRouter {
    // Create an empty router.
    fn new() -> FnPointerRouter {
        FnPointerRouter { routes: HashMap::new() } }
    // Add a route to the router.
    fn add_route(&mut self, url: &str, callback: fn(&Request) -> Response) {
        self.routes.insert(url.to_string(), callback); }
}