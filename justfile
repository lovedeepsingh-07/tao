default:
	@just -l

run example:
	- cargo run --example {{example}}
