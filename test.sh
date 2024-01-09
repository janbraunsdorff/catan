cargo watch -q -c -w app/src -x run  
cargo watch -q -c -w app/tests -x "test --package app --test quick_dev -- quick_dev --exact --nocapture"

 cargo watch  -q -c -w ./catan_core -x "test --package catan_core  -- --nocapture" 

 cargo fmt --all  