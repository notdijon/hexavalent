const fs = require('fs');
const path = require('path');

function* readPrefs() {
	const descriptions = fs.readFileSync(path.join(__dirname, 'cfgfiles.c'), 'utf8');
	const prefixLines = 2;
	const lines = descriptions.split('\n').slice(prefixLines);

	for (const line of lines) {
		if (!line) continue;
		if (!(/^\s/).test(line)) continue;
		if (line.includes('{0, 0, 0}')) continue;

		const [, name, type] = (/{"(\w+)",[^,]+, (\w+)/).exec(line);

		yield { name, type };
	}
}

function nameToCamelCase(spacedName) {
	const words = spacedName.split(/_/);

	return words.map(w => {
		const first = w[0];
		const rest = w.slice(1);
		return first.toUpperCase() + rest.toLowerCase();
	}).join('');
}

function typeToRust(type) {
	switch (type) {
		case 'TYPE_STR': return 'String';
		case 'TYPE_INT': return 'i32';
		case 'TYPE_BOOL': return 'bool';
		default: throw new Error(`Unsupported type: ${type}`);
	}
}

function* generateRustLines() {
	for (const { name, type } of readPrefs()) {
		yield `pref!(${nameToCamelCase(name)}, "${name}", ${typeToRust(type)});`;
	}
}

function main() {
	const file = fs.createWriteStream(path.join(__dirname, 'mod.rs'), 'utf8');

	for (const line of generateRustLines()) {
		file.write(line);
		file.write('\n');
	}

	file.end();
}

main();
