import rust from "@wasm-tool/rollup-plugin-rust";
import copy from 'rollup-plugin-copy'

export default [
  {
    input: {
      background: "src/background/Cargo.toml",
    },
    output: {
      dir: "dist",
      format: "esm",
      entryFileNames: "js/[name].js",
    },
    plugins: [
      rust({
        importHook: function (path) {
          return "browser.runtime.getURL(" + JSON.stringify(path) + ")";
        },
      }),
      copy({
        targets: [
          {src: 'static/*', dest: 'dist'},
        ]
      })
    ],
  },
  {
    input: {
      content: "src/content/Cargo.toml"
    },
    output: {
      dir: "dist",
      format: "iife",
      entryFileNames: "js/[name].js",
    },
    plugins: [
      rust({
        importHook: function (path) {
          return "browser.runtime.getURL(" + JSON.stringify(path) + ")";
        },
      }),
      copy({
        targets: [
          {src: 'src/content/dist/*', dest: 'dist'},
        ]
      })
    ],
  },
  {
    input: {
      in_page: "src/in_page/Cargo.toml"
    },
    output: {
      dir: "dist",
      format: "esm",
      entryFileNames: "js/[name].js",
    },
    plugins: [
      rust({
        inlineWasm: true
      }),
      copy({
        targets: [
          {src: 'src/in_page/dist/*', dest: 'dist'},
        ]
      })
    ],
  },
  {
    input: {
      popup: "Cargo.toml"
    },
    output: {
      dir: "dist",
      format: "esm",
      entryFileNames: "js/[name].js",
    },
    plugins: [
      rust({
        importHook: function (path) {
          return "browser.runtime.getURL(" + JSON.stringify(path) + ")";
        },
      }),
      copy({
        targets: [
          {src: 'static/*', dest: 'dist'},
        ]
      })
    ],
  }
];