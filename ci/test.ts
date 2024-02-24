import { BASE_DEPENDENCIES } from '@iron-e/scabbard/rust';
import { enqueue, readIgnoreFile } from '@iron-e/scabbard';

enqueue('tests' , async client => {
	const excludePromise = Array.fromAsync(readIgnoreFile());
	const project = client.host().directory(".");

	const base = client
		.container()
		.pipeline('install deps')
		.fromWithDeps('rust:1.76.0-alpine', BASE_DEPENDENCIES)
		.withWorkDirectory(project, { exclude: await excludePromise });

	const volume = client.cargoHomeCache();
	const withCargo = base
		.pipeline('mount cargo cache')
		.withCargoHome(volume)
		.pipeline('install cargo-hack')
		.withCargoInstall('cargo-hack@0.6.20');

	const output = await withCargo
		.pipeline('run')
		.withExec(["cargo", "hack", "--feature-powerset", "test"])
		.stdout();

	console.log(output);
});

import.meta.filename.runPipelinesIfMain();
