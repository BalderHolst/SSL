# This Makefile was generated by Nix.
# To make changes edit nix configuration.

main: thelp

thelp:
	@echo "usage: make <task>"
	@echo ""
	@echo "Available Tasks:"
	@echo -e '	build'
	@echo -e '	check-examples'
	@echo -e '	gen-scripts'
	@echo -e '	record-examples'
	@echo -e '	run-examples'
	@echo -e "\nUse 'make thelp' command to show this list."
	

build: 
	cargo build --release
	


check-examples: 
	ls ./examples | xargs -I{} bash -c \
	    "echo 'Checking example {}' ; ./target/release/ssl examples/{} --ast | diff - tests/{}.ast"
	


gen-scripts: 
	nix run .#gen-scripts


record-examples: build
	ls ./examples | xargs -I{} bash -c \
	    "mkdir -p tests/ ; echo 'Recording example {}' ; ./target/release/ssl examples/{} --ast > tests/{}.ast"
	


run-examples: build
	ls ./examples | xargs -I{} bash -c \
	    "mkdir -p art/ ; echo 'Running example {}'; ./target/release/ssl examples/{} --output art/{}.png"
	
