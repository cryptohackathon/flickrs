FROM rustlang/rust:nightly as wasm-builder

RUN cargo install wasm-pack

COPY . .
RUN wasm-pack build --release

FROM node as react-builder

COPY flick-rs flick-rs
COPY --from=wasm-builder pkg pkg

RUN cd flick-rs && npm install && npm run build

FROM nginx

COPY --from=react-builder flick-rs/build /usr/share/nginx/html
RUN echo "types { application/wasm wasm; }" > /etc/nginx/conf.d/wasm-mime.conf
