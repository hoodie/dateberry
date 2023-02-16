default: dev
alias b := build
alias d := dev

ui:
    yarn --cwd $PWD/app vite dev

app: 
    yarn --cwd $PWD/app vite build
    cp app/dist/index.html src/

build: app
    cargo build

serve: 
    cargo run --example local
    
dev: app
    @just serve
#   cargo shuttle run
