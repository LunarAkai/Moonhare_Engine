run:
    cargo run

test:
    cargo test --verbose
    
doc:
    cargo doc --no-deps --package moonhare_engine     

fmt:
    cargo fmt    

push commit_message:
    ./push_to_git.sh "{{commit_message}}"    