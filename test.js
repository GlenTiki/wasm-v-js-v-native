const wasm = require('./build/Release/primes.wasm.js');
const assert = require('assert')

console.log(wasm.sqrt(17))
assert(wasm.sqrt(9) === 3, 'should be equal')
assert(wasm.sqrt(16) === 4, 'should be equal')
assert(wasm.sqrt(25) === 5, 'should be equal')

assert(wasm.prime(7) === 17, `Looks like your wasm is not right ${wasm.prime(7)}`)
