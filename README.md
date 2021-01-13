[![pipeline status](https://gitlab.com/etrovub/smartnets/flickrs/badges/master/pipeline.svg)](https://gitlab.com/etrovub/smartnets/flickrs/-/commits/master)
[![coverage report](https://gitlab.com/etrovub/smartnets/flickrs/badges/master/coverage.svg)](https://gitlab.com/etrovub/smartnets/flickrs/-/commits/master)
[![client docs](https://img.shields.io/badge/client%20doc-master-blue)](https://etrovub.gitlab.io/smartnets/flickrs/doc/flickrs_client/)
[![server docs](https://img.shields.io/badge/server%20doc-master-blue)](https://etrovub.gitlab.io/smartnets/flickrs/doc/flickrs_server/)

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
