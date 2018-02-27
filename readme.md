# wasm-v-js-v-native

A repo to benchmark web assembly generated with rust against javascript and a native c++ addon.

Heavily inspired by and uses code from: https://medium.com/the-node-js-collection/speed-up-your-node-js-app-with-native-addons-5e76a06f4a40

## setup

To run the benchmarks, you need to rust setup to compile to wasm-unknown-unknown. Check this repo for instruction on how to do that: https://github.com/rust-lang-nursery/rust-wasm

You'll also need wasm-gc installed: https://github.com/alexcrichton/wasm-gc

Clone this repo

Run `npm install`

then run `npm run benchmark`

Benchmarking output is valid csv. I dropped this into the calcs.csv with some column titles and uploaded to google sheets to generate my chart.

## License
MIT