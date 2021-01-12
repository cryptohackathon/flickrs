[![pipeline status](https://gitlab.com/etrovub/smartnets/flickrs/badges/master/pipeline.svg)](https://gitlab.com/etrovub/smartnets/flickrs/-/commits/master)
[![coverage report](https://gitlab.com/etrovub/smartnets/flickrs/badges/master/coverage.svg)](https://gitlab.com/etrovub/smartnets/flickrs/-/commits/master)

# FlickRS
![FlickRS logo](./flick-rs.png)
## How to run client

In `./flickrs-client/` run the following commands:
```
wasm-pack build

or 

wasm-pack build --release
```

This will generate the wasm file.

After generating the wasm file, run the following commands in `./flickrs-client/flick-rs/`:
```
npm install
npm start
```
