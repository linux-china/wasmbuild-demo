wasmbuild demo
===============

wasmbuild: a build tool to generate wasm-bindgen glue code for Deno and the browser.

# How to bundle app?

* Build project with Wasm module inline as base64 text:  `deno run -A https://deno.land/x/wasmbuild@0.8.0/main.ts --sync`
* Bundle project: `mkdir -p dist ; deno bundle demo.ts dist/bundle.js`

# References

* wasmbuild: https://github.com/denoland/wasmbuild
* wasm-bindgen: https://github.com/rustwasm/wasm-bindgen
