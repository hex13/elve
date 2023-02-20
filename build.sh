wasm-pack build --release --target web
cp -r pkg frontend
cp pkg/elve_bg.wasm public
npx esbuild --bundle frontend/index.js --format="esm" --loader:.js=jsx --outfile=public/out.js
