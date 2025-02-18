# Changelog

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.1.0 to 0.1.1

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.6.0 to 0.6.1

## [0.11.3](https://github.com/ucb-substrate/substrate2/compare/spectre-v0.11.2...spectre-v0.11.3) (2025-02-18)


### Features

* **spectre:** add support for IBIS model includes ([#591](https://github.com/ucb-substrate/substrate2/issues/591)) ([8ec582f](https://github.com/ucb-substrate/substrate2/commit/8ec582f7088c3c863fba1db2ed899dd1a25bab54))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.10.2 to 0.10.3
    * spice bumped from 0.9.2 to 0.9.3

## [0.11.2](https://github.com/ucb-substrate/substrate2/compare/spectre-v0.11.1...spectre-v0.11.2) (2025-02-02)


### Features

* **dcop:** support Spectre dc operating point simulation ([#563](https://github.com/ucb-substrate/substrate2/issues/563)) ([76bf16d](https://github.com/ucb-substrate/substrate2/commit/76bf16d10b4ea43f61049bad499382d8e3582d88))
* **psfparser:** add psfparser to Substrate ([#561](https://github.com/ucb-substrate/substrate2/issues/561)) ([42ae7fb](https://github.com/ucb-substrate/substrate2/commit/42ae7fbc27744151c6c5282a9b0fd73861138987))


### Bug Fixes

* **process:** spawn processes with stdin set to null ([#560](https://github.com/ucb-substrate/substrate2/issues/560)) ([a6bc7d1](https://github.com/ucb-substrate/substrate2/commit/a6bc7d12d631494fd0dead3732e3068ec396cc93))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * psfparser bumped from 0.1.3 to 0.1.4
    * substrate bumped from 0.10.1 to 0.10.2
    * spice bumped from 0.9.1 to 0.9.2

## [0.11.1](https://github.com/ucb-substrate/substrate2/compare/spectre-v0.11.0...spectre-v0.11.1) (2025-01-24)


### Dependencies

* update dependencies ([0b87032](https://github.com/ucb-substrate/substrate2/commit/0b8703276631fbb19a958453394c981d6b092441))
* update dependencies ([#538](https://github.com/ucb-substrate/substrate2/issues/538)) ([19438d6](https://github.com/ucb-substrate/substrate2/commit/19438d65ac7078a2a971b4147420364ca0717763))
* update deps, GH actions ([#551](https://github.com/ucb-substrate/substrate2/issues/551)) ([357e82a](https://github.com/ucb-substrate/substrate2/commit/357e82ae0a01317d3ad5afb33b5290d3ac10cd7a))
* The following workspace dependencies were updated
  * dependencies
    * cache bumped from 0.7.0 to 0.7.1
    * scir bumped from 0.9.0 to 0.9.1
    * substrate bumped from 0.10.0 to 0.10.1
    * spice bumped from 0.9.0 to 0.9.1
    * type_dispatch bumped from 0.5.0 to 0.5.1

## [0.11.0](https://github.com/ucb-substrate/substrate2/compare/spectre-v0.10.0...spectre-v0.11.0) (2025-01-23)


### Features

* **ac:** implement Save for Substrate types in Spectre AC sim, fix lints ([#471](https://github.com/ucb-substrate/substrate2/issues/471)) ([9825520](https://github.com/ucb-substrate/substrate2/commit/98255207569cc00bd9ddc35419c2df1e48f1999c))
* **blackboxing:** support Substrate and SCIR blackboxes ([#135](https://github.com/ucb-substrate/substrate2/issues/135)) ([049a598](https://github.com/ucb-substrate/substrate2/commit/049a598e2b8d11228c63f03dc878fc4c56e036a6))
* **cache:** implement persistent caching ([#171](https://github.com/ucb-substrate/substrate2/issues/171)) ([1f8ea24](https://github.com/ucb-substrate/substrate2/commit/1f8ea24f805085392bfd1a2067bb8774d0fa4ae4))
* **cadence:** add support for Pegasus and Quantus ([#462](https://github.com/ucb-substrate/substrate2/issues/462)) ([953e4cb](https://github.com/ucb-substrate/substrate2/commit/953e4cb761c510668f65f4825f1be3914db45e3c))
* **cdl:** add CDL parser ([#419](https://github.com/ucb-substrate/substrate2/issues/419)) ([23f0dab](https://github.com/ucb-substrate/substrate2/commit/23f0dab7b7a94cbe8960371b89d15211bddf51da))
* **codegen:** implement derive proc macro for layout hard macros ([#200](https://github.com/ucb-substrate/substrate2/issues/200)) ([5138224](https://github.com/ucb-substrate/substrate2/commit/5138224013f537e678dfb20204e964852ed40ccb))
* **conv:** better error messages in schema conversions ([#440](https://github.com/ucb-substrate/substrate2/issues/440)) ([bad9503](https://github.com/ucb-substrate/substrate2/commit/bad9503b8a3b98d8e0bc19779ed45e7628164f41))
* **corners:** add API for process corners ([#141](https://github.com/ucb-substrate/substrate2/issues/141)) ([a61b15a](https://github.com/ucb-substrate/substrate2/commit/a61b15a80851a6393aaa9da2db41e01a34f0ce5b))
* **docs:** fix user docs and update dev docs ([#480](https://github.com/ucb-substrate/substrate2/issues/480)) ([f727a1e](https://github.com/ucb-substrate/substrate2/commit/f727a1e7bd2a795ace1c51c3d6e02f3673d07a29))
* **docs:** update docs for new simulation APIs ([#326](https://github.com/ucb-substrate/substrate2/issues/326)) ([ef133df](https://github.com/ucb-substrate/substrate2/commit/ef133dfac5f352121fe0e561b76541d5af62970e))
* **dspf:** propagate nested nodes from DSPF instances ([#407](https://github.com/ucb-substrate/substrate2/issues/407)) ([8455bd2](https://github.com/ucb-substrate/substrate2/commit/8455bd2a523bb872dc1ce3fc0e89a185108dca3c))
* **executors:** executor API and local executor implementation ([#161](https://github.com/ucb-substrate/substrate2/issues/161)) ([c372d51](https://github.com/ucb-substrate/substrate2/commit/c372d511e1c67ad976dc86958c87e9ad13bdfde4))
* **ics:** spectre initial conditions ([#275](https://github.com/ucb-substrate/substrate2/issues/275)) ([ce3724e](https://github.com/ucb-substrate/substrate2/commit/ce3724e9e907f3eb3653dbf39f763865914235e3))
* **includes:** add support for including sections in spectre ([#205](https://github.com/ucb-substrate/substrate2/issues/205)) ([8522ff4](https://github.com/ucb-substrate/substrate2/commit/8522ff4241755d4c194bacb893765e608122814e))
* **inverter-design:** initial inverter design logic ([#144](https://github.com/ucb-substrate/substrate2/issues/144)) ([911a376](https://github.com/ucb-substrate/substrate2/commit/911a3768bf730d7fd134310d500560873f008783))
* **io:** composable port directions and runtime connection checking ([#231](https://github.com/ucb-substrate/substrate2/issues/231)) ([e1e367a](https://github.com/ucb-substrate/substrate2/commit/e1e367a2b8940319cb4f804888746a094f06e161))
* **macros:** refactor derive NestedData, start organizing tests ([#477](https://github.com/ucb-substrate/substrate2/issues/477)) ([aca48ef](https://github.com/ucb-substrate/substrate2/commit/aca48ef7a49c959e35ec4614345a55e667ff5146))
* **macros:** refactor macro reexports ([#250](https://github.com/ucb-substrate/substrate2/issues/250)) ([a332717](https://github.com/ucb-substrate/substrate2/commit/a332717e549fdea50306067e1c92dc60293aed4c))
* **montecarlo:** add Monte Carlo simulation support to Spectre plugin ([#347](https://github.com/ucb-substrate/substrate2/issues/347)) ([cc9dfe4](https://github.com/ucb-substrate/substrate2/commit/cc9dfe42db5be1a8aaeaf3fb81992a0ad7251ef8))
* **netlister:** reduce duplicate code between spectre and SPICE netlisters ([#261](https://github.com/ucb-substrate/substrate2/issues/261)) ([5ba3623](https://github.com/ucb-substrate/substrate2/commit/5ba36230e653e4dc77819c5c50b527311768cd83))
* **netlisting:** implement basic Spectre netlister ([#130](https://github.com/ucb-substrate/substrate2/issues/130)) ([506837f](https://github.com/ucb-substrate/substrate2/commit/506837fca0f1964ebc622df970575b321456ab68))
* **netlists:** consistent Spectre/Spice netlist API ([#349](https://github.com/ucb-substrate/substrate2/issues/349)) ([2f9fabf](https://github.com/ucb-substrate/substrate2/commit/2f9fabf336fa1048d759e78834979ef892fc0bcf))
* **netlists:** support ideal 2-terminal capacitors ([#269](https://github.com/ucb-substrate/substrate2/issues/269)) ([7de9843](https://github.com/ucb-substrate/substrate2/commit/7de9843c9b629ea06518448fe26d384de4a66cdc))
* **netlists:** use consistent ordering via indexmap ([#266](https://github.com/ucb-substrate/substrate2/issues/266)) ([f275c19](https://github.com/ucb-substrate/substrate2/commit/f275c19396ed4f7d255836822ff72b808f89cde7)), closes [#265](https://github.com/ucb-substrate/substrate2/issues/265)
* **ngspice:** create ngspice simulator ([#274](https://github.com/ucb-substrate/substrate2/issues/274)) ([0205300](https://github.com/ucb-substrate/substrate2/commit/02053006bc26d0b3d9e1d380def89836d7921857))
* **parameters:** substrate schematic primitives support parameters ([#233](https://github.com/ucb-substrate/substrate2/issues/233)) ([5dabcb2](https://github.com/ucb-substrate/substrate2/commit/5dabcb270cab0d259b7301d67f77de6d55261092))
* **parser:** use nutmeg format for spectre output ([#289](https://github.com/ucb-substrate/substrate2/issues/289)) ([034f58f](https://github.com/ucb-substrate/substrate2/commit/034f58f99c587c61003761971e76c26038de9b3b))
* **primitives:** revamp schematic primitives ([#291](https://github.com/ucb-substrate/substrate2/issues/291)) ([e5ba06a](https://github.com/ucb-substrate/substrate2/commit/e5ba06ab10008b72e78397ad70781caa6bc61791))
* **primitives:** support parameters in SCIR primitive devices ([#232](https://github.com/ucb-substrate/substrate2/issues/232)) ([a8f5b45](https://github.com/ucb-substrate/substrate2/commit/a8f5b45a00b77d050f6a812c469e19da3305e064))
* **psf:** use PSF binary format for Spectre plugin ([#345](https://github.com/ucb-substrate/substrate2/issues/345)) ([a4ec152](https://github.com/ucb-substrate/substrate2/commit/a4ec152d5e1299bc38f2664fe900dd7d34ba8b5c))
* **refactor:** rename Has_ and Has_Data ([#282](https://github.com/ucb-substrate/substrate2/issues/282)) ([2018153](https://github.com/ucb-substrate/substrate2/commit/2018153686dd7ef3df0e10874db3c656ca245026))
* **refactor:** significantly refactor IO APIs ([#348](https://github.com/ucb-substrate/substrate2/issues/348)) ([c85d043](https://github.com/ucb-substrate/substrate2/commit/c85d04334a0ba1740f9990b91fb55ab1f2ef77c5))
* **reorg:** move substrate-api into substrate ([#155](https://github.com/ucb-substrate/substrate2/issues/155)) ([e902a1b](https://github.com/ucb-substrate/substrate2/commit/e902a1b603cca6c719770c5cd742e081bfd33e51))
* **repo:** reorganize repo ([#207](https://github.com/ucb-substrate/substrate2/issues/207)) ([54a6b43](https://github.com/ucb-substrate/substrate2/commit/54a6b43079d283a29bc0aa9e18dc6230b56fa385))
* **save-api:** add typed API for saving arbitrary signals ([#228](https://github.com/ucb-substrate/substrate2/issues/228)) ([046be02](https://github.com/ucb-substrate/substrate2/commit/046be02acbedc7fa2bb4896b92ec17babd80eee5))
* **schematic:** associated type schema and bundle primitives ([#455](https://github.com/ucb-substrate/substrate2/issues/455)) ([f5fde78](https://github.com/ucb-substrate/substrate2/commit/f5fde78824ce9ed0be494ef68d71620181bf6b48))
* **schematics:** blackboxes can reference nodes ([#208](https://github.com/ucb-substrate/substrate2/issues/208)) ([d998b4a](https://github.com/ucb-substrate/substrate2/commit/d998b4a133d47d0123768dfb3c27f8ee32ed9db9))
* **schematics:** rename `HasSchematic` and `HasSchematicImpl` ([#226](https://github.com/ucb-substrate/substrate2/issues/226)) ([a2b9c78](https://github.com/ucb-substrate/substrate2/commit/a2b9c78ea6ff56983e9a02aeafe655e92852c264))
* **scir:** uniquify names when exporting to SCIR ([#148](https://github.com/ucb-substrate/substrate2/issues/148)) ([29c2f72](https://github.com/ucb-substrate/substrate2/commit/29c2f729f5a205b144053b61c0d8c0ca2446071b))
* **sim:** allow setting temp in Spectre sims ([#401](https://github.com/ucb-substrate/substrate2/issues/401)) ([0557fce](https://github.com/ucb-substrate/substrate2/commit/0557fceb1f0da4799914b0ea4a1e0919aed97bc7))
* **simulation:** access nested nodes without strings in simulation ([#139](https://github.com/ucb-substrate/substrate2/issues/139)) ([ed7989c](https://github.com/ucb-substrate/substrate2/commit/ed7989cfb190528163a1722ae5fe3383ec3c4310))
* **simulation:** automatically generate saved data ([#457](https://github.com/ucb-substrate/substrate2/issues/457)) ([2c936d0](https://github.com/ucb-substrate/substrate2/commit/2c936d00e927b99b624f29e6450826e90f68f9bf))
* **simulation:** implement save for nested instances ([#476](https://github.com/ucb-substrate/substrate2/issues/476)) ([a47d905](https://github.com/ucb-substrate/substrate2/commit/a47d905097c6c196153b53f142ca7e1ffba5eb51))
* **simulation:** improve simulation APIs ([#320](https://github.com/ucb-substrate/substrate2/issues/320)) ([4ed59a1](https://github.com/ucb-substrate/substrate2/commit/4ed59a1283f9546e8336cc96015bd87c55682777))
* **simulation:** simplify SCIR paths for data access ([#143](https://github.com/ucb-substrate/substrate2/issues/143)) ([d42e6f9](https://github.com/ucb-substrate/substrate2/commit/d42e6f9b1d4236a9024d4a4b839319749033b8d3))
* **simulation:** support transient simulation in spectre ([#133](https://github.com/ucb-substrate/substrate2/issues/133)) ([4605862](https://github.com/ucb-substrate/substrate2/commit/460586252e3695ae32b0ab8d83b90023125d1a33))
* **simulation:** testbench schematic components ([#136](https://github.com/ucb-substrate/substrate2/issues/136)) ([97e6b0f](https://github.com/ucb-substrate/substrate2/commit/97e6b0ffd5ea7abd2a547952d5c963745854ed75))
* **slices:** use `SliceOne` instead of `Slice` where possible ([#253](https://github.com/ucb-substrate/substrate2/issues/253)) ([8eba8ed](https://github.com/ucb-substrate/substrate2/commit/8eba8ed5aad0aa4911ae31f4521d297487256087))
* **spectre:** add `global 0` to spectre netlists ([#387](https://github.com/ucb-substrate/substrate2/issues/387)) ([19257b4](https://github.com/ucb-substrate/substrate2/commit/19257b45cbdf02acb22c1408cff0d9a578d437c3))
* **spectre:** add isource (current source) ([#369](https://github.com/ucb-substrate/substrate2/issues/369)) ([f318644](https://github.com/ucb-substrate/substrate2/commit/f318644d5ae554985a22d8abf274b6a8ff9c7ec9))
* **spectre:** allow overriding spectre flags ([#443](https://github.com/ucb-substrate/substrate2/issues/443)) ([5eebbe7](https://github.com/ucb-substrate/substrate2/commit/5eebbe7d3cd0e07a8431621c564af1d626fd8e7f))
* **spectre:** allow setting global save option ([#405](https://github.com/ucb-substrate/substrate2/issues/405)) ([7836a34](https://github.com/ucb-substrate/substrate2/commit/7836a34b1677332603ec6c437e0e8468f00f6c8d))
* **spectre:** support AC simulation ([#390](https://github.com/ucb-substrate/substrate2/issues/390)) ([dc3584a](https://github.com/ucb-substrate/substrate2/commit/dc3584a50ff8ebed525566a86d82033cf87d7b29))
* **spectre:** support n-port primitives ([#410](https://github.com/ucb-substrate/substrate2/issues/410)) ([693ab82](https://github.com/ucb-substrate/substrate2/commit/693ab8287876b3cd0517d34674c3ff069da2eff8))
* **spectre:** support SPF format primitives ([#386](https://github.com/ucb-substrate/substrate2/issues/386)) ([06adc0f](https://github.com/ucb-substrate/substrate2/commit/06adc0fb155161e2f05a735fe21d2c2361cd4930))
* **spectre:** support transient noise fmax/fmin ([#411](https://github.com/ucb-substrate/substrate2/issues/411)) ([df09ef0](https://github.com/ucb-substrate/substrate2/commit/df09ef00dfc361d2d542266a82a156a4948dbb66))
* **spectre:** use APS and multithreading flags ([#395](https://github.com/ucb-substrate/substrate2/issues/395)) ([facbca6](https://github.com/ucb-substrate/substrate2/commit/facbca6087d058bb6a421d09e0ec149eba6e3456))
* **spectre:** vsource uses primitives instead of being blackboxed ([5dabcb2](https://github.com/ucb-substrate/substrate2/commit/5dabcb270cab0d259b7301d67f77de6d55261092))
* **spice:** refactor netlisting and fix voltage source netlist ([#316](https://github.com/ucb-substrate/substrate2/issues/316)) ([7a3df69](https://github.com/ucb-substrate/substrate2/commit/7a3df695cf9b38c837ff86d5a5da2417c4db7aa2))
* **stdcells:** add standard cell support to Sky130 PDK ([#323](https://github.com/ucb-substrate/substrate2/issues/323)) ([0b2048e](https://github.com/ucb-substrate/substrate2/commit/0b2048ed44d89c5de87380cac48a4bbff2b4c20a))
* **terminals:** add support for terminal paths ([#236](https://github.com/ucb-substrate/substrate2/issues/236)) ([3fba7f6](https://github.com/ucb-substrate/substrate2/commit/3fba7f6227bbf2efcaf79d849c79175e44d783a4))
* **tests:** add SCIR and SPICE netlister blackbox tests ([049a598](https://github.com/ucb-substrate/substrate2/commit/049a598e2b8d11228c63f03dc878fc4c56e036a6))
* **tests:** fix compilation and lint errors ([#482](https://github.com/ucb-substrate/substrate2/issues/482)) ([b55d04e](https://github.com/ucb-substrate/substrate2/commit/b55d04ecd2472f9f72b926ba5286f0d928bc2691))
* **tests:** reorganize tests and documentation ([#464](https://github.com/ucb-substrate/substrate2/issues/464)) ([928b9b7](https://github.com/ucb-substrate/substrate2/commit/928b9b7c45dc334ca11d86e4564edc58bf6db6f2))
* **views:** view API for improved codegen ([#463](https://github.com/ucb-substrate/substrate2/issues/463)) ([b75328c](https://github.com/ucb-substrate/substrate2/commit/b75328c9a4840ed9200a9035e28e27ac9265770f))
* **waveform:** support generic waveform datatypes ([#379](https://github.com/ucb-substrate/substrate2/issues/379)) ([93e59fd](https://github.com/ucb-substrate/substrate2/commit/93e59fd8c005e2f7f2aeece9a637dff337e4ce68))
* **windows:** fix issues for windows ([#197](https://github.com/ucb-substrate/substrate2/issues/197)) ([008b607](https://github.com/ucb-substrate/substrate2/commit/008b607b2c21c14ac3106dca6eb74d806131ef8f))


### Bug Fixes

* **deps:** bump rust to version 1.75.0 ([#362](https://github.com/ucb-substrate/substrate2/issues/362)) ([e1e82c9](https://github.com/ucb-substrate/substrate2/commit/e1e82c94cdf6ba4426f3f73f29dca40674a7f064))
* **deps:** remove opacity from substrate and deps ([#288](https://github.com/ucb-substrate/substrate2/issues/288)) ([a8c97b3](https://github.com/ucb-substrate/substrate2/commit/a8c97b30b4d075343903fa580437e9a099a745a2))
* **deps:** update rust crate rust_decimal to 1.31 ([#219](https://github.com/ucb-substrate/substrate2/issues/219)) ([6f596d5](https://github.com/ucb-substrate/substrate2/commit/6f596d5c46dc1bf045a1b8a5ef727adbc3b147cf))
* **deps:** update rust crate rust_decimal to 1.32 ([#296](https://github.com/ucb-substrate/substrate2/issues/296)) ([a2fe877](https://github.com/ucb-substrate/substrate2/commit/a2fe877d03d3f907f348d7711a2132194ae91034))
* **deps:** update rust crate rust_decimal_macros to 1.31 ([#220](https://github.com/ucb-substrate/substrate2/issues/220)) ([72147d3](https://github.com/ucb-substrate/substrate2/commit/72147d385368e2bd302821c981dd75209aa87dcb))
* **deps:** update rust crate rust_decimal_macros to 1.32 ([#297](https://github.com/ucb-substrate/substrate2/issues/297)) ([5474cc8](https://github.com/ucb-substrate/substrate2/commit/5474cc8778b81c30b34fc7d146eec6e5e2532a26))
* **dspf:** add derives to dspf types ([#409](https://github.com/ucb-substrate/substrate2/issues/409)) ([81f00cd](https://github.com/ucb-substrate/substrate2/commit/81f00cde52a12fc1b96c007d556da55eafc4d0be))
* **netlisting:** update release manifests ([506837f](https://github.com/ucb-substrate/substrate2/commit/506837fca0f1964ebc622df970575b321456ab68))
* **scir:** add additional functionality for SCIR and SPICE libraries ([#337](https://github.com/ucb-substrate/substrate2/issues/337)) ([e49f075](https://github.com/ucb-substrate/substrate2/commit/e49f07529273c38cc8ec9ae1a5020ae48fb2a202))
* **simulation:** add missing SPICE functionality and update Sky 130 PDK ([#336](https://github.com/ucb-substrate/substrate2/issues/336)) ([f802be5](https://github.com/ucb-substrate/substrate2/commit/f802be5bf0361c38b415d976dbb0f2c984a2e304))
* **simulation:** standardize ngspice and spectre transient data formats ([#327](https://github.com/ucb-substrate/substrate2/issues/327)) ([0aa42d6](https://github.com/ucb-substrate/substrate2/commit/0aa42d6000d28a8aecb655e06330f4545e155b9b))
* **spectre:** escape ports in subckt declarations ([#441](https://github.com/ucb-substrate/substrate2/issues/441)) ([3eae4ad](https://github.com/ucb-substrate/substrate2/commit/3eae4adac5b03a326724d16bee722df6c4ec7cf2))
* **spectre:** make monte carlo return vec of analysis outputs ([#388](https://github.com/ucb-substrate/substrate2/issues/388)) ([01c382d](https://github.com/ucb-substrate/substrate2/commit/01c382d908939327bd9c1344be9d928524cba021))
* **spectre:** use default number of threads ([#414](https://github.com/ucb-substrate/substrate2/issues/414)) ([748c9e4](https://github.com/ucb-substrate/substrate2/commit/748c9e42c4a922a6f858d44291fafdceb1c1e11d))
* **waveform:** fix spectre PWL waveform netlisting ([#380](https://github.com/ucb-substrate/substrate2/issues/380)) ([a47d55c](https://github.com/ucb-substrate/substrate2/commit/a47d55cca56d2359a3f0522a2c9ed8205bbb49e3))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * cache bumped from 0.6.0 to 0.7.0
    * scir bumped from 0.8.0 to 0.9.0
    * substrate bumped from 0.9.0 to 0.10.0
    * spice bumped from 0.8.0 to 0.9.0
    * type_dispatch bumped from 0.4.0 to 0.5.0

## [0.10.0](https://github.com/ucb-substrate/substrate2/compare/spectre-v0.9.1...spectre-v0.10.0) (2025-01-22)


### Features

* **ac:** implement Save for Substrate types in Spectre AC sim, fix lints ([#471](https://github.com/ucb-substrate/substrate2/issues/471)) ([9825520](https://github.com/ucb-substrate/substrate2/commit/98255207569cc00bd9ddc35419c2df1e48f1999c))
* **cadence:** add support for Pegasus and Quantus ([#462](https://github.com/ucb-substrate/substrate2/issues/462)) ([953e4cb](https://github.com/ucb-substrate/substrate2/commit/953e4cb761c510668f65f4825f1be3914db45e3c))
* **cdl:** add CDL parser ([#419](https://github.com/ucb-substrate/substrate2/issues/419)) ([23f0dab](https://github.com/ucb-substrate/substrate2/commit/23f0dab7b7a94cbe8960371b89d15211bddf51da))
* **conv:** better error messages in schema conversions ([#440](https://github.com/ucb-substrate/substrate2/issues/440)) ([bad9503](https://github.com/ucb-substrate/substrate2/commit/bad9503b8a3b98d8e0bc19779ed45e7628164f41))
* **docs:** fix user docs and update dev docs ([#480](https://github.com/ucb-substrate/substrate2/issues/480)) ([f727a1e](https://github.com/ucb-substrate/substrate2/commit/f727a1e7bd2a795ace1c51c3d6e02f3673d07a29))
* **docs:** update docs for new simulation APIs ([#326](https://github.com/ucb-substrate/substrate2/issues/326)) ([ef133df](https://github.com/ucb-substrate/substrate2/commit/ef133dfac5f352121fe0e561b76541d5af62970e))
* **dspf:** propagate nested nodes from DSPF instances ([#407](https://github.com/ucb-substrate/substrate2/issues/407)) ([8455bd2](https://github.com/ucb-substrate/substrate2/commit/8455bd2a523bb872dc1ce3fc0e89a185108dca3c))
* **macros:** refactor derive NestedData, start organizing tests ([#477](https://github.com/ucb-substrate/substrate2/issues/477)) ([aca48ef](https://github.com/ucb-substrate/substrate2/commit/aca48ef7a49c959e35ec4614345a55e667ff5146))
* **montecarlo:** add Monte Carlo simulation support to Spectre plugin ([#347](https://github.com/ucb-substrate/substrate2/issues/347)) ([cc9dfe4](https://github.com/ucb-substrate/substrate2/commit/cc9dfe42db5be1a8aaeaf3fb81992a0ad7251ef8))
* **netlists:** consistent Spectre/Spice netlist API ([#349](https://github.com/ucb-substrate/substrate2/issues/349)) ([2f9fabf](https://github.com/ucb-substrate/substrate2/commit/2f9fabf336fa1048d759e78834979ef892fc0bcf))
* **psf:** use PSF binary format for Spectre plugin ([#345](https://github.com/ucb-substrate/substrate2/issues/345)) ([a4ec152](https://github.com/ucb-substrate/substrate2/commit/a4ec152d5e1299bc38f2664fe900dd7d34ba8b5c))
* **refactor:** significantly refactor IO APIs ([#348](https://github.com/ucb-substrate/substrate2/issues/348)) ([c85d043](https://github.com/ucb-substrate/substrate2/commit/c85d04334a0ba1740f9990b91fb55ab1f2ef77c5))
* **schematic:** associated type schema and bundle primitives ([#455](https://github.com/ucb-substrate/substrate2/issues/455)) ([f5fde78](https://github.com/ucb-substrate/substrate2/commit/f5fde78824ce9ed0be494ef68d71620181bf6b48))
* **sim:** allow setting temp in Spectre sims ([#401](https://github.com/ucb-substrate/substrate2/issues/401)) ([0557fce](https://github.com/ucb-substrate/substrate2/commit/0557fceb1f0da4799914b0ea4a1e0919aed97bc7))
* **simulation:** automatically generate saved data ([#457](https://github.com/ucb-substrate/substrate2/issues/457)) ([2c936d0](https://github.com/ucb-substrate/substrate2/commit/2c936d00e927b99b624f29e6450826e90f68f9bf))
* **simulation:** implement save for nested instances ([#476](https://github.com/ucb-substrate/substrate2/issues/476)) ([a47d905](https://github.com/ucb-substrate/substrate2/commit/a47d905097c6c196153b53f142ca7e1ffba5eb51))
* **spectre:** add `global 0` to spectre netlists ([#387](https://github.com/ucb-substrate/substrate2/issues/387)) ([19257b4](https://github.com/ucb-substrate/substrate2/commit/19257b45cbdf02acb22c1408cff0d9a578d437c3))
* **spectre:** add isource (current source) ([#369](https://github.com/ucb-substrate/substrate2/issues/369)) ([f318644](https://github.com/ucb-substrate/substrate2/commit/f318644d5ae554985a22d8abf274b6a8ff9c7ec9))
* **spectre:** allow overriding spectre flags ([#443](https://github.com/ucb-substrate/substrate2/issues/443)) ([5eebbe7](https://github.com/ucb-substrate/substrate2/commit/5eebbe7d3cd0e07a8431621c564af1d626fd8e7f))
* **spectre:** allow setting global save option ([#405](https://github.com/ucb-substrate/substrate2/issues/405)) ([7836a34](https://github.com/ucb-substrate/substrate2/commit/7836a34b1677332603ec6c437e0e8468f00f6c8d))
* **spectre:** support AC simulation ([#390](https://github.com/ucb-substrate/substrate2/issues/390)) ([dc3584a](https://github.com/ucb-substrate/substrate2/commit/dc3584a50ff8ebed525566a86d82033cf87d7b29))
* **spectre:** support n-port primitives ([#410](https://github.com/ucb-substrate/substrate2/issues/410)) ([693ab82](https://github.com/ucb-substrate/substrate2/commit/693ab8287876b3cd0517d34674c3ff069da2eff8))
* **spectre:** support SPF format primitives ([#386](https://github.com/ucb-substrate/substrate2/issues/386)) ([06adc0f](https://github.com/ucb-substrate/substrate2/commit/06adc0fb155161e2f05a735fe21d2c2361cd4930))
* **spectre:** support transient noise fmax/fmin ([#411](https://github.com/ucb-substrate/substrate2/issues/411)) ([df09ef0](https://github.com/ucb-substrate/substrate2/commit/df09ef00dfc361d2d542266a82a156a4948dbb66))
* **spectre:** use APS and multithreading flags ([#395](https://github.com/ucb-substrate/substrate2/issues/395)) ([facbca6](https://github.com/ucb-substrate/substrate2/commit/facbca6087d058bb6a421d09e0ec149eba6e3456))
* **tests:** fix compilation and lint errors ([#482](https://github.com/ucb-substrate/substrate2/issues/482)) ([b55d04e](https://github.com/ucb-substrate/substrate2/commit/b55d04ecd2472f9f72b926ba5286f0d928bc2691))
* **tests:** reorganize tests and documentation ([#464](https://github.com/ucb-substrate/substrate2/issues/464)) ([928b9b7](https://github.com/ucb-substrate/substrate2/commit/928b9b7c45dc334ca11d86e4564edc58bf6db6f2))
* **views:** view API for improved codegen ([#463](https://github.com/ucb-substrate/substrate2/issues/463)) ([b75328c](https://github.com/ucb-substrate/substrate2/commit/b75328c9a4840ed9200a9035e28e27ac9265770f))
* **waveform:** support generic waveform datatypes ([#379](https://github.com/ucb-substrate/substrate2/issues/379)) ([93e59fd](https://github.com/ucb-substrate/substrate2/commit/93e59fd8c005e2f7f2aeece9a637dff337e4ce68))


### Bug Fixes

* **deps:** bump rust to version 1.75.0 ([#362](https://github.com/ucb-substrate/substrate2/issues/362)) ([e1e82c9](https://github.com/ucb-substrate/substrate2/commit/e1e82c94cdf6ba4426f3f73f29dca40674a7f064))
* **dspf:** add derives to dspf types ([#409](https://github.com/ucb-substrate/substrate2/issues/409)) ([81f00cd](https://github.com/ucb-substrate/substrate2/commit/81f00cde52a12fc1b96c007d556da55eafc4d0be))
* **scir:** add additional functionality for SCIR and SPICE libraries ([#337](https://github.com/ucb-substrate/substrate2/issues/337)) ([e49f075](https://github.com/ucb-substrate/substrate2/commit/e49f07529273c38cc8ec9ae1a5020ae48fb2a202))
* **simulation:** add missing SPICE functionality and update Sky 130 PDK ([#336](https://github.com/ucb-substrate/substrate2/issues/336)) ([f802be5](https://github.com/ucb-substrate/substrate2/commit/f802be5bf0361c38b415d976dbb0f2c984a2e304))
* **spectre:** escape ports in subckt declarations ([#441](https://github.com/ucb-substrate/substrate2/issues/441)) ([3eae4ad](https://github.com/ucb-substrate/substrate2/commit/3eae4adac5b03a326724d16bee722df6c4ec7cf2))
* **spectre:** make monte carlo return vec of analysis outputs ([#388](https://github.com/ucb-substrate/substrate2/issues/388)) ([01c382d](https://github.com/ucb-substrate/substrate2/commit/01c382d908939327bd9c1344be9d928524cba021))
* **spectre:** use default number of threads ([#414](https://github.com/ucb-substrate/substrate2/issues/414)) ([748c9e4](https://github.com/ucb-substrate/substrate2/commit/748c9e42c4a922a6f858d44291fafdceb1c1e11d))
* **waveform:** fix spectre PWL waveform netlisting ([#380](https://github.com/ucb-substrate/substrate2/issues/380)) ([a47d55c](https://github.com/ucb-substrate/substrate2/commit/a47d55cca56d2359a3f0522a2c9ed8205bbb49e3))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * cache bumped from 0.5.0 to 0.6.0
    * scir bumped from 0.7.0 to 0.8.0
    * substrate bumped from 0.8.1 to 0.9.0
    * spice bumped from 0.7.1 to 0.8.0
    * type_dispatch bumped from 0.3.0 to 0.4.0

## [0.9.1](https://github.com/ucb-substrate/substrate2/compare/spectre-v0.9.0...spectre-v0.9.1) (2023-11-26)


### Bug Fixes

* **simulation:** standardize ngspice and spectre transient data formats ([#327](https://github.com/ucb-substrate/substrate2/issues/327)) ([0aa42d6](https://github.com/ucb-substrate/substrate2/commit/0aa42d6000d28a8aecb655e06330f4545e155b9b))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.8.0 to 0.8.1
    * spice bumped from 0.7.0 to 0.7.1

## [0.9.0](https://github.com/ucb-substrate/substrate2/compare/spectre-v0.8.0...spectre-v0.9.0) (2023-11-25)


### Features

* **simulation:** improve simulation APIs ([#320](https://github.com/ucb-substrate/substrate2/issues/320)) ([4ed59a1](https://github.com/ucb-substrate/substrate2/commit/4ed59a1283f9546e8336cc96015bd87c55682777))
* **stdcells:** add standard cell support to Sky130 PDK ([#323](https://github.com/ucb-substrate/substrate2/issues/323)) ([0b2048e](https://github.com/ucb-substrate/substrate2/commit/0b2048ed44d89c5de87380cac48a4bbff2b4c20a))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * cache bumped from 0.4.0 to 0.5.0
    * substrate bumped from 0.7.1 to 0.8.0
    * spice bumped from 0.6.0 to 0.7.0

## [0.8.0](https://github.com/ucb-substrate/substrate2/compare/spectre-v0.7.0...spectre-v0.8.0) (2023-11-04)


### Features

* **spice:** refactor netlisting and fix voltage source netlist ([#316](https://github.com/ucb-substrate/substrate2/issues/316)) ([7a3df69](https://github.com/ucb-substrate/substrate2/commit/7a3df695cf9b38c837ff86d5a5da2417c4db7aa2))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * scir bumped from 0.6.0 to 0.7.0
    * substrate bumped from 0.7.0 to 0.7.1
    * spice bumped from 0.5.0 to 0.6.0

## [0.7.0](https://github.com/ucb-substrate/substrate2/compare/spectre-v0.6.1...spectre-v0.7.0) (2023-11-02)


### Features

* **ics:** spectre initial conditions ([#275](https://github.com/ucb-substrate/substrate2/issues/275)) ([ce3724e](https://github.com/ucb-substrate/substrate2/commit/ce3724e9e907f3eb3653dbf39f763865914235e3))
* **netlister:** reduce duplicate code between spectre and SPICE netlisters ([#261](https://github.com/ucb-substrate/substrate2/issues/261)) ([5ba3623](https://github.com/ucb-substrate/substrate2/commit/5ba36230e653e4dc77819c5c50b527311768cd83))
* **netlists:** support ideal 2-terminal capacitors ([#269](https://github.com/ucb-substrate/substrate2/issues/269)) ([7de9843](https://github.com/ucb-substrate/substrate2/commit/7de9843c9b629ea06518448fe26d384de4a66cdc))
* **netlists:** use consistent ordering via indexmap ([#266](https://github.com/ucb-substrate/substrate2/issues/266)) ([f275c19](https://github.com/ucb-substrate/substrate2/commit/f275c19396ed4f7d255836822ff72b808f89cde7)), closes [#265](https://github.com/ucb-substrate/substrate2/issues/265)
* **ngspice:** create ngspice simulator ([#274](https://github.com/ucb-substrate/substrate2/issues/274)) ([0205300](https://github.com/ucb-substrate/substrate2/commit/02053006bc26d0b3d9e1d380def89836d7921857))
* **parser:** use nutmeg format for spectre output ([#289](https://github.com/ucb-substrate/substrate2/issues/289)) ([034f58f](https://github.com/ucb-substrate/substrate2/commit/034f58f99c587c61003761971e76c26038de9b3b))
* **primitives:** revamp schematic primitives ([#291](https://github.com/ucb-substrate/substrate2/issues/291)) ([e5ba06a](https://github.com/ucb-substrate/substrate2/commit/e5ba06ab10008b72e78397ad70781caa6bc61791))
* **refactor:** rename Has_ and Has_Data ([#282](https://github.com/ucb-substrate/substrate2/issues/282)) ([2018153](https://github.com/ucb-substrate/substrate2/commit/2018153686dd7ef3df0e10874db3c656ca245026))


### Bug Fixes

* **deps:** remove opacity from substrate and deps ([#288](https://github.com/ucb-substrate/substrate2/issues/288)) ([a8c97b3](https://github.com/ucb-substrate/substrate2/commit/a8c97b30b4d075343903fa580437e9a099a745a2))
* **deps:** update rust crate rust_decimal to 1.32 ([#296](https://github.com/ucb-substrate/substrate2/issues/296)) ([a2fe877](https://github.com/ucb-substrate/substrate2/commit/a2fe877d03d3f907f348d7711a2132194ae91034))
* **deps:** update rust crate rust_decimal_macros to 1.32 ([#297](https://github.com/ucb-substrate/substrate2/issues/297)) ([5474cc8](https://github.com/ucb-substrate/substrate2/commit/5474cc8778b81c30b34fc7d146eec6e5e2532a26))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * cache bumped from 0.3.1 to 0.4.0
    * scir bumped from 0.5.0 to 0.6.0
    * nutlex bumped from 0.1.0 to 0.2.0
    * substrate bumped from 0.6.1 to 0.7.0
    * spice bumped from 0.4.0 to 0.5.0

## [0.6.0](https://github.com/substrate-labs/substrate2/compare/spectre-v0.5.0...spectre-v0.6.0) (2023-08-08)


### Features

* **macros:** refactor macro reexports ([#250](https://github.com/substrate-labs/substrate2/issues/250)) ([a332717](https://github.com/substrate-labs/substrate2/commit/a332717e549fdea50306067e1c92dc60293aed4c))
* **slices:** use `SliceOne` instead of `Slice` where possible ([#253](https://github.com/substrate-labs/substrate2/issues/253)) ([8eba8ed](https://github.com/substrate-labs/substrate2/commit/8eba8ed5aad0aa4911ae31f4521d297487256087))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * cache bumped from 0.3.0 to 0.3.1
    * scir bumped from 0.4.0 to 0.5.0
    * substrate bumped from 0.5.0 to 0.6.0

## [0.5.0](https://github.com/substrate-labs/substrate2/compare/spectre-v0.4.0...spectre-v0.5.0) (2023-08-05)


### Features

* **terminals:** add support for terminal paths ([#236](https://github.com/substrate-labs/substrate2/issues/236)) ([3fba7f6](https://github.com/substrate-labs/substrate2/commit/3fba7f6227bbf2efcaf79d849c79175e44d783a4))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * cache bumped from 0.2.1 to 0.3.0
    * scir bumped from 0.3.0 to 0.4.0
    * substrate bumped from 0.4.0 to 0.5.0

## [0.4.0](https://github.com/substrate-labs/substrate2/compare/spectre-v0.3.0...spectre-v0.4.0) (2023-08-04)


### Features

* **includes:** add support for including sections in spectre ([#205](https://github.com/substrate-labs/substrate2/issues/205)) ([8522ff4](https://github.com/substrate-labs/substrate2/commit/8522ff4241755d4c194bacb893765e608122814e))
* **io:** composable port directions and runtime connection checking ([#231](https://github.com/substrate-labs/substrate2/issues/231)) ([e1e367a](https://github.com/substrate-labs/substrate2/commit/e1e367a2b8940319cb4f804888746a094f06e161))
* **parameters:** substrate schematic primitives support parameters ([#233](https://github.com/substrate-labs/substrate2/issues/233)) ([5dabcb2](https://github.com/substrate-labs/substrate2/commit/5dabcb270cab0d259b7301d67f77de6d55261092))
* **primitives:** support parameters in SCIR primitive devices ([#232](https://github.com/substrate-labs/substrate2/issues/232)) ([a8f5b45](https://github.com/substrate-labs/substrate2/commit/a8f5b45a00b77d050f6a812c469e19da3305e064))
* **repo:** reorganize repo ([#207](https://github.com/substrate-labs/substrate2/issues/207)) ([54a6b43](https://github.com/substrate-labs/substrate2/commit/54a6b43079d283a29bc0aa9e18dc6230b56fa385))
* **save-api:** add typed API for saving arbitrary signals ([#228](https://github.com/substrate-labs/substrate2/issues/228)) ([046be02](https://github.com/substrate-labs/substrate2/commit/046be02acbedc7fa2bb4896b92ec17babd80eee5))
* **schematics:** blackboxes can reference nodes ([#208](https://github.com/substrate-labs/substrate2/issues/208)) ([d998b4a](https://github.com/substrate-labs/substrate2/commit/d998b4a133d47d0123768dfb3c27f8ee32ed9db9))
* **schematics:** rename `HasSchematic` and `HasSchematicImpl` ([#226](https://github.com/substrate-labs/substrate2/issues/226)) ([a2b9c78](https://github.com/substrate-labs/substrate2/commit/a2b9c78ea6ff56983e9a02aeafe655e92852c264))
* **spectre:** vsource uses primitives instead of being blackboxed ([5dabcb2](https://github.com/substrate-labs/substrate2/commit/5dabcb270cab0d259b7301d67f77de6d55261092))


### Bug Fixes

* **deps:** update rust crate rust_decimal to 1.31 ([#219](https://github.com/substrate-labs/substrate2/issues/219)) ([6f596d5](https://github.com/substrate-labs/substrate2/commit/6f596d5c46dc1bf045a1b8a5ef727adbc3b147cf))
* **deps:** update rust crate rust_decimal_macros to 1.31 ([#220](https://github.com/substrate-labs/substrate2/issues/220)) ([72147d3](https://github.com/substrate-labs/substrate2/commit/72147d385368e2bd302821c981dd75209aa87dcb))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * cache bumped from 0.2.0 to 0.2.1
    * scir bumped from 0.2.0 to 0.3.0
    * substrate bumped from 0.3.0 to 0.4.0

## [0.3.0](https://github.com/substrate-labs/substrate2/compare/spectre-v0.2.0...spectre-v0.3.0) (2023-07-23)


### Features

* **cache:** implement persistent caching ([#171](https://github.com/substrate-labs/substrate2/issues/171)) ([1f8ea24](https://github.com/substrate-labs/substrate2/commit/1f8ea24f805085392bfd1a2067bb8774d0fa4ae4))
* **codegen:** implement derive proc macro for layout hard macros ([#200](https://github.com/substrate-labs/substrate2/issues/200)) ([5138224](https://github.com/substrate-labs/substrate2/commit/5138224013f537e678dfb20204e964852ed40ccb))
* **executors:** executor API and local executor implementation ([#161](https://github.com/substrate-labs/substrate2/issues/161)) ([c372d51](https://github.com/substrate-labs/substrate2/commit/c372d511e1c67ad976dc86958c87e9ad13bdfde4))
* **windows:** fix issues for windows ([#197](https://github.com/substrate-labs/substrate2/issues/197)) ([008b607](https://github.com/substrate-labs/substrate2/commit/008b607b2c21c14ac3106dca6eb74d806131ef8f))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * scir bumped from 0.1.0 to 0.2.0
    * cache bumped from 0.1.0 to 0.2.0
    * substrate bumped from 0.2.0 to 0.3.0

## [0.2.0](https://github.com/substrate-labs/substrate2/compare/spectre-v0.1.1...spectre-v0.2.0) (2023-07-07)


### Features

* **reorg:** move substrate-api into substrate ([#155](https://github.com/substrate-labs/substrate2/issues/155)) ([e902a1b](https://github.com/substrate-labs/substrate2/commit/e902a1b603cca6c719770c5cd742e081bfd33e51))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * substrate bumped from 0.1.1 to 0.2.0

## 0.1.0 (2023-07-07)


### Features

* **blackboxing:** support Substrate and SCIR blackboxes ([#135](https://github.com/substrate-labs/substrate2/issues/135)) ([049a598](https://github.com/substrate-labs/substrate2/commit/049a598e2b8d11228c63f03dc878fc4c56e036a6))
* **corners:** add API for process corners ([#141](https://github.com/substrate-labs/substrate2/issues/141)) ([a61b15a](https://github.com/substrate-labs/substrate2/commit/a61b15a80851a6393aaa9da2db41e01a34f0ce5b))
* **inverter-design:** initial inverter design logic ([#144](https://github.com/substrate-labs/substrate2/issues/144)) ([911a376](https://github.com/substrate-labs/substrate2/commit/911a3768bf730d7fd134310d500560873f008783))
* **netlisting:** implement basic Spectre netlister ([#130](https://github.com/substrate-labs/substrate2/issues/130)) ([506837f](https://github.com/substrate-labs/substrate2/commit/506837fca0f1964ebc622df970575b321456ab68))
* **scir:** uniquify names when exporting to SCIR ([#148](https://github.com/substrate-labs/substrate2/issues/148)) ([29c2f72](https://github.com/substrate-labs/substrate2/commit/29c2f729f5a205b144053b61c0d8c0ca2446071b))
* **simulation:** access nested nodes without strings in simulation ([#139](https://github.com/substrate-labs/substrate2/issues/139)) ([ed7989c](https://github.com/substrate-labs/substrate2/commit/ed7989cfb190528163a1722ae5fe3383ec3c4310))
* **simulation:** simplify SCIR paths for data access ([#143](https://github.com/substrate-labs/substrate2/issues/143)) ([d42e6f9](https://github.com/substrate-labs/substrate2/commit/d42e6f9b1d4236a9024d4a4b839319749033b8d3))
* **simulation:** support transient simulation in spectre ([#133](https://github.com/substrate-labs/substrate2/issues/133)) ([4605862](https://github.com/substrate-labs/substrate2/commit/460586252e3695ae32b0ab8d83b90023125d1a33))
* **simulation:** testbench schematic components ([#136](https://github.com/substrate-labs/substrate2/issues/136)) ([97e6b0f](https://github.com/substrate-labs/substrate2/commit/97e6b0ffd5ea7abd2a547952d5c963745854ed75))
* **tests:** add SCIR and SPICE netlister blackbox tests ([049a598](https://github.com/substrate-labs/substrate2/commit/049a598e2b8d11228c63f03dc878fc4c56e036a6))


### Bug Fixes

* **netlisting:** update release manifests ([506837f](https://github.com/substrate-labs/substrate2/commit/506837fca0f1964ebc622df970575b321456ab68))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * scir bumped from 0.0.0 to 0.1.0
    * substrate bumped from 0.0.0 to 0.1.0
    * opacity bumped from 0.0.0 to 0.1.0
