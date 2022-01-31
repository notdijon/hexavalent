const fs = require('fs');
const path = require('path');

function repeatWhile(callback) {
	const arr = [];
	let x;
	while ((x = callback())) {
		arr.push(x);
	}
	return arr;
}

function allMatches(str, regex) {
	return repeatWhile(() => regex.exec(str));
}

function main() {
	const input = fs.readFileSync(path.join(__dirname, 'handle_orig.rs'), 'utf8');

	const output = input.replace(/unsafe fn (\w+)\((?:.|\n)+?\{/g, (fullMatch, name) => {
		const [, firstArgDepth] = /(\n\s+?)\w+?:/g.exec(fullMatch) || [null, ''];

		const argRegex = new RegExp(`${firstArgDepth}(\\w+):`, 'g');

		const args = allMatches(fullMatch, argRegex).map(([, name]) => name).slice(1 /* skip ph */);

		const matchWithoutPhArg = fullMatch.replace(/ph: \*mut hexchat_plugin,\n?/g, '');

		return matchWithoutPhArg + `// Safety: forwarded to caller
			unsafe { ((*self.handle.as_ptr()).${name})(self.handle.as_ptr(), ${args.join(',')}) }
		`;
	});

	fs.writeFileSync(path.join(__dirname, 'handle.rs'), output, 'utf8');
}

main();
