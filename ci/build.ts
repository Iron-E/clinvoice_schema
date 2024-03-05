import './scope';
import { Container } from '@dagger.io/dagger';
import { enqueue } from '@iron-e/scabbard';
import { WITH_CARGO_HACK } from '@iron-e/scabbard/rust/scope/client';

enqueue('build', async (_, inject) => {
	const withCargo = (await inject(WITH_CARGO_HACK)).instance(Container);
	const output = await withCargo
		.pipeline('run')
		.withExec(['cargo', 'hack', '--feature-powerset', 'build'])
		.stdout();

	console.log(output);
});

await import.meta.filename.runPipelinesIfMain();
