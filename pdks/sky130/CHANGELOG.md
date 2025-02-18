# Changelog

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.1.0 to 0.1.1
    * spectre bumped from 0.1.0 to 0.1.1

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.6.0 to 0.6.1
    * spectre bumped from 0.6.0 to 0.6.1

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.7.0 to 0.7.1
    * scir bumped from 0.6.0 to 0.7.0
    * spectre bumped from 0.7.0 to 0.8.0
    * ngspice bumped from 0.1.0 to 0.2.0
    * spice bumped from 0.5.0 to 0.6.0

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.8.0 to 0.8.1
    * spectre bumped from 0.9.0 to 0.9.1
    * ngspice bumped from 0.3.0 to 0.3.1
    * spice bumped from 0.7.0 to 0.7.1

## [0.10.3](https://github.com/ucb-substrate/substrate2/compare/sky130-v0.10.2...sky130-v0.10.3) (2025-02-18)


### Features

* **gds:** convert GDS to generic layer type via FromGds trait ([#590](https://github.com/ucb-substrate/substrate2/issues/590)) ([1b98f28](https://github.com/ucb-substrate/substrate2/commit/1b98f289b4cd5b94f4248691b35bad8ec73b83c5))
* **gds:** support importing GDS libraries into sky130 ([#583](https://github.com/ucb-substrate/substrate2/issues/583)) ([5e3181b](https://github.com/ucb-substrate/substrate2/commit/5e3181b1307e32a017126028fc15a13255129195))
* **stdcells:** implement layout for sky130 stdcells ([#586](https://github.com/ucb-substrate/substrate2/issues/586)) ([6e438ec](https://github.com/ucb-substrate/substrate2/commit/6e438ecde6b092231b4f9b6f17e3004663c17f74))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.10.2 to 0.10.3
    * layir bumped from 0.2.1 to 0.2.2
    * gdsconv bumped from 0.2.1 to 0.2.2
    * gds bumped from 0.4.1 to 0.4.2
    * spectre bumped from 0.11.2 to 0.11.3
    * ngspice bumped from 0.5.2 to 0.5.3
    * spice bumped from 0.9.2 to 0.9.3
    * geometry bumped from 0.7.1 to 0.7.2

## [0.10.2](https://github.com/ucb-substrate/substrate2/compare/sky130-v0.10.1...sky130-v0.10.2) (2025-02-02)


### Features

* **sky130spconv:** CLI to convert between sky130 schemas ([#570](https://github.com/ucb-substrate/substrate2/issues/570)) ([a705a71](https://github.com/ucb-substrate/substrate2/commit/a705a71238d61794dd5c322b3b55594d4719886b))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.10.1 to 0.10.2
    * spectre bumped from 0.11.1 to 0.11.2
    * ngspice bumped from 0.5.1 to 0.5.2
    * spice bumped from 0.9.1 to 0.9.2

## [0.10.1](https://github.com/ucb-substrate/substrate2/compare/sky130-v0.10.0...sky130-v0.10.1) (2025-01-24)


### Dependencies

* update dependencies ([0b87032](https://github.com/ucb-substrate/substrate2/commit/0b8703276631fbb19a958453394c981d6b092441))
* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.10.0 to 0.10.1
    * scir bumped from 0.9.0 to 0.9.1
    * layir bumped from 0.2.0 to 0.2.1
    * gdsconv bumped from 0.2.0 to 0.2.1
    * gds bumped from 0.4.0 to 0.4.1
    * spectre bumped from 0.11.0 to 0.11.1
    * ngspice bumped from 0.5.0 to 0.5.1
    * spice bumped from 0.9.0 to 0.9.1
    * geometry_macros bumped from 0.1.0 to 0.1.1
    * geometry bumped from 0.7.0 to 0.7.1

## [0.10.0](https://github.com/ucb-substrate/substrate2/compare/sky130-v0.9.0...sky130-v0.10.0) (2025-01-23)


### Features

* **docs:** inverter tutorial cleanup and layout/pex sections ([#487](https://github.com/ucb-substrate/substrate2/issues/487)) ([5e509df](https://github.com/ucb-substrate/substrate2/commit/5e509df95a5c145fc69280269d36d860418fb1c0))


### Bug Fixes

* **ci:** use head_ref instead of ref and fix gdsconv version ([#498](https://github.com/ucb-substrate/substrate2/issues/498)) ([bc5d66e](https://github.com/ucb-substrate/substrate2/commit/bc5d66e5aad82ea79436e2fb3ec33e960a58f7b6))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.9.0 to 0.10.0
    * scir bumped from 0.8.0 to 0.9.0
    * layir bumped from 0.1.0 to 0.2.0
    * gdsconv bumped from 0.1.0 to 0.2.0
    * gds bumped from 0.3.1 to 0.4.0
    * spectre bumped from 0.10.0 to 0.11.0
    * ngspice bumped from 0.4.0 to 0.5.0
    * spice bumped from 0.8.0 to 0.9.0
    * geometry_macros bumped from 0.0.2 to 0.0.3
    * geometry bumped from 0.6.0 to 0.7.0

## [0.9.0](https://github.com/ucb-substrate/substrate2/compare/sky130-v0.8.1...sky130-v0.9.0) (2025-01-22)


### Features

* **docs:** inverter tutorial cleanup and layout/pex sections ([#487](https://github.com/ucb-substrate/substrate2/issues/487)) ([5e509df](https://github.com/ucb-substrate/substrate2/commit/5e509df95a5c145fc69280269d36d860418fb1c0))


### Bug Fixes

* **ci:** use head_ref instead of ref and fix gdsconv version ([#498](https://github.com/ucb-substrate/substrate2/issues/498)) ([bc5d66e](https://github.com/ucb-substrate/substrate2/commit/bc5d66e5aad82ea79436e2fb3ec33e960a58f7b6))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.8.1 to 0.9.0
    * scir bumped from 0.7.0 to 0.8.0
    * layir bumped from 0.0.0 to 0.1.0
    * gdsconv bumped from 0.0.0 to 0.1.0
    * gds bumped from 0.3.0 to 0.3.1
    * spectre bumped from 0.9.1 to 0.10.0
    * ngspice bumped from 0.3.1 to 0.4.0
    * spice bumped from 0.7.1 to 0.8.0
    * geometry_macros bumped from 0.0.1 to 0.0.2
    * geometry bumped from 0.5.0 to 0.6.0

## [0.8.0](https://github.com/ucb-substrate/substrate2/compare/sky130pdk-v0.7.1...sky130pdk-v0.8.0) (2023-11-25)


### Features

* **simulation:** improve simulation APIs ([#320](https://github.com/ucb-substrate/substrate2/issues/320)) ([4ed59a1](https://github.com/ucb-substrate/substrate2/commit/4ed59a1283f9546e8336cc96015bd87c55682777))
* **stdcells:** add standard cell support to Sky130 PDK ([#323](https://github.com/ucb-substrate/substrate2/issues/323)) ([0b2048e](https://github.com/ucb-substrate/substrate2/commit/0b2048ed44d89c5de87380cac48a4bbff2b4c20a))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.7.1 to 0.8.0
    * spectre bumped from 0.8.0 to 0.9.0
    * ngspice bumped from 0.2.0 to 0.3.0
    * spice bumped from 0.6.0 to 0.7.0

## [0.7.0](https://github.com/ucb-substrate/substrate2/compare/sky130pdk-v0.6.1...sky130pdk-v0.7.0) (2023-11-02)


### Features

* **netlists:** use consistent ordering via indexmap ([#266](https://github.com/ucb-substrate/substrate2/issues/266)) ([f275c19](https://github.com/ucb-substrate/substrate2/commit/f275c19396ed4f7d255836822ff72b808f89cde7)), closes [#265](https://github.com/ucb-substrate/substrate2/issues/265)
* **ngspice:** create ngspice simulator ([#274](https://github.com/ucb-substrate/substrate2/issues/274)) ([0205300](https://github.com/ucb-substrate/substrate2/commit/02053006bc26d0b3d9e1d380def89836d7921857))
* **primitives:** revamp schematic primitives ([#291](https://github.com/ucb-substrate/substrate2/issues/291)) ([e5ba06a](https://github.com/ucb-substrate/substrate2/commit/e5ba06ab10008b72e78397ad70781caa6bc61791))
* **refactor:** rename Has_ and Has_Data ([#282](https://github.com/ucb-substrate/substrate2/issues/282)) ([2018153](https://github.com/ucb-substrate/substrate2/commit/2018153686dd7ef3df0e10874db3c656ca245026))


### Bug Fixes

* **deps:** update rust crate rust_decimal to 1.32 ([#296](https://github.com/ucb-substrate/substrate2/issues/296)) ([a2fe877](https://github.com/ucb-substrate/substrate2/commit/a2fe877d03d3f907f348d7711a2132194ae91034))
* **deps:** update rust crate rust_decimal_macros to 1.32 ([#297](https://github.com/ucb-substrate/substrate2/issues/297)) ([5474cc8](https://github.com/ucb-substrate/substrate2/commit/5474cc8778b81c30b34fc7d146eec6e5e2532a26))
* **mos:** flatten SKY130 PDK MOS devices ([#271](https://github.com/ucb-substrate/substrate2/issues/271)) ([f4ce572](https://github.com/ucb-substrate/substrate2/commit/f4ce572ded2b5d1942113d3002a8de6f0c57c0f9))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.6.1 to 0.7.0
    * scir bumped from 0.5.0 to 0.6.0
    * spectre bumped from 0.6.1 to 0.7.0
    * ngspice bumped from 0.0.0 to 0.1.0
    * spice bumped from 0.4.0 to 0.5.0

## [0.6.0](https://github.com/substrate-labs/substrate2/compare/sky130pdk-v0.5.0...sky130pdk-v0.6.0) (2023-08-08)


### Features

* **macros:** refactor macro reexports ([#250](https://github.com/substrate-labs/substrate2/issues/250)) ([a332717](https://github.com/substrate-labs/substrate2/commit/a332717e549fdea50306067e1c92dc60293aed4c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.5.0 to 0.6.0
    * spectre bumped from 0.5.0 to 0.6.0

## [0.5.0](https://github.com/substrate-labs/substrate2/compare/sky130pdk-v0.4.0...sky130pdk-v0.5.0) (2023-08-05)


### Features

* **terminals:** add support for terminal paths ([#236](https://github.com/substrate-labs/substrate2/issues/236)) ([3fba7f6](https://github.com/substrate-labs/substrate2/commit/3fba7f6227bbf2efcaf79d849c79175e44d783a4))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.4.0 to 0.5.0
    * spectre bumped from 0.4.0 to 0.5.0

## [0.4.0](https://github.com/substrate-labs/substrate2/compare/sky130pdk-v0.3.0...sky130pdk-v0.4.0) (2023-08-04)


### Features

* **corners:** require specifying corner by default ([#221](https://github.com/substrate-labs/substrate2/issues/221)) ([4c2c3e4](https://github.com/substrate-labs/substrate2/commit/4c2c3e4a3cd8b7e68921baf3af8b87f1da048936))
* **io:** composable port directions and runtime connection checking ([#231](https://github.com/substrate-labs/substrate2/issues/231)) ([e1e367a](https://github.com/substrate-labs/substrate2/commit/e1e367a2b8940319cb4f804888746a094f06e161))
* **parameters:** substrate schematic primitives support parameters ([#233](https://github.com/substrate-labs/substrate2/issues/233)) ([5dabcb2](https://github.com/substrate-labs/substrate2/commit/5dabcb270cab0d259b7301d67f77de6d55261092))
* **pdk:** remove `PdkData` object to clean up interface ([#218](https://github.com/substrate-labs/substrate2/issues/218)) ([1dd166a](https://github.com/substrate-labs/substrate2/commit/1dd166a8f23e7b3c011c01b5c8527b8c5494ddea))
* **save-api:** add typed API for saving arbitrary signals ([#228](https://github.com/substrate-labs/substrate2/issues/228)) ([046be02](https://github.com/substrate-labs/substrate2/commit/046be02acbedc7fa2bb4896b92ec17babd80eee5))
* **schematics:** rename `HasSchematic` and `HasSchematicImpl` ([#226](https://github.com/substrate-labs/substrate2/issues/226)) ([a2b9c78](https://github.com/substrate-labs/substrate2/commit/a2b9c78ea6ff56983e9a02aeafe655e92852c264))
* **spectre:** vsource uses primitives instead of being blackboxed ([5dabcb2](https://github.com/substrate-labs/substrate2/commit/5dabcb270cab0d259b7301d67f77de6d55261092))


### Bug Fixes

* **deps:** update rust crate rust_decimal to 1.31 ([#219](https://github.com/substrate-labs/substrate2/issues/219)) ([6f596d5](https://github.com/substrate-labs/substrate2/commit/6f596d5c46dc1bf045a1b8a5ef727adbc3b147cf))
* **deps:** update rust crate rust_decimal_macros to 1.31 ([#220](https://github.com/substrate-labs/substrate2/issues/220)) ([72147d3](https://github.com/substrate-labs/substrate2/commit/72147d385368e2bd302821c981dd75209aa87dcb))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.3.0 to 0.4.0
    * spectre bumped from 0.3.0 to 0.4.0

## [0.3.0](https://github.com/substrate-labs/substrate2/compare/sky130pdk-v0.2.0...sky130pdk-v0.3.0) (2023-07-23)


### Features

* **gds-import:** implement GDS to RawCell importer ([#196](https://github.com/substrate-labs/substrate2/issues/196)) ([fc37eeb](https://github.com/substrate-labs/substrate2/commit/fc37eeb6bac10779491b98bcadcc0eeaeb7d8ec5))
* **pdks:** PDKs store the names of schematic primitives ([#185](https://github.com/substrate-labs/substrate2/issues/185)) ([3446ba8](https://github.com/substrate-labs/substrate2/commit/3446ba869f564f844b39ee524b52106954a293c5))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.2.0 to 0.3.0
    * spectre bumped from 0.2.0 to 0.3.0

## [0.2.0](https://github.com/substrate-labs/substrate2/compare/sky130pdk-v0.1.1...sky130pdk-v0.2.0) (2023-07-07)


### Features

* **reorg:** move substrate-api into substrate ([#155](https://github.com/substrate-labs/substrate2/issues/155)) ([e902a1b](https://github.com/substrate-labs/substrate2/commit/e902a1b603cca6c719770c5cd742e081bfd33e51))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.1.1 to 0.2.0
    * spectre bumped from 0.1.1 to 0.2.0

## 0.1.0 (2023-07-07)


### Features

* **corners:** add API for process corners ([#141](https://github.com/substrate-labs/substrate2/issues/141)) ([a61b15a](https://github.com/substrate-labs/substrate2/commit/a61b15a80851a6393aaa9da2db41e01a34f0ce5b))
* **layer-families:** implement layer families and clean up codegen ([#127](https://github.com/substrate-labs/substrate2/issues/127)) ([06f50b8](https://github.com/substrate-labs/substrate2/commit/06f50b8236ba40f405d7a5e20987a28e01f69f7c))
* **layers:** initial layer set for SKY130 PDK ([#120](https://github.com/substrate-labs/substrate2/issues/120)) ([1ea5a7e](https://github.com/substrate-labs/substrate2/commit/1ea5a7ee08ebe5e4f3f1c93f9d52424286b0443b))
* **mos:** add sky130pdk transistor blocks ([#126](https://github.com/substrate-labs/substrate2/issues/126)) ([3e9ee79](https://github.com/substrate-labs/substrate2/commit/3e9ee7935e030ca3e5c4d56f19ccafc27445a6f0))
* **mos:** add standard 4-terminal MosIo ([8678716](https://github.com/substrate-labs/substrate2/commit/86787160c49a1ac7c011d08ce1b9d7851bdfa0d8))
* **proc-macros:** allow missing docs on generated structs ([8678716](https://github.com/substrate-labs/substrate2/commit/86787160c49a1ac7c011d08ce1b9d7851bdfa0d8))
* **proc-macros:** macros respect field and struct visibilities ([8678716](https://github.com/substrate-labs/substrate2/commit/86787160c49a1ac7c011d08ce1b9d7851bdfa0d8))
* **proc-macros:** proc macros find substrate crate location ([#125](https://github.com/substrate-labs/substrate2/issues/125)) ([8678716](https://github.com/substrate-labs/substrate2/commit/86787160c49a1ac7c011d08ce1b9d7851bdfa0d8))
* **sky130pdk:** add Sky130Pdk struct definition ([#124](https://github.com/substrate-labs/substrate2/issues/124)) ([06ced7a](https://github.com/substrate-labs/substrate2/commit/06ced7ad90162d066e841513cf33e4ec2acc042c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.0.0 to 0.1.0
    * spectre bumped from 0.0.0 to 0.1.0
