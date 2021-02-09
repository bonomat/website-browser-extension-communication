import rust from "@wasm-tool/rollup-plugin-rust";

export default {
  input: {
    content: "./Cargo.toml",
  },
  output: {
    dir: "dist",
    format: "iife",
    sourcemap: true,
    entryFileNames: "js/[name].js",
  },
  plugins: [
    rust({
      outDir: "js",
      importHook: function (path) {
        return "browser.runtime.getURL(" + JSON.stringify(path) + ")";
      },
    }),
  ],
};