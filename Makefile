.PHONY: watch-build watch-example

watch-build: 
	@ ls **/* | entr cargo build

watch-example:	
	@ ls **/* | entr cargo run --example linked_list_example
