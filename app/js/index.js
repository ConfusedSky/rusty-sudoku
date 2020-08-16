// Wasm must be loaded async so we import async at head then not worry about it
import("./start.jsx").catch(console.error);
