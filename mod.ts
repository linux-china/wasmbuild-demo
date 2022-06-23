import {instantiate} from "./lib/wasm_demo.generated.js";
// initialize Wasm
await instantiate();

export {add, hello} from "./lib/wasm_demo.generated.js";

