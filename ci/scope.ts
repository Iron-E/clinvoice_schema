import { BASE_DEPENDENCIES } from '@iron-e/scabbard/rust';
import { CacheVolume, Container, Directory } from '@dagger.io/dagger';
import { declare, inject, readIgnoreFile } from '@iron-e/scabbard';

const exclude = await Array.fromAsync(readIgnoreFile());

declare('cargoHome', client => client.cargoHomeCache());
declare('project', client => client.host().directory('.'));

declare('base', ['project'], client =>
	client
		.container()
		.pipeline('install deps')
		.fromWithDeps('rust:1.76.0-alpine', BASE_DEPENDENCIES)
		.withWorkDirectory(inject('project').instance(Directory), { exclude }),
);

declare('withCargo', ['base', 'cargoHome'], _ =>
	inject('base')
		.instance(Container)
		.pipeline('mount cargo cache')
		.withCargoHome(inject('cargoHome').instance(CacheVolume))
		.pipeline('install cargo-hack')
		.withCargoInstall('cargo-hack@0.6.20')
);
