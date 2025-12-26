default:
    just --list

dev PAT="":
    cargo run -- {{ PAT }}

rel PAT="":
    cargo run --release -- {{ PAT }}

test PAT="":
    cargo test -- {{ PAT }}

bench PAT="":
    cargo bench -- {{ PAT }}

# "cargo install samply". Must open in Firefox
profile PAT="":
    samply record cargo bench --profile profile -- {{ PAT }}
