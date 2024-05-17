#!/bin/sh 

cd lib
wasm-pack build --target web

cd ../site
npm run dev