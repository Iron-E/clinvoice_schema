import { Pipeline } from './util/pipeline.mjs';

Pipeline.register('test: --no-default-features', 'test the project with no features' , async _ => {
});

Pipeline.main(import.meta.url);
