#!/bin/bash
set -e
wasm-pack build
cd www
npm install
npm start
