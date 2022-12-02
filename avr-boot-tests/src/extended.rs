use super::avr;

#[test]
fn simple() {
    run_test("extended_simple", "atmega1280");
}

#[test]
fn page() {
    run_test("extended_page", "atmega1280");
}

fn run_test(test: &str, target: &str) {
    let mut avr = avr(test, target);
    avr.run_for_ms(10);
    let expected = "Start\nWrote 128 words at address 65536\nCheck pass\n".to_string();

    assert_eq!(expected, avr.uart0().recv::<String>());
}
