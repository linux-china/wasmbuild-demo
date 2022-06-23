import {instantiate, hello, add, myip} from "./lib/wasm_demo.generated.js";
// initialize Wasm
await instantiate();

export {add, hello, myip};
