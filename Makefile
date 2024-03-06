rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt --quiet

install:
	# Install if needed
	#@echo "Updating rust toolchain"
	#rustup update stable
	#rustup default stable 

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

watch:
	cargo lambda watch

build: 
	cargo lambda build --release 

deploy:
	cargo lambda deploy 

aws-invoke: 
	cargo lambda invoke --remote max_stock_profit_with_logging_and_tracing --data-ascii "{"prices": [3,2,6,5,0,3], "k": 2}"

all: format lint test 
