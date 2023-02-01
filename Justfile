alias b := build
alias r := run

app: 
    yarn --cwd app vite build
    cp app/dist/index.html src/

build: app
    cargo build

run:  build
    cargo shuttle run