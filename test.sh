cargo watch -q -c -w app/src -x run  
cargo watch -q -c -w app/tests -x "test --package app --test quick_dev -- quick_dev --exact --nocapture"