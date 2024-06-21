
expand_fyrox() {
	cargo expand --package "$1" > expand/$1.rs
}

mkdir -p expand

expand_fyrox fyrox-animation
expand_fyrox fyrox-core
expand_fyrox fyrox-core-derive
expand_fyrox fyrox-dylib
expand_fyrox fyrox-graph
expand_fyrox fyrox-impl
expand_fyrox fyrox-math
expand_fyrox fyrox-resource
expand_fyrox fyrox-scripts
expand_fyrox fyrox-sound
expand_fyrox fyrox-ui
