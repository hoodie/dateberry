alias b := build
alias d := devt 

app: 
    yarn --cwd app vite build
    cp app/dist/index.html src/

build: app
    cargo build

serve: 
    cargo run --example local
    
dev: app
    @just serve
#   cargo shuttle run
