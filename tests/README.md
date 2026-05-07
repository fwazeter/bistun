### How to Run the Stress Test
Execute the test using the following command. We use `--nocapture` so you can see the microsecond metrics in real-time.

```bash
cargo test --test stress_tests -- --nocapture
```