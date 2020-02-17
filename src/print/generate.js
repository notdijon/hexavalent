const fs = require('fs');
const path = require('path');

function* readDescriptions() {
	const descriptions = fs.readFileSync(path.join(__dirname, 'text.c'), 'utf8');
	const commentLines = 18;
	const lines = descriptions.split('\n').slice(commentLines);

	for (let i = 0; i < lines.length;) {
		const [, key] = (/const\s+(\w+)\[]/).exec(lines[i]);
		++i;
		const fields = [];
		while (!lines[i].startsWith('}')) {
			const [, field] = (/N_\("(.+)"\)/).exec(lines[i]) || (/\s"(.+)"\s*$/).exec(lines[i]);
			++i;
			fields.push(field);
		}
		++i;
		while (lines[i] === '') ++i;
		yield {
			key,
			fields,
		}
	}
}

function* readTextEvents() {
	const textEvents = fs.readFileSync(path.join(__dirname, 'textevents.in'), 'utf8');
	const lines = textEvents.split('\n');
	for (let i = 0; i < lines.length; i += 6) {
		yield {
			name: lines[i + 0],
			signal: lines[i + 1],
			fields_key: lines[i + 2],
			format: lines[i + 3],
			field_count_maybe: lines[i + 4],
		};
	}
}

function nameToCamelCase(spacedName) {
	const words = spacedName.split(/\W/);

	return words.map(w => {
		const first = w[0];
		const rest = w.slice(1);
		return first.toUpperCase() + rest.toLowerCase();
	}).join('');
}

function* generateRustLines() {
	const field_descriptions = Object.create(null);

	field_descriptions.pevt_generic_none_help = [];

	for (const { key, fields } of readDescriptions()) {
		field_descriptions[key] = fields;
	}

	for (const { name, fields_key, format } of readTextEvents()) {
		yield `print_event!(${nameToCamelCase(name)}, "${name}", "${format}", ${field_descriptions[fields_key].map((field, i) => `${i}: "${field}"`).join(', ')});`;
	}
}

function main() {
	const file = fs.createWriteStream(path.join(__dirname, 'events.rs'), 'utf8');

	for (const line of generateRustLines()) {
		file.write(line);
		file.write('\n');
	}

	file.end();
}

main();
