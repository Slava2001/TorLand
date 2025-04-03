wasm-pack build .\botc\ --target web --release --out-dir=../site/TorLand/compiler/botc || (echo "Failed to build botc" && exit)
del .\site\TorLand\compiler\botc\.gitignore
wasm-pack build .\torland\ --target web --release --out-dir=../site/TorLand/simulation/torland || (echo "Failed to build torland" && exit)
del .\site\TorLand\simulation\torland\.gitignore
