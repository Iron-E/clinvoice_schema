import { Pipeline } from './util/pipeline.mjs';

// register pipes
export * from './test.mjs';

// run pipeline
Pipeline.main(import.meta.url);
