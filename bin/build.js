import { writeFile, readFile } from 'fs/promises';
import { join } from 'path';
import { icons } from 'feather-icons';

(async () => {
    const reservedKeywords = ['box', 'move', 'type'];

    const sanitize = (string) => (reservedKeywords.includes(string) ? `r#${string}` : string);

    const slugContents = (await readFile(join('bin', 'slug.rs'))).toString();

    const slug = (name, markup) =>
        slugContents.replaceAll('[name]', name).replaceAll('[markup]', markup);

    const capitalize = (string) => string[0].toUpperCase() + string.substring(1);

    const toPascal = (string) => string.split('-').map(capitalize).join('');

    const underscore = (string) => string.replaceAll('-', '_');

    const modify = (string) => `pub mod ${sanitize(underscore(string))};\n`;

    const mod = Object.keys(icons).map(modify).join('');

    const exportContent = `#![recursion_limit = "1024"]\n\n${mod}`;

    await writeFile(join('src', 'lib.rs'), exportContent);

    await Promise.all(
        Object.entries(icons).map(([k, { contents }]) =>
            writeFile(join('src', `${underscore(k)}.rs`), slug(toPascal(k), contents))
        )
    );
})().catch(console.error);
