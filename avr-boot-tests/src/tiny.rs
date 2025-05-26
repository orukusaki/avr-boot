use super::avr;
use test_case::test_case;
// simavr doesn't support spm for attinx8 yet, check https://github.com/buserror/simavr/pull/513
// #[test_case("simple", "attiny24", "attiny84"; "attiny24 simple")]
// #[test_case("simple", "attiny25", "attiny85"; "attiny25 simple")]
// #[test_case("simple", "attiny44", "attiny84"; "attiny44 simple")]
// #[test_case("simple", "attiny45", "attiny85"; "attiny45 simple")]
// #[test_case("simple", "attiny84", "attiny84"; "attiny84 simple")]
// #[test_case("simple", "attiny85", "attiny85"; "attiny85 simple")]
// #[test_case("simple", "attiny2313", "attiny2313"; "attiny2313 simple")]
// #[test_case("simple", "attiny2313a", "attiny2313"; "attiny2313a simple")]
// #[test_case("simple", "attiny4313", "attiny2313"; "attiny4313 simple")]
// fn run_test(test: &str, target: &str, hal: &str) {
//     let mut avr = avr(test, target, hal);

//     avr.run_for_ms(10);
//     avr.pins().pb0().assert_high();
// }
