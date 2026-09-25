
For the reports to use gnuplot, install it (otherwise it will use plotters)

```shell
sudo apt install gnuplot
```

Run the testbed (it will fail without)
```shell
python make.py rts
```


To avoid avoidable variations:
- close your browser
- plug in your laptop
- go away (**after** launching the benchmark)

To run a specific bench

```shell
cargo bench --bench $bench_name
```

The output is in `target/criterion/report/index.html`

If you want to keep it (just after a release for instance) copy paste the
reports in in `report/$version_$date` TODO provide a script.

```shell
cp target/criterion cli/benches/reports/dev_2026-09-05/ -r
```

To compare the run to a reference, otherwise it compares to the last one:

To save a baseline called "baseline"
```shell
cargo bench --bench upload_speed -- --save-baseline baseline
```

To compare to the saved baseline
```shell
cargo bench --bench upload_speed -- --baseline baseline
```
