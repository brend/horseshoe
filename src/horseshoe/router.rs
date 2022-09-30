use std::collections::HashMap;

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
    routes: HashMap<String, Callback<Box<dyn Fn() -> ()>>>
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::<String, Callback<Box<dyn Fn() -> ()>>>::new()
        }
    }

    pub fn handle(&self, method: &str, path: &str) {
        if let Some(callback) = self.routes.get(&path.to_string()) {
            (callback.handler)();
        } else {
            println!("Unhandled route");
        }
    }

    pub fn get<F>(&mut self, path: &str, handler: F)
    where F: Fn() + 'static
    {
        let callback: Callback<Box<dyn Fn() -> ()>> = Callback::new(Box::new(handler));

        self.routes.insert(path.to_string(), callback);
    }
}