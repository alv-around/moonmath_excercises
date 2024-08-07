To compile circom:

```bash
circom ex101.circom --r1cs --wasm --sym --c
```

to calculate witness

```bash
cd ex101_js
node generate_witness.js ex101.wasm ../ex101_witness.json witness.wtns
```