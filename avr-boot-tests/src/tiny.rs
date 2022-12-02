use super::avr;

// simavr doesn't support spm for attinx8 yet, check https://github.com/buserror/simavr/pull/513
#[test]
#[ignore]
fn simple() {
    run_test("simple", "attiny85");
}

// simavr doesn't support spm for attinx8 yet, check https://github.com/buserror/simavr/pull/513
#[test]
#[ignore]
fn page() {
    run_test("page", "attiny85");
}

fn run_test(test: &str, target: &str) {
    let mut avr = avr(test, target);
    avr.run_for_ms(10);
    let expected = "Start\nWrote 32 words at address 1792\nCheck pass\n".to_string();

    assert_eq!(expected, avr.uart0().recv::<String>());
}
