import {instantiate, hello, add} from "./lib/wasm_demo.generated.js";
// initialize Wasm
await instantiate();

export {add, hello};

