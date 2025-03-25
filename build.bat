wasm-pack build .\botc\ --target web --release --out-dir=../site/sections/TorLand/compiler/botc || (echo "Failed to build botc" && exit)
del .\site\sections\TorLand\compiler\botc\.gitignore
wasm-pack build .\torland\ --target web --release --out-dir=../site/sections/TorLand/simulation/torland || (echo "Failed to build torland" && exit)
del .\site\sections\TorLand\simulation\torland\.gitignore
