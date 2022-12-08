use super::avr;
use test_case::test_case;

#[test_case("buffer_fill_from_fn",      "atmega328",    "atmega328p";   "atmega328 buffer_fill_from_fn")]
#[test_case("buffer_fill_from_iter",    "atmega328",    "atmega328p";   "atmega328 buffer_fill_from_iter")]
#[test_case("buffer_store_from_bytes",  "atmega328",    "atmega328p";   "atmega328 buffer_store_from_bytes")]
#[test_case("buffer_store_from_slice",  "atmega328",    "atmega328p";   "atmega328 buffer_store_from_slice")]
#[test_case("buffer_copy_from_bytes",  "atmega328",    "atmega328p";   "atmega328 buffer_copy_from_bytes")]
#[test_case("buffer_copy_from_slice",  "atmega328",    "atmega328p";   "atmega328 buffer_copy_from_slice")]
#[test_case("buffer",                   "atmega328",    "atmega328p";   "atmega328 buffer")]
#[test_case("page",                     "atmega328",    "atmega328p";   "atmega328 page")]
#[test_case("simple",                   "atmega328",    "atmega328p";   "atmega328 simple")]
#[test_case("simple",                   "atmega164p",   "atmega168";    "atmega164p simple")]
#[test_case("simple",                   "atmega164pa",  "atmega168";    "atmega164pa simple")]
#[test_case("simple",                   "atmega168",    "atmega168";    "atmega168 simple")]
#[test_case("simple",                   "atmega168p",   "atmega168";    "atmega168p simple")]
#[test_case("simple",                   "atmega168pa",  "atmega168";    "atmega168pa simple")]
#[test_case("simple",                   "atmega324a",   "atmega328p";   "atmega324a simple")]
#[test_case("simple",                   "atmega324p",   "atmega328p";   "atmega324p simple")]
#[test_case("simple",                   "atmega324pa",  "atmega328p";   "atmega324pa simple")]
#[test_case("simple",                   "atmega328p",   "atmega328p";   "atmega328p simple")]
#[test_case("simple",                   "atmega48",     "atmega48p";    "atmega48 simple")]
#[test_case("simple",                   "atmega48p",    "atmega48p";    "atmega48p simple")]
#[test_case("simple",                   "atmega48pa",   "atmega48p";    "atmega48pa simple")]
#[test_case("simple",                   "atmega644",    "atmega328p";   "atmega644 simple")]
#[test_case("simple",                   "atmega644p",   "atmega328p";   "atmega644p simple")]
#[test_case("simple",                   "atmega88",     "atmega328p";   "atmega88 simple")]
#[test_case("simple",                   "atmega88p",    "atmega328p";   "atmega88p simple")]
#[test_case("simple",                   "atmega88pa",   "atmega328p";   "atmega88pa simple")]
#[test_case("extended_page",            "atmega1280",   "atmega1280";   "atmega1280 page")]
#[test_case("extended_simple",          "atmega1280",   "atmega1280";   "atmega1280 simple")]
#[test_case("extended_simple",          "atmega1281",   "atmega1280";   "atmega1281 simple")]
#[test_case("extended_simple",          "atmega1284",   "atmega1280";   "atmega1284 simple")]
#[test_case("extended_simple",          "atmega1284p",  "atmega1280";   "atmega1284p simple")]
#[test_case("extended_simple",          "atmega128rfr2","atmega1280";   "atmega128rfr2 simple")]
#[test_case("extended_simple",          "atmega2560",   "atmega2560";   "atmega2560 simple")]
#[test_case("extended_simple",          "atmega2561",   "atmega2560";   "atmega2561 simple")]
fn run_test(test: &str, target: &str, hal: &str) {
    let mut avr = avr(test, target, hal);
    avr.run_for_ms(10);

    avr.pins().pb0().assert_high();
}
