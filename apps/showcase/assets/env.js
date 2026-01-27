
// Polyfill for 'env' module expected by some WASM dependencies (like old wasm-timer or std)
export function now() {
    return performance.now();
}
