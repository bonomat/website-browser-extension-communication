import rust from "@wasm-tool/rollup-plugin-rust";
import copy from 'rollup-plugin-copy'

export default {
  input: {
    background: "src/background/Cargo.toml",
  },
  output: {
    dir: "dist",
    format: "esm",
    sourcemap: true,
    // TODO source map URL is missing the js/
    entryFileNames: "js/[name].js",
  },
  plugins: [
    rust({
      outDir: "js",
      importHook: function (path) {
        return "browser.runtime.getURL(" + JSON.stringify(path) + ")";
      },
    }),
    copy({
      targets: [
        { src: 'static/*', dest: 'dist' },
      ]
    })
  ],
};