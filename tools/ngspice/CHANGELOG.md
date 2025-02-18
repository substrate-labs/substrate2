# Changelog

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.10.1 to 0.10.2
    * nutlex bumped from 0.4.1 to 0.4.2
    * spice bumped from 0.9.1 to 0.9.2

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.10.2 to 0.10.3
    * spice bumped from 0.9.2 to 0.9.3

## [0.5.1](https://github.com/ucb-substrate/substrate2/compare/ngspice-v0.5.0...ngspice-v0.5.1) (2025-01-24)


### Dependencies

* update dependencies ([0b87032](https://github.com/ucb-substrate/substrate2/commit/0b8703276631fbb19a958453394c981d6b092441))
* update deps, GH actions ([#551](https://github.com/ucb-substrate/substrate2/issues/551)) ([357e82a](https://github.com/ucb-substrate/substrate2/commit/357e82ae0a01317d3ad5afb33b5290d3ac10cd7a))
* The following workspace dependencies were updated
  * dependencies
    * cache bumped from 0.7.0 to 0.7.1
    * scir bumped from 0.9.0 to 0.9.1
    * substrate bumped from 0.10.0 to 0.10.1
    * nutlex bumped from 0.4.0 to 0.4.1
    * spice bumped from 0.9.0 to 0.9.1

## [0.5.0](https://github.com/ucb-substrate/substrate2/compare/ngspice-v0.4.0...ngspice-v0.5.0) (2025-01-23)


### Features

* **ac:** implement Save for Substrate types in Spectre AC sim, fix lints ([#471](https://github.com/ucb-substrate/substrate2/issues/471)) ([9825520](https://github.com/ucb-substrate/substrate2/commit/98255207569cc00bd9ddc35419c2df1e48f1999c))
* **cdl:** add CDL parser ([#419](https://github.com/ucb-substrate/substrate2/issues/419)) ([23f0dab](https://github.com/ucb-substrate/substrate2/commit/23f0dab7b7a94cbe8960371b89d15211bddf51da))
* **docs:** fix user docs and update dev docs ([#480](https://github.com/ucb-substrate/substrate2/issues/480)) ([f727a1e](https://github.com/ucb-substrate/substrate2/commit/f727a1e7bd2a795ace1c51c3d6e02f3673d07a29))
* **docs:** inverter tutorial cleanup and layout/pex sections ([#487](https://github.com/ucb-substrate/substrate2/issues/487)) ([5e509df](https://github.com/ucb-substrate/substrate2/commit/5e509df95a5c145fc69280269d36d860418fb1c0))
* **docs:** update docs for new simulation APIs ([#326](https://github.com/ucb-substrate/substrate2/issues/326)) ([ef133df](https://github.com/ucb-substrate/substrate2/commit/ef133dfac5f352121fe0e561b76541d5af62970e))
* **docs:** versioned documentation between HEAD and release ([#470](https://github.com/ucb-substrate/substrate2/issues/470)) ([968182b](https://github.com/ucb-substrate/substrate2/commit/968182bf8f8d8b4cf923c0fd66f1ca1b32b12b16))
* **isource:** add support for ngspice current sources ([#453](https://github.com/ucb-substrate/substrate2/issues/453)) ([098b8b8](https://github.com/ucb-substrate/substrate2/commit/098b8b8633d6998f5c5298484166ead7ac600c4d))
* **macros:** refactor derive NestedData, start organizing tests ([#477](https://github.com/ucb-substrate/substrate2/issues/477)) ([aca48ef](https://github.com/ucb-substrate/substrate2/commit/aca48ef7a49c959e35ec4614345a55e667ff5146))
* **montecarlo:** add Monte Carlo simulation support to Spectre plugin ([#347](https://github.com/ucb-substrate/substrate2/issues/347)) ([cc9dfe4](https://github.com/ucb-substrate/substrate2/commit/cc9dfe42db5be1a8aaeaf3fb81992a0ad7251ef8))
* **ngspice:** create ngspice simulator ([#274](https://github.com/ucb-substrate/substrate2/issues/274)) ([0205300](https://github.com/ucb-substrate/substrate2/commit/02053006bc26d0b3d9e1d380def89836d7921857))
* **parser:** use nutmeg format for spectre output ([#289](https://github.com/ucb-substrate/substrate2/issues/289)) ([034f58f](https://github.com/ucb-substrate/substrate2/commit/034f58f99c587c61003761971e76c26038de9b3b))
* **pex:** magic-netgen pex mapping, reorganize pex tests ([#467](https://github.com/ucb-substrate/substrate2/issues/467)) ([e32802b](https://github.com/ucb-substrate/substrate2/commit/e32802bfc567f3dea50cc86b11576f7d6863fac2))
* **primitives:** revamp schematic primitives ([#291](https://github.com/ucb-substrate/substrate2/issues/291)) ([e5ba06a](https://github.com/ucb-substrate/substrate2/commit/e5ba06ab10008b72e78397ad70781caa6bc61791))
* **refactor:** rename Has_ and Has_Data ([#282](https://github.com/ucb-substrate/substrate2/issues/282)) ([2018153](https://github.com/ucb-substrate/substrate2/commit/2018153686dd7ef3df0e10874db3c656ca245026))
* **refactor:** significantly refactor IO APIs ([#348](https://github.com/ucb-substrate/substrate2/issues/348)) ([c85d043](https://github.com/ucb-substrate/substrate2/commit/c85d04334a0ba1740f9990b91fb55ab1f2ef77c5))
* **simulation:** improve simulation APIs ([#320](https://github.com/ucb-substrate/substrate2/issues/320)) ([4ed59a1](https://github.com/ucb-substrate/substrate2/commit/4ed59a1283f9546e8336cc96015bd87c55682777))
* **spice:** refactor netlisting and fix voltage source netlist ([#316](https://github.com/ucb-substrate/substrate2/issues/316)) ([7a3df69](https://github.com/ucb-substrate/substrate2/commit/7a3df695cf9b38c837ff86d5a5da2417c4db7aa2))
* **stdcells:** add standard cell support to Sky130 PDK ([#323](https://github.com/ucb-substrate/substrate2/issues/323)) ([0b2048e](https://github.com/ucb-substrate/substrate2/commit/0b2048ed44d89c5de87380cac48a4bbff2b4c20a))
* **tests:** fix compilation and lint errors ([#482](https://github.com/ucb-substrate/substrate2/issues/482)) ([b55d04e](https://github.com/ucb-substrate/substrate2/commit/b55d04ecd2472f9f72b926ba5286f0d928bc2691))
* **tests:** reorganize tests and documentation ([#464](https://github.com/ucb-substrate/substrate2/issues/464)) ([928b9b7](https://github.com/ucb-substrate/substrate2/commit/928b9b7c45dc334ca11d86e4564edc58bf6db6f2))


### Bug Fixes

* **deps:** remove opacity from substrate and deps ([#288](https://github.com/ucb-substrate/substrate2/issues/288)) ([a8c97b3](https://github.com/ucb-substrate/substrate2/commit/a8c97b30b4d075343903fa580437e9a099a745a2))
* **deps:** update rust crate rust_decimal to 1.32 ([#296](https://github.com/ucb-substrate/substrate2/issues/296)) ([a2fe877](https://github.com/ucb-substrate/substrate2/commit/a2fe877d03d3f907f348d7711a2132194ae91034))
* **deps:** update rust crate rust_decimal_macros to 1.32 ([#297](https://github.com/ucb-substrate/substrate2/issues/297)) ([5474cc8](https://github.com/ucb-substrate/substrate2/commit/5474cc8778b81c30b34fc7d146eec6e5e2532a26))
* ngspice tests ([#310](https://github.com/ucb-substrate/substrate2/issues/310)) ([62e16bd](https://github.com/ucb-substrate/substrate2/commit/62e16bdf296a6150066369f6465f49d299d86842))
* **simulation:** add missing SPICE functionality and update Sky 130 PDK ([#336](https://github.com/ucb-substrate/substrate2/issues/336)) ([f802be5](https://github.com/ucb-substrate/substrate2/commit/f802be5bf0361c38b415d976dbb0f2c984a2e304))
* **simulation:** standardize ngspice and spectre transient data formats ([#327](https://github.com/ucb-substrate/substrate2/issues/327)) ([0aa42d6](https://github.com/ucb-substrate/substrate2/commit/0aa42d6000d28a8aecb655e06330f4545e155b9b))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * cache bumped from 0.6.0 to 0.7.0
    * scir bumped from 0.8.0 to 0.9.0
    * substrate bumped from 0.9.0 to 0.10.0
    * nutlex bumped from 0.3.0 to 0.4.0
    * spice bumped from 0.8.0 to 0.9.0

## [0.4.0](https://github.com/ucb-substrate/substrate2/compare/ngspice-v0.3.1...ngspice-v0.4.0) (2025-01-22)


### Features

* **ac:** implement Save for Substrate types in Spectre AC sim, fix lints ([#471](https://github.com/ucb-substrate/substrate2/issues/471)) ([9825520](https://github.com/ucb-substrate/substrate2/commit/98255207569cc00bd9ddc35419c2df1e48f1999c))
* **cdl:** add CDL parser ([#419](https://github.com/ucb-substrate/substrate2/issues/419)) ([23f0dab](https://github.com/ucb-substrate/substrate2/commit/23f0dab7b7a94cbe8960371b89d15211bddf51da))
* **docs:** fix user docs and update dev docs ([#480](https://github.com/ucb-substrate/substrate2/issues/480)) ([f727a1e](https://github.com/ucb-substrate/substrate2/commit/f727a1e7bd2a795ace1c51c3d6e02f3673d07a29))
* **docs:** inverter tutorial cleanup and layout/pex sections ([#487](https://github.com/ucb-substrate/substrate2/issues/487)) ([5e509df](https://github.com/ucb-substrate/substrate2/commit/5e509df95a5c145fc69280269d36d860418fb1c0))
* **docs:** update docs for new simulation APIs ([#326](https://github.com/ucb-substrate/substrate2/issues/326)) ([ef133df](https://github.com/ucb-substrate/substrate2/commit/ef133dfac5f352121fe0e561b76541d5af62970e))
* **docs:** versioned documentation between HEAD and release ([#470](https://github.com/ucb-substrate/substrate2/issues/470)) ([968182b](https://github.com/ucb-substrate/substrate2/commit/968182bf8f8d8b4cf923c0fd66f1ca1b32b12b16))
* **isource:** add support for ngspice current sources ([#453](https://github.com/ucb-substrate/substrate2/issues/453)) ([098b8b8](https://github.com/ucb-substrate/substrate2/commit/098b8b8633d6998f5c5298484166ead7ac600c4d))
* **macros:** refactor derive NestedData, start organizing tests ([#477](https://github.com/ucb-substrate/substrate2/issues/477)) ([aca48ef](https://github.com/ucb-substrate/substrate2/commit/aca48ef7a49c959e35ec4614345a55e667ff5146))
* **montecarlo:** add Monte Carlo simulation support to Spectre plugin ([#347](https://github.com/ucb-substrate/substrate2/issues/347)) ([cc9dfe4](https://github.com/ucb-substrate/substrate2/commit/cc9dfe42db5be1a8aaeaf3fb81992a0ad7251ef8))
* **pex:** magic-netgen pex mapping, reorganize pex tests ([#467](https://github.com/ucb-substrate/substrate2/issues/467)) ([e32802b](https://github.com/ucb-substrate/substrate2/commit/e32802bfc567f3dea50cc86b11576f7d6863fac2))
* **refactor:** significantly refactor IO APIs ([#348](https://github.com/ucb-substrate/substrate2/issues/348)) ([c85d043](https://github.com/ucb-substrate/substrate2/commit/c85d04334a0ba1740f9990b91fb55ab1f2ef77c5))
* **tests:** fix compilation and lint errors ([#482](https://github.com/ucb-substrate/substrate2/issues/482)) ([b55d04e](https://github.com/ucb-substrate/substrate2/commit/b55d04ecd2472f9f72b926ba5286f0d928bc2691))
* **tests:** reorganize tests and documentation ([#464](https://github.com/ucb-substrate/substrate2/issues/464)) ([928b9b7](https://github.com/ucb-substrate/substrate2/commit/928b9b7c45dc334ca11d86e4564edc58bf6db6f2))


### Bug Fixes

* **simulation:** add missing SPICE functionality and update Sky 130 PDK ([#336](https://github.com/ucb-substrate/substrate2/issues/336)) ([f802be5](https://github.com/ucb-substrate/substrate2/commit/f802be5bf0361c38b415d976dbb0f2c984a2e304))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * cache bumped from 0.5.0 to 0.6.0
    * scir bumped from 0.7.0 to 0.8.0
    * substrate bumped from 0.8.1 to 0.9.0
    * nutlex bumped from 0.2.0 to 0.3.0
    * spice bumped from 0.7.1 to 0.8.0

## [0.3.1](https://github.com/ucb-substrate/substrate2/compare/ngspice-v0.3.0...ngspice-v0.3.1) (2023-11-26)


### Bug Fixes

* **simulation:** standardize ngspice and spectre transient data formats ([#327](https://github.com/ucb-substrate/substrate2/issues/327)) ([0aa42d6](https://github.com/ucb-substrate/substrate2/commit/0aa42d6000d28a8aecb655e06330f4545e155b9b))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.8.0 to 0.8.1
    * spice bumped from 0.7.0 to 0.7.1

## [0.3.0](https://github.com/ucb-substrate/substrate2/compare/ngspice-v0.2.0...ngspice-v0.3.0) (2023-11-25)


### Features

* **simulation:** improve simulation APIs ([#320](https://github.com/ucb-substrate/substrate2/issues/320)) ([4ed59a1](https://github.com/ucb-substrate/substrate2/commit/4ed59a1283f9546e8336cc96015bd87c55682777))
* **stdcells:** add standard cell support to Sky130 PDK ([#323](https://github.com/ucb-substrate/substrate2/issues/323)) ([0b2048e](https://github.com/ucb-substrate/substrate2/commit/0b2048ed44d89c5de87380cac48a4bbff2b4c20a))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * cache bumped from 0.4.0 to 0.5.0
    * substrate bumped from 0.7.1 to 0.8.0
    * spice bumped from 0.6.0 to 0.7.0

## [0.2.0](https://github.com/ucb-substrate/substrate2/compare/ngspice-v0.1.0...ngspice-v0.2.0) (2023-11-04)


### Features

* **spice:** refactor netlisting and fix voltage source netlist ([#316](https://github.com/ucb-substrate/substrate2/issues/316)) ([7a3df69](https://github.com/ucb-substrate/substrate2/commit/7a3df695cf9b38c837ff86d5a5da2417c4db7aa2))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * scir bumped from 0.6.0 to 0.7.0
    * substrate bumped from 0.7.0 to 0.7.1
    * spice bumped from 0.5.0 to 0.6.0

## 0.1.0 (2023-11-02)


### Features

* **ngspice:** create ngspice simulator ([#274](https://github.com/ucb-substrate/substrate2/issues/274)) ([0205300](https://github.com/ucb-substrate/substrate2/commit/02053006bc26d0b3d9e1d380def89836d7921857))
* **parser:** use nutmeg format for spectre output ([#289](https://github.com/ucb-substrate/substrate2/issues/289)) ([034f58f](https://github.com/ucb-substrate/substrate2/commit/034f58f99c587c61003761971e76c26038de9b3b))
* **primitives:** revamp schematic primitives ([#291](https://github.com/ucb-substrate/substrate2/issues/291)) ([e5ba06a](https://github.com/ucb-substrate/substrate2/commit/e5ba06ab10008b72e78397ad70781caa6bc61791))
* **refactor:** rename Has_ and Has_Data ([#282](https://github.com/ucb-substrate/substrate2/issues/282)) ([2018153](https://github.com/ucb-substrate/substrate2/commit/2018153686dd7ef3df0e10874db3c656ca245026))


### Bug Fixes

* **deps:** remove opacity from substrate and deps ([#288](https://github.com/ucb-substrate/substrate2/issues/288)) ([a8c97b3](https://github.com/ucb-substrate/substrate2/commit/a8c97b30b4d075343903fa580437e9a099a745a2))
* **deps:** update rust crate rust_decimal to 1.32 ([#296](https://github.com/ucb-substrate/substrate2/issues/296)) ([a2fe877](https://github.com/ucb-substrate/substrate2/commit/a2fe877d03d3f907f348d7711a2132194ae91034))
* **deps:** update rust crate rust_decimal_macros to 1.32 ([#297](https://github.com/ucb-substrate/substrate2/issues/297)) ([5474cc8](https://github.com/ucb-substrate/substrate2/commit/5474cc8778b81c30b34fc7d146eec6e5e2532a26))
* ngspice tests ([#310](https://github.com/ucb-substrate/substrate2/issues/310)) ([62e16bd](https://github.com/ucb-substrate/substrate2/commit/62e16bdf296a6150066369f6465f49d299d86842))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * cache bumped from 0.3.1 to 0.4.0
    * scir bumped from 0.5.0 to 0.6.0
    * substrate bumped from 0.6.1 to 0.7.0
    * nutlex bumped from 0.1.0 to 0.2.0
    * spice bumped from 0.4.0 to 0.5.0
