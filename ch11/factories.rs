// Define the Service trait.
trait Service {
    fn perform_action(&self);
}

// Define a ConcreteImpl struct which implements Service.
struct ConcreteImpl;

impl Service for ConcreteImpl {
    fn perform_action(&self) {
        println!("ConcreteImpl is performing an action.");
    }
}

// Define the ServiceFactory trait.
trait ServiceFactory {
    fn make_svc(&self) -> Box<dyn Service>;
}

// Define the ServiceFactoryImpl struct.
struct ServiceFactoryImpl;

impl ServiceFactory for ServiceFactoryImpl {
    fn make_svc(&self) -> Box<dyn Service> {
        Box::new(ConcreteImpl)
    }
}

fn main() {
    let factory = ServiceFactoryImpl;
    let service = factory.make_svc();
    service.perform_action();
}
