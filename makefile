push:
	git add .
	git commit -m "init"
	git push origin master


minigrep:
	cargo run -- mi divina_commedia.txt > output.txt

minigrep-insensitive:
	IGNORE_CASE=1 cargo run -- MI divina_commedia.txt
