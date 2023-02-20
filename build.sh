wasm-pack build --release --target web
cp -r pkg frontend
cp pkg/elve_bg.wasm public
npx esbuild --bundle frontend/index.js --format="esm" --outfile=public/out.js
