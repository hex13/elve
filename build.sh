wasm-pack build --release --target web
cp -r pkg public
cp pkg/elve_bg.wasm public
npx esbuild --bundle public/index.js --format="esm" --outfile=public/out.js
