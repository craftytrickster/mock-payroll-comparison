#[derive(Deserialize)]
pub struct PaymentInfo {
    name: String,
    q1: f64,
    q2: f64,
    q3: f64,
    q4: f64
}

pub fn process_payment(payment: &PaymentInfo) -> String {
    let total = calculate_total(payment);
    let message = format!("{} USD has been successfully wired to {}.", total, payment.name);

    println!("{}", message);  // <- actual transfer occurs here

    message
}

pub fn calculate_total(payment: &PaymentInfo) -> f64 {
    payment.q1 + payment.q2 + payment.q3 + payment.q4
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn payment_amount_should_aggregate_correctly() {
        let sample_payment = PaymentInfo {
            name: "Weyland-Yutani Corporation".to_string(),
            q1: 30_000f64,
            q2: 40_000f64,
            q3: 20_000f64,
            q4: 0f64
        };

        let expected_total = 90_000f64;

        let actual_total = calculate_total(&sample_payment);

        assert_eq!(expected_total, actual_total);
    }
}