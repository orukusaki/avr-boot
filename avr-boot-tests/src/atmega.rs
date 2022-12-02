use super::avr;

#[test]
fn simple() {
    run_test("simple", "atmega328", 64);
}

#[test]
fn buffer() {
    run_test("buffer", "atmega328", 64);
}

#[test]
fn buffer_store_from_slice() {
    run_test("buffer_store_from_slice", "atmega328", 64);
}
#[test]
fn buffer_fill_from_slice() {
    run_test("buffer_fill_from_slice", "atmega328", 64);
}

#[test]
fn buffer_fill_from_fn() {
    run_test("buffer_fill_from_fn", "atmega328", 64);
}

#[test]
fn buffer_fill_from_iter() {
    run_test("buffer_fill_from_iter", "atmega328", 64);
}

#[test]
fn page() {
    run_test("page", "atmega328", 64);
}

#[test]
fn page_644() {
    run_test("page", "atmega644", 128);
}

fn run_test(test: &str, target: &str, page_size: usize) {
    let mut avr = avr(test, target);
    avr.run_for_ms(10);
    let expected = format!(
        "Start\nWrote {} words at address 1792\nCheck pass\n",
        page_size
    );

    assert_eq!(expected, avr.uart0().recv::<String>());
}
