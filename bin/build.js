import { writeFile, readFile, mkdir } from 'fs/promises';
import { join } from 'path';
import { icons } from 'feather-icons';

(async () => {
    const outDir = join(process.env['OUT_DIR'], 'feather-src');

    const reservedKeywords = ['box', 'move', 'type'];

    const sanitize = (string) => (reservedKeywords.includes(string) ? `r#${string}` : string);

    const slugContents = (await readFile(join('bin', 'slug.rs'))).toString();

    const slug = (name, markup) =>
        slugContents.replaceAll('[name]', name).replaceAll('[markup]', markup);

    const capitalize = (string) => string[0].toUpperCase() + string.substring(1);

    const toPascal = (string) => string.split('-').map(capitalize).join('');

    const underscore = (string) => string.replaceAll('-', '_');

    const modify = (k) => `
    #[path = concat!(env!("OUT_DIR"), "/feather-src/${underscore(k)}.rs")]
    pub mod ${sanitize(underscore(k))};
    `;

    const mod = Object.keys(icons).map(modify).join('');

    await mkdir(outDir, { recursive: true });

    await writeFile(join(outDir, 'lib.rs'), mod);

    await Promise.all(
        Object.entries(icons).map(([k, { contents }]) =>
            writeFile(join(outDir, `${underscore(k)}.rs`), slug(toPascal(k), contents))
        )
    );
})().catch((err) => {
    console.error(err);
    process.exit(1);
});
