use std::collections::HashMap;

type Method = String;
type Route = String;

pub struct Callback<F>
where
    F: Fn() -> (),
{
    pub handler: F,
}

impl<F> Callback<F>
where
    F: Fn() -> (),
{
    pub fn new(handler: F) -> Self {
        Self { handler }
    }
}

pub struct Router {
    routes: HashMap<Method, HashMap<Route, Vec<Callback<Box<dyn Fn() -> ()>>>>>
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn handle(&self, method: &str, path: &str) {
        if let Some(routes_for_method) = self.routes.get(&method.to_string()) {
            if let Some(callbacks) = routes_for_method.get(&path.to_string()) {
                
                for callback in callbacks {
                    (callback.handler)();
                }

            } else {
                println!("No \"{}\" route registered for path \"{}\"", method, path);
            }
        } else {
            println!("No routes registered for method \"{}\"", method);
        }
    }

    pub fn add<F>(&mut self, method: &str, path: &str, handler: F)
    where F: Fn() + 'static
    {
        let method = &method.to_uppercase().to_string();
        let callback: Callback<Box<dyn Fn() -> ()>> = Callback::new(Box::new(handler));

        // get routes for method
        if !self.routes.contains_key(method) {
            self.routes.insert(String::from(method), HashMap::new());
        }

        let routes_for_method = self.routes.get_mut(method).unwrap();

        // get handlers for path
        if !routes_for_method.contains_key(&path.to_string()) {
            routes_for_method.insert(path.to_string(), Vec::new());
        }

        let handlers = routes_for_method.get_mut(&path.to_string()).unwrap();

        // add new handler
        handlers.push(callback);
    }
}