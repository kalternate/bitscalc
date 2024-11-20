#!/bin/sh 

cd lib
wasm-pack build --target web

cd ../site
npm install
npm run dev