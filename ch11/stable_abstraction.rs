// Don’t refer to volatile concrete classes. Refer to abstract interfaces instead.

// Define an Abstract Interface
trait MessageService {
    fn send(&self, recipient: &str, message: &str);
}

// Implement Concrete Classes
struct EmailService;

impl MessageService for EmailService {
    fn send(&self, recipient: &str, message: &str) {
        println!("Sending email to {}: {}", recipient, message);
    }
}

struct SMSService;

impl MessageService for SMSService {
    fn send(&self, recipient: &str, message: &str) {
        println!("Sending SMS to {}: {}", recipient, message);
    }
}

// Use an Abstract  Factory
enum ServiceType {
    Email,
    SMS,
}

fn get_message_service(service_type: ServiceType) -> Box<dyn MessageService> {
    match service_type {
        ServiceType::Email => Box::new(EmailService),
        ServiceType::SMS => Box::new(SMSService),
    }
}

fn main() {
    let service_type = ServiceType::Email;
    let email_service = get_message_service(service_type);
    email_service.send("James", "Hello from Email");

    let service_type = ServiceType::SMS;
    let sms_service = get_message_service(service_type);
    sms_service.send("Tim", "Hello from SMS");
}
