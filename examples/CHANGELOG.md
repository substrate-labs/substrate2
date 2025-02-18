# Changelog

## [0.2.1](https://github.com/ucb-substrate/substrate2/compare/examples-v0.2.0...examples-v0.2.1) (2025-02-18)


### Bug Fixes

* **examples:** fix release example compilation ([#576](https://github.com/ucb-substrate/substrate2/issues/576)) ([1e3d89f](https://github.com/ucb-substrate/substrate2/commit/1e3d89f3dd8c152640ec0408fefc9e32e4d7ddba))
* **justfile:** remove extra targets from justfile, fix formatting ([#588](https://github.com/ucb-substrate/substrate2/issues/588)) ([efc3591](https://github.com/ucb-substrate/substrate2/commit/efc35916dcfc4fe04ef59cffe9155f5069916d07))

## [0.2.0](https://github.com/ucb-substrate/substrate2/compare/examples-v0.1.0...examples-v0.2.0) (2025-01-23)


### Features

* **docs:** inverter design tutorial updates ([#154](https://github.com/ucb-substrate/substrate2/issues/154)) ([276cfd1](https://github.com/ucb-substrate/substrate2/commit/276cfd1733bd26e4b5b68a0667f610012c895261))
* **docs:** update docs for new simulation APIs ([#326](https://github.com/ucb-substrate/substrate2/issues/326)) ([ef133df](https://github.com/ucb-substrate/substrate2/commit/ef133dfac5f352121fe0e561b76541d5af62970e))
* **docs:** update tutorials and revamp documentation website ([#315](https://github.com/ucb-substrate/substrate2/issues/315)) ([49bdf7f](https://github.com/ucb-substrate/substrate2/commit/49bdf7ff61e2fdbf19022697d518ad7fbafb465f))
* **docs:** versioned documentation between HEAD and release ([#470](https://github.com/ucb-substrate/substrate2/issues/470)) ([968182b](https://github.com/ucb-substrate/substrate2/commit/968182bf8f8d8b4cf923c0fd66f1ca1b32b12b16))
* **examples:** separate workspace for examples ([#160](https://github.com/ucb-substrate/substrate2/issues/160)) ([5121ee9](https://github.com/ucb-substrate/substrate2/commit/5121ee9402ed5810923e0f37dabe10277588b15f))
* **macros:** refactor derive NestedData, start organizing tests ([#477](https://github.com/ucb-substrate/substrate2/issues/477)) ([aca48ef](https://github.com/ucb-substrate/substrate2/commit/aca48ef7a49c959e35ec4614345a55e667ff5146))
* **simulation:** improve simulation APIs ([#320](https://github.com/ucb-substrate/substrate2/issues/320)) ([4ed59a1](https://github.com/ucb-substrate/substrate2/commit/4ed59a1283f9546e8336cc96015bd87c55682777))
* **spice:** refactor netlisting and fix voltage source netlist ([#316](https://github.com/ucb-substrate/substrate2/issues/316)) ([7a3df69](https://github.com/ucb-substrate/substrate2/commit/7a3df695cf9b38c837ff86d5a5da2417c4db7aa2))
* **type-dispatch:** add helper crate for dispatching types ([#225](https://github.com/ucb-substrate/substrate2/issues/225)) ([13ee1aa](https://github.com/ucb-substrate/substrate2/commit/13ee1aa1b287ed0c147549003c0af815b849577b))


### Bug Fixes

* **ci:** fix ci for Substrate v2.1 ([#490](https://github.com/ucb-substrate/substrate2/issues/490)) ([cc09d71](https://github.com/ucb-substrate/substrate2/commit/cc09d7199b41fb2986d1d733aa3678db49464f70))
* **ci:** test release examples after crates are published ([#493](https://github.com/ucb-substrate/substrate2/issues/493)) ([686a972](https://github.com/ucb-substrate/substrate2/commit/686a972a9e6ca7833fd5ff548e3b3f0c5469952c))
* **docs:** fix snippet publishing ([#512](https://github.com/ucb-substrate/substrate2/issues/512)) ([456f8bf](https://github.com/ucb-substrate/substrate2/commit/456f8bfe659d4fa2a05f6d56394a6171c4fd34dd))
* **docs:** move use statement into code snippet ([#367](https://github.com/ucb-substrate/substrate2/issues/367)) ([c72bbc8](https://github.com/ucb-substrate/substrate2/commit/c72bbc8a89b0e617c7fae880313385a6383384bc))
* **docs:** remove Cargo.tomls in CI to allow publishing of API docs ([#515](https://github.com/ucb-substrate/substrate2/issues/515)) ([2d14f50](https://github.com/ucb-substrate/substrate2/commit/2d14f50add396a1d775428b273df7d8d022aea05))
* **docs:** update docs to latest release (substrate v0.3.0) ([#204](https://github.com/ucb-substrate/substrate2/issues/204)) ([df45b6c](https://github.com/ucb-substrate/substrate2/commit/df45b6c56c7eb8d01bd2ec104c5d2593bc8f80cc))
* **docs:** use numbered deps in inverter tutorial ([#159](https://github.com/ucb-substrate/substrate2/issues/159)) ([f2b8e08](https://github.com/ucb-substrate/substrate2/commit/f2b8e0846e7080d38748766acf3624415b4d0a29))
* **release:** use correct gds for `examples/release/colbuf/test_col_buffer_array.gds` ([#520](https://github.com/ucb-substrate/substrate2/issues/520)) ([63ea1ea](https://github.com/ucb-substrate/substrate2/commit/63ea1ea60426ac788218c5830cca92601737d5e4))
