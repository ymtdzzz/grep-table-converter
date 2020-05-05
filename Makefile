# Option
#========================================
LOG_LEVEL := debug
APP_ARGS := ""

# Environment
#========================================
export RUST_LOG=grepross=${LOG_LEVEL}

# Task
#========================================
run:
	cargo run ${APP_ARGS}

test:
	cargo test

check:
	cargo check ${OPTION}
