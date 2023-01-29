alias b := build
alias r := run
build: 
    yarn --cwd app vite build
    cargo build

run:  build
    cargo shuttle run