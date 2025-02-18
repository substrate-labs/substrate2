# Changelog

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.8.0 to 0.8.1

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.10.2 to 0.10.3

## [0.9.2](https://github.com/ucb-substrate/substrate2/compare/spice-v0.9.1...spice-v0.9.2) (2025-02-02)


### Features

* **spicemerge:** add command line tool for aggregating SPICE netlists ([#568](https://github.com/ucb-substrate/substrate2/issues/568)) ([ffd6975](https://github.com/ucb-substrate/substrate2/commit/ffd6975e83bcaaea898ff18b94e48b8a75661096))


### Bug Fixes

* **deps:** update nom to v8 ([#559](https://github.com/ucb-substrate/substrate2/issues/559)) ([79143c7](https://github.com/ucb-substrate/substrate2/commit/79143c749367067b031bf23546b592b4e2f282a0))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.10.1 to 0.10.2

## [0.9.1](https://github.com/ucb-substrate/substrate2/compare/spice-v0.9.0...spice-v0.9.1) (2025-01-24)


### Dependencies

* update dependencies ([0b87032](https://github.com/ucb-substrate/substrate2/commit/0b8703276631fbb19a958453394c981d6b092441))
* update dependencies ([#538](https://github.com/ucb-substrate/substrate2/issues/538)) ([19438d6](https://github.com/ucb-substrate/substrate2/commit/19438d65ac7078a2a971b4147420364ca0717763))
* update deps, GH actions ([#551](https://github.com/ucb-substrate/substrate2/issues/551)) ([357e82a](https://github.com/ucb-substrate/substrate2/commit/357e82ae0a01317d3ad5afb33b5290d3ac10cd7a))
* The following workspace dependencies were updated
  * dependencies
    * scir bumped from 0.9.0 to 0.9.1
    * substrate bumped from 0.10.0 to 0.10.1
    * enumify bumped from 0.2.0 to 0.2.1

## [0.9.0](https://github.com/ucb-substrate/substrate2/compare/spice-v0.8.0...spice-v0.9.0) (2025-01-23)


### Features

* **ac:** implement Save for Substrate types in Spectre AC sim, fix lints ([#471](https://github.com/ucb-substrate/substrate2/issues/471)) ([9825520](https://github.com/ucb-substrate/substrate2/commit/98255207569cc00bd9ddc35419c2df1e48f1999c))
* **bjt:** add support for BJTs ([#432](https://github.com/ucb-substrate/substrate2/issues/432)) ([e0c4516](https://github.com/ucb-substrate/substrate2/commit/e0c45162da072ea21567b8e23d11dce36b4cff17))
* **cadence:** add support for Pegasus and Quantus ([#462](https://github.com/ucb-substrate/substrate2/issues/462)) ([953e4cb](https://github.com/ucb-substrate/substrate2/commit/953e4cb761c510668f65f4825f1be3914db45e3c))
* **cdl2spice:** add CDL to SPICE conversion command line tool ([#420](https://github.com/ucb-substrate/substrate2/issues/420)) ([1edb23a](https://github.com/ucb-substrate/substrate2/commit/1edb23a7bbd45d96bbb1c11418eb0d0843b7138b))
* **cdl:** add CDL parser ([#419](https://github.com/ucb-substrate/substrate2/issues/419)) ([23f0dab](https://github.com/ucb-substrate/substrate2/commit/23f0dab7b7a94cbe8960371b89d15211bddf51da))
* **conv:** better error messages in schema conversions ([#440](https://github.com/ucb-substrate/substrate2/issues/440)) ([bad9503](https://github.com/ucb-substrate/substrate2/commit/bad9503b8a3b98d8e0bc19779ed45e7628164f41))
* **conversion:** convert parsed SPICE to SCIR ([#178](https://github.com/ucb-substrate/substrate2/issues/178)) ([9cb7bc3](https://github.com/ucb-substrate/substrate2/commit/9cb7bc3ba549ae12e7a59465241c848800c39363))
* **docs:** fix user docs and update dev docs ([#480](https://github.com/ucb-substrate/substrate2/issues/480)) ([f727a1e](https://github.com/ucb-substrate/substrate2/commit/f727a1e7bd2a795ace1c51c3d6e02f3673d07a29))
* **dspf:** propagate nested nodes from DSPF instances ([#407](https://github.com/ucb-substrate/substrate2/issues/407)) ([8455bd2](https://github.com/ucb-substrate/substrate2/commit/8455bd2a523bb872dc1ce3fc0e89a185108dca3c))
* **netlister:** reduce duplicate code between spectre and SPICE netlisters ([#261](https://github.com/ucb-substrate/substrate2/issues/261)) ([5ba3623](https://github.com/ucb-substrate/substrate2/commit/5ba36230e653e4dc77819c5c50b527311768cd83))
* **netlists:** consistent Spectre/Spice netlist API ([#349](https://github.com/ucb-substrate/substrate2/issues/349)) ([2f9fabf](https://github.com/ucb-substrate/substrate2/commit/2f9fabf336fa1048d759e78834979ef892fc0bcf))
* **netlists:** use consistent ordering via indexmap ([#266](https://github.com/ucb-substrate/substrate2/issues/266)) ([f275c19](https://github.com/ucb-substrate/substrate2/commit/f275c19396ed4f7d255836822ff72b808f89cde7)), closes [#265](https://github.com/ucb-substrate/substrate2/issues/265)
* **ngspice:** create ngspice simulator ([#274](https://github.com/ucb-substrate/substrate2/issues/274)) ([0205300](https://github.com/ucb-substrate/substrate2/commit/02053006bc26d0b3d9e1d380def89836d7921857))
* **organization:** move `spice` from netlist/ to libs/ ([#174](https://github.com/ucb-substrate/substrate2/issues/174)) ([bd31a44](https://github.com/ucb-substrate/substrate2/commit/bd31a4481aef357daeb2c217dd7f403f6f882f78))
* **parser:** add support for 2-terminal diodes ([b74afa1](https://github.com/ucb-substrate/substrate2/commit/b74afa1118cbb37f6865eb8d472218658ee6f1b4))
* **parser:** be able to parse PEX netlists ([#363](https://github.com/ucb-substrate/substrate2/issues/363)) ([2e2f8ac](https://github.com/ucb-substrate/substrate2/commit/2e2f8ac229434fc0c03fce9e9f3ca1d0915b3469))
* **parser:** parse negative numbers and exponents ([#364](https://github.com/ucb-substrate/substrate2/issues/364)) ([53c01f6](https://github.com/ucb-substrate/substrate2/commit/53c01f60177d3d50e0302e24873be3e29f55aaa3))
* **pex:** magic-netgen pex mapping, reorganize pex tests ([#467](https://github.com/ucb-substrate/substrate2/issues/467)) ([e32802b](https://github.com/ucb-substrate/substrate2/commit/e32802bfc567f3dea50cc86b11576f7d6863fac2))
* **primitives:** revamp schematic primitives ([#291](https://github.com/ucb-substrate/substrate2/issues/291)) ([e5ba06a](https://github.com/ucb-substrate/substrate2/commit/e5ba06ab10008b72e78397ad70781caa6bc61791))
* **primitives:** support parameters in SCIR primitive devices ([#232](https://github.com/ucb-substrate/substrate2/issues/232)) ([a8f5b45](https://github.com/ucb-substrate/substrate2/commit/a8f5b45a00b77d050f6a812c469e19da3305e064))
* **proc-macros:** codegen for schematic hard macros ([#191](https://github.com/ucb-substrate/substrate2/issues/191)) ([50240b1](https://github.com/ucb-substrate/substrate2/commit/50240b167876873c4133315d35298b44e8eeac51))
* **refactor:** significantly refactor IO APIs ([#348](https://github.com/ucb-substrate/substrate2/issues/348)) ([c85d043](https://github.com/ucb-substrate/substrate2/commit/c85d04334a0ba1740f9990b91fb55ab1f2ef77c5))
* **repo:** reorganize repo ([#207](https://github.com/ucb-substrate/substrate2/issues/207)) ([54a6b43](https://github.com/ucb-substrate/substrate2/commit/54a6b43079d283a29bc0aa9e18dc6230b56fa385))
* **schematic:** associated type schema and bundle primitives ([#455](https://github.com/ucb-substrate/substrate2/issues/455)) ([f5fde78](https://github.com/ucb-substrate/substrate2/commit/f5fde78824ce9ed0be494ef68d71620181bf6b48))
* **schematics:** blackboxes can reference nodes ([#208](https://github.com/ucb-substrate/substrate2/issues/208)) ([d998b4a](https://github.com/ucb-substrate/substrate2/commit/d998b4a133d47d0123768dfb3c27f8ee32ed9db9))
* **scir-instances:** allow Substrate users to instantiate raw SCIR instances ([#184](https://github.com/ucb-substrate/substrate2/issues/184)) ([8fd5192](https://github.com/ucb-substrate/substrate2/commit/8fd5192fd2017ab04e9e3220612d0a132702bb2e))
* **scir:** expose port directions, update docs ([#426](https://github.com/ucb-substrate/substrate2/issues/426)) ([fd883b7](https://github.com/ucb-substrate/substrate2/commit/fd883b7ca803f7b45d4736a7b4b460e602b84704))
* **simulation:** automatically generate saved data ([#457](https://github.com/ucb-substrate/substrate2/issues/457)) ([2c936d0](https://github.com/ucb-substrate/substrate2/commit/2c936d00e927b99b624f29e6450826e90f68f9bf))
* **simulation:** improve simulation APIs ([#320](https://github.com/ucb-substrate/substrate2/issues/320)) ([4ed59a1](https://github.com/ucb-substrate/substrate2/commit/4ed59a1283f9546e8336cc96015bd87c55682777))
* **slices:** use `SliceOne` instead of `Slice` where possible ([#253](https://github.com/ucb-substrate/substrate2/issues/253)) ([8eba8ed](https://github.com/ucb-substrate/substrate2/commit/8eba8ed5aad0aa4911ae31f4521d297487256087))
* **spice-parser:** spice parser follows include directives ([#229](https://github.com/ucb-substrate/substrate2/issues/229)) ([5259acf](https://github.com/ucb-substrate/substrate2/commit/5259acfa703c3879d44d324279293278c46f1ff5))
* **spice-to-scir:** do not convert blackboxed subcircuits ([#179](https://github.com/ucb-substrate/substrate2/issues/179)) ([c501313](https://github.com/ucb-substrate/substrate2/commit/c501313334279b636f1d8b581357dd805177f1ca))
* **spice:** add `RawInstanceWithCell` primitive ([#384](https://github.com/ucb-substrate/substrate2/issues/384)) ([847d76b](https://github.com/ucb-substrate/substrate2/commit/847d76b2a92265faf7b8bbd079f126d1b1ba4802))
* **spice:** refactor netlisting and fix voltage source netlist ([#316](https://github.com/ucb-substrate/substrate2/issues/316)) ([7a3df69](https://github.com/ucb-substrate/substrate2/commit/7a3df695cf9b38c837ff86d5a5da2417c4db7aa2))
* **stdcells:** add standard cell support to Sky130 PDK ([#323](https://github.com/ucb-substrate/substrate2/issues/323)) ([0b2048e](https://github.com/ucb-substrate/substrate2/commit/0b2048ed44d89c5de87380cac48a4bbff2b4c20a))
* **terminals:** add support for terminal paths ([#236](https://github.com/ucb-substrate/substrate2/issues/236)) ([3fba7f6](https://github.com/ucb-substrate/substrate2/commit/3fba7f6227bbf2efcaf79d849c79175e44d783a4))
* **tests:** fix compilation and lint errors ([#482](https://github.com/ucb-substrate/substrate2/issues/482)) ([b55d04e](https://github.com/ucb-substrate/substrate2/commit/b55d04ecd2472f9f72b926ba5286f0d928bc2691))
* **tests:** reorganize tests and documentation ([#464](https://github.com/ucb-substrate/substrate2/issues/464)) ([928b9b7](https://github.com/ucb-substrate/substrate2/commit/928b9b7c45dc334ca11d86e4564edc58bf6db6f2))
* **validation:** create type for unvalidated SCIR library ([#252](https://github.com/ucb-substrate/substrate2/issues/252)) ([1550a22](https://github.com/ucb-substrate/substrate2/commit/1550a22b9a1c9f7cd9717feaa45d00487cc8848e))
* **validation:** SCIR driver analysis and validation ([#239](https://github.com/ucb-substrate/substrate2/issues/239)) ([5a91448](https://github.com/ucb-substrate/substrate2/commit/5a914489294bed06be1bd34aaa1036e4357d9a52))
* **views:** view API for improved codegen ([#463](https://github.com/ucb-substrate/substrate2/issues/463)) ([b75328c](https://github.com/ucb-substrate/substrate2/commit/b75328c9a4840ed9200a9035e28e27ac9265770f))


### Bug Fixes

* **cdl:** CDL parser ignores slashes ([#423](https://github.com/ucb-substrate/substrate2/issues/423)) ([e2b259f](https://github.com/ucb-substrate/substrate2/commit/e2b259f040913df5d73a81f778be43b716a4bbfc))
* **deps:** add missing `registry=substrate` for in-tree dependencies ([#517](https://github.com/ucb-substrate/substrate2/issues/517)) ([505d95c](https://github.com/ucb-substrate/substrate2/commit/505d95c17c5997166c1987cbc30e344fdd4c78fb))
* **deps:** remove opacity from spice library ([#287](https://github.com/ucb-substrate/substrate2/issues/287)) ([a45b728](https://github.com/ucb-substrate/substrate2/commit/a45b7288e240a9955d91acb437fa251fccb66b75))
* **parser:** fix bug in SPICE exponent parser ([#366](https://github.com/ucb-substrate/substrate2/issues/366)) ([4ced97a](https://github.com/ucb-substrate/substrate2/commit/4ced97a660f166837ec6f1468bc5f363a7b1a3ba))
* **scir:** add additional functionality for SCIR and SPICE libraries ([#337](https://github.com/ucb-substrate/substrate2/issues/337)) ([e49f075](https://github.com/ucb-substrate/substrate2/commit/e49f07529273c38cc8ec9ae1a5020ae48fb2a202))
* **simulation:** add missing SPICE functionality and update Sky 130 PDK ([#336](https://github.com/ucb-substrate/substrate2/issues/336)) ([f802be5](https://github.com/ucb-substrate/substrate2/commit/f802be5bf0361c38b415d976dbb0f2c984a2e304))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * scir bumped from 0.8.0 to 0.9.0
    * substrate bumped from 0.9.0 to 0.10.0
    * enumify bumped from 0.1.1 to 0.2.0

## [0.8.0](https://github.com/ucb-substrate/substrate2/compare/spice-v0.7.1...spice-v0.8.0) (2025-01-22)


### Features

* **ac:** implement Save for Substrate types in Spectre AC sim, fix lints ([#471](https://github.com/ucb-substrate/substrate2/issues/471)) ([9825520](https://github.com/ucb-substrate/substrate2/commit/98255207569cc00bd9ddc35419c2df1e48f1999c))
* **bjt:** add support for BJTs ([#432](https://github.com/ucb-substrate/substrate2/issues/432)) ([e0c4516](https://github.com/ucb-substrate/substrate2/commit/e0c45162da072ea21567b8e23d11dce36b4cff17))
* **cadence:** add support for Pegasus and Quantus ([#462](https://github.com/ucb-substrate/substrate2/issues/462)) ([953e4cb](https://github.com/ucb-substrate/substrate2/commit/953e4cb761c510668f65f4825f1be3914db45e3c))
* **cdl2spice:** add CDL to SPICE conversion command line tool ([#420](https://github.com/ucb-substrate/substrate2/issues/420)) ([1edb23a](https://github.com/ucb-substrate/substrate2/commit/1edb23a7bbd45d96bbb1c11418eb0d0843b7138b))
* **cdl:** add CDL parser ([#419](https://github.com/ucb-substrate/substrate2/issues/419)) ([23f0dab](https://github.com/ucb-substrate/substrate2/commit/23f0dab7b7a94cbe8960371b89d15211bddf51da))
* **conv:** better error messages in schema conversions ([#440](https://github.com/ucb-substrate/substrate2/issues/440)) ([bad9503](https://github.com/ucb-substrate/substrate2/commit/bad9503b8a3b98d8e0bc19779ed45e7628164f41))
* **docs:** fix user docs and update dev docs ([#480](https://github.com/ucb-substrate/substrate2/issues/480)) ([f727a1e](https://github.com/ucb-substrate/substrate2/commit/f727a1e7bd2a795ace1c51c3d6e02f3673d07a29))
* **dspf:** propagate nested nodes from DSPF instances ([#407](https://github.com/ucb-substrate/substrate2/issues/407)) ([8455bd2](https://github.com/ucb-substrate/substrate2/commit/8455bd2a523bb872dc1ce3fc0e89a185108dca3c))
* **netlists:** consistent Spectre/Spice netlist API ([#349](https://github.com/ucb-substrate/substrate2/issues/349)) ([2f9fabf](https://github.com/ucb-substrate/substrate2/commit/2f9fabf336fa1048d759e78834979ef892fc0bcf))
* **parser:** add support for 2-terminal diodes ([b74afa1](https://github.com/ucb-substrate/substrate2/commit/b74afa1118cbb37f6865eb8d472218658ee6f1b4))
* **parser:** be able to parse PEX netlists ([#363](https://github.com/ucb-substrate/substrate2/issues/363)) ([2e2f8ac](https://github.com/ucb-substrate/substrate2/commit/2e2f8ac229434fc0c03fce9e9f3ca1d0915b3469))
* **parser:** parse negative numbers and exponents ([#364](https://github.com/ucb-substrate/substrate2/issues/364)) ([53c01f6](https://github.com/ucb-substrate/substrate2/commit/53c01f60177d3d50e0302e24873be3e29f55aaa3))
* **pex:** magic-netgen pex mapping, reorganize pex tests ([#467](https://github.com/ucb-substrate/substrate2/issues/467)) ([e32802b](https://github.com/ucb-substrate/substrate2/commit/e32802bfc567f3dea50cc86b11576f7d6863fac2))
* **refactor:** significantly refactor IO APIs ([#348](https://github.com/ucb-substrate/substrate2/issues/348)) ([c85d043](https://github.com/ucb-substrate/substrate2/commit/c85d04334a0ba1740f9990b91fb55ab1f2ef77c5))
* **schematic:** associated type schema and bundle primitives ([#455](https://github.com/ucb-substrate/substrate2/issues/455)) ([f5fde78](https://github.com/ucb-substrate/substrate2/commit/f5fde78824ce9ed0be494ef68d71620181bf6b48))
* **scir:** expose port directions, update docs ([#426](https://github.com/ucb-substrate/substrate2/issues/426)) ([fd883b7](https://github.com/ucb-substrate/substrate2/commit/fd883b7ca803f7b45d4736a7b4b460e602b84704))
* **simulation:** automatically generate saved data ([#457](https://github.com/ucb-substrate/substrate2/issues/457)) ([2c936d0](https://github.com/ucb-substrate/substrate2/commit/2c936d00e927b99b624f29e6450826e90f68f9bf))
* **spice:** add `RawInstanceWithCell` primitive ([#384](https://github.com/ucb-substrate/substrate2/issues/384)) ([847d76b](https://github.com/ucb-substrate/substrate2/commit/847d76b2a92265faf7b8bbd079f126d1b1ba4802))
* **tests:** fix compilation and lint errors ([#482](https://github.com/ucb-substrate/substrate2/issues/482)) ([b55d04e](https://github.com/ucb-substrate/substrate2/commit/b55d04ecd2472f9f72b926ba5286f0d928bc2691))
* **tests:** reorganize tests and documentation ([#464](https://github.com/ucb-substrate/substrate2/issues/464)) ([928b9b7](https://github.com/ucb-substrate/substrate2/commit/928b9b7c45dc334ca11d86e4564edc58bf6db6f2))
* **views:** view API for improved codegen ([#463](https://github.com/ucb-substrate/substrate2/issues/463)) ([b75328c](https://github.com/ucb-substrate/substrate2/commit/b75328c9a4840ed9200a9035e28e27ac9265770f))


### Bug Fixes

* **cdl:** CDL parser ignores slashes ([#423](https://github.com/ucb-substrate/substrate2/issues/423)) ([e2b259f](https://github.com/ucb-substrate/substrate2/commit/e2b259f040913df5d73a81f778be43b716a4bbfc))
* **parser:** fix bug in SPICE exponent parser ([#366](https://github.com/ucb-substrate/substrate2/issues/366)) ([4ced97a](https://github.com/ucb-substrate/substrate2/commit/4ced97a660f166837ec6f1468bc5f363a7b1a3ba))
* **scir:** add additional functionality for SCIR and SPICE libraries ([#337](https://github.com/ucb-substrate/substrate2/issues/337)) ([e49f075](https://github.com/ucb-substrate/substrate2/commit/e49f07529273c38cc8ec9ae1a5020ae48fb2a202))
* **simulation:** add missing SPICE functionality and update Sky 130 PDK ([#336](https://github.com/ucb-substrate/substrate2/issues/336)) ([f802be5](https://github.com/ucb-substrate/substrate2/commit/f802be5bf0361c38b415d976dbb0f2c984a2e304))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * scir bumped from 0.7.0 to 0.8.0
    * substrate bumped from 0.8.1 to 0.9.0
    * enumify bumped from 0.1.0 to 0.1.1

## [0.7.0](https://github.com/ucb-substrate/substrate2/compare/spice-v0.6.0...spice-v0.7.0) (2023-11-25)


### Features

* **simulation:** improve simulation APIs ([#320](https://github.com/ucb-substrate/substrate2/issues/320)) ([4ed59a1](https://github.com/ucb-substrate/substrate2/commit/4ed59a1283f9546e8336cc96015bd87c55682777))
* **stdcells:** add standard cell support to Sky130 PDK ([#323](https://github.com/ucb-substrate/substrate2/issues/323)) ([0b2048e](https://github.com/ucb-substrate/substrate2/commit/0b2048ed44d89c5de87380cac48a4bbff2b4c20a))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.7.1 to 0.8.0

## [0.6.0](https://github.com/ucb-substrate/substrate2/compare/spice-v0.5.0...spice-v0.6.0) (2023-11-04)


### Features

* **spice:** refactor netlisting and fix voltage source netlist ([#316](https://github.com/ucb-substrate/substrate2/issues/316)) ([7a3df69](https://github.com/ucb-substrate/substrate2/commit/7a3df695cf9b38c837ff86d5a5da2417c4db7aa2))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * scir bumped from 0.6.0 to 0.7.0
    * substrate bumped from 0.7.0 to 0.7.1

## [0.5.0](https://github.com/ucb-substrate/substrate2/compare/spice-v0.4.0...spice-v0.5.0) (2023-11-02)


### Features

* **netlister:** reduce duplicate code between spectre and SPICE netlisters ([#261](https://github.com/ucb-substrate/substrate2/issues/261)) ([5ba3623](https://github.com/ucb-substrate/substrate2/commit/5ba36230e653e4dc77819c5c50b527311768cd83))
* **netlists:** use consistent ordering via indexmap ([#266](https://github.com/ucb-substrate/substrate2/issues/266)) ([f275c19](https://github.com/ucb-substrate/substrate2/commit/f275c19396ed4f7d255836822ff72b808f89cde7)), closes [#265](https://github.com/ucb-substrate/substrate2/issues/265)
* **ngspice:** create ngspice simulator ([#274](https://github.com/ucb-substrate/substrate2/issues/274)) ([0205300](https://github.com/ucb-substrate/substrate2/commit/02053006bc26d0b3d9e1d380def89836d7921857))
* **primitives:** revamp schematic primitives ([#291](https://github.com/ucb-substrate/substrate2/issues/291)) ([e5ba06a](https://github.com/ucb-substrate/substrate2/commit/e5ba06ab10008b72e78397ad70781caa6bc61791))


### Bug Fixes

* **deps:** remove opacity from spice library ([#287](https://github.com/ucb-substrate/substrate2/issues/287)) ([a45b728](https://github.com/ucb-substrate/substrate2/commit/a45b7288e240a9955d91acb437fa251fccb66b75))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * scir bumped from 0.5.0 to 0.6.0
    * substrate bumped from 0.6.1 to 0.7.0

## [0.4.0](https://github.com/substrate-labs/substrate2/compare/spice-v0.3.0...spice-v0.4.0) (2023-08-08)


### Features

* **slices:** use `SliceOne` instead of `Slice` where possible ([#253](https://github.com/substrate-labs/substrate2/issues/253)) ([8eba8ed](https://github.com/substrate-labs/substrate2/commit/8eba8ed5aad0aa4911ae31f4521d297487256087))
* **validation:** create type for unvalidated SCIR library ([#252](https://github.com/substrate-labs/substrate2/issues/252)) ([1550a22](https://github.com/substrate-labs/substrate2/commit/1550a22b9a1c9f7cd9717feaa45d00487cc8848e))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * scir bumped from 0.4.0 to 0.5.0

## [0.3.0](https://github.com/substrate-labs/substrate2/compare/spice-v0.2.0...spice-v0.3.0) (2023-08-05)


### Features

* **terminals:** add support for terminal paths ([#236](https://github.com/substrate-labs/substrate2/issues/236)) ([3fba7f6](https://github.com/substrate-labs/substrate2/commit/3fba7f6227bbf2efcaf79d849c79175e44d783a4))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * scir bumped from 0.3.0 to 0.4.0

## [0.2.0](https://github.com/substrate-labs/substrate2/compare/spice-v0.1.0...spice-v0.2.0) (2023-08-04)


### Features

* **primitives:** support parameters in SCIR primitive devices ([#232](https://github.com/substrate-labs/substrate2/issues/232)) ([a8f5b45](https://github.com/substrate-labs/substrate2/commit/a8f5b45a00b77d050f6a812c469e19da3305e064))
* **repo:** reorganize repo ([#207](https://github.com/substrate-labs/substrate2/issues/207)) ([54a6b43](https://github.com/substrate-labs/substrate2/commit/54a6b43079d283a29bc0aa9e18dc6230b56fa385))
* **schematics:** blackboxes can reference nodes ([#208](https://github.com/substrate-labs/substrate2/issues/208)) ([d998b4a](https://github.com/substrate-labs/substrate2/commit/d998b4a133d47d0123768dfb3c27f8ee32ed9db9))
* **spice-parser:** spice parser follows include directives ([#229](https://github.com/substrate-labs/substrate2/issues/229)) ([5259acf](https://github.com/substrate-labs/substrate2/commit/5259acfa703c3879d44d324279293278c46f1ff5))
* **validation:** SCIR driver analysis and validation ([#239](https://github.com/substrate-labs/substrate2/issues/239)) ([5a91448](https://github.com/substrate-labs/substrate2/commit/5a914489294bed06be1bd34aaa1036e4357d9a52))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * scir bumped from 0.2.0 to 0.3.0

## [0.1.0](https://github.com/substrate-labs/substrate2/compare/spice-v0.0.0...spice-v0.1.0) (2023-07-23)


### Features

* **conversion:** convert parsed SPICE to SCIR ([#178](https://github.com/substrate-labs/substrate2/issues/178)) ([9cb7bc3](https://github.com/substrate-labs/substrate2/commit/9cb7bc3ba549ae12e7a59465241c848800c39363))
* **organization:** move `spice` from netlist/ to libs/ ([#174](https://github.com/substrate-labs/substrate2/issues/174)) ([bd31a44](https://github.com/substrate-labs/substrate2/commit/bd31a4481aef357daeb2c217dd7f403f6f882f78))
* **proc-macros:** codegen for schematic hard macros ([#191](https://github.com/substrate-labs/substrate2/issues/191)) ([50240b1](https://github.com/substrate-labs/substrate2/commit/50240b167876873c4133315d35298b44e8eeac51))
* **scir-instances:** allow Substrate users to instantiate raw SCIR instances ([#184](https://github.com/substrate-labs/substrate2/issues/184)) ([8fd5192](https://github.com/substrate-labs/substrate2/commit/8fd5192fd2017ab04e9e3220612d0a132702bb2e))
* **spice-to-scir:** do not convert blackboxed subcircuits ([#179](https://github.com/substrate-labs/substrate2/issues/179)) ([c501313](https://github.com/substrate-labs/substrate2/commit/c501313334279b636f1d8b581357dd805177f1ca))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * scir bumped from 0.1.0 to 0.2.0
