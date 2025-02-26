wasm-pack build .\botc\ --target web --release --out-dir=../site/compiler/botc || (echo "Failed to build botc" && exit)
del .\site\compiler\botc\.gitignore
wasm-pack build .\torland\ --target web --release --out-dir=../site/torland || (echo "Failed to build torland" && exit)
del .\site\torland\.gitignore
