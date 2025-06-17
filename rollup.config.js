import { readFileSync } from 'fs'
import { dirname, join } from 'path'
import { cwd } from 'process'
import { nodeResolve } from '@rollup/plugin-node-resolve'
import typescript from '@rollup/plugin-typescript'
import terser from '@rollup/plugin-terser'

const input = "guest-js/index.ts"

const pkg = JSON.parse(readFileSync(join(cwd(), 'package.json'), 'utf8'))

const pluginJsName = pkg.name
  .replace('tauri-plugin-', '')
  .replace(/-./g, (x) => x[1].toUpperCase())

const iifeVarName = `__TAURI_PLUGIN_${pkg.name
  .replace('tauri-plugin-', '')
  .replace('-', (x) => '_')
  .toUpperCase()}__`

export default [
  {
    input,
    output: [
      {
        file: pkg.exports.import,
        format: 'esm'
      },
      {
        file: pkg.exports.require,
        format: 'cjs'
      }
    ],
    plugins: [
      typescript({
        declaration: true,
        declarationDir: dirname(pkg.exports.import)
      })
    ],
    external: [
      /^@tauri-apps\/api/,
      ...Object.keys(pkg.dependencies || {}),
      ...Object.keys(pkg.peerDependencies || {})
    ]
  },
  {
    input,
    output: {
      format: "iife",
      name: iifeVarName,
      banner: "if ('__TAURI__' in window) {",
      footer: `Object.defineProperty(window.__TAURI__, '${pluginJsName}', { value: ${iifeVarName} }) }`,
      file: "api-iife.js"
    },
    plugins: [typescript(), terser(), nodeResolve()],
    onwarn: (warning) => {
      throw Object.assign(new Error(), warning)
    }
  }
]
