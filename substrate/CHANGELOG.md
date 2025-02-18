# Changelog

* The following workspace dependencies were updated
  * dependencies
    * codegen bumped from 0.1.0 to 0.1.1
    * substrate_api bumped from 0.1.0 to 0.1.1

* The following workspace dependencies were updated
  * dependencies
    * examples bumped from 0.4.0 to 0.4.1
    * codegen bumped from 0.7.0 to 0.7.1
    * scir bumped from 0.6.0 to 0.7.0
  * dev-dependencies
    * sky130pdk bumped from <=0.7.0 to <=0.7.1
    * spectre bumped from <=0.7.0 to <=0.8.0
    * spice bumped from <=0.5.0 to <=0.6.0

* The following workspace dependencies were updated
  * dependencies
    * examples bumped from 0.5.0 to 0.5.1
    * codegen bumped from 0.8.0 to 0.8.1
  * dev-dependencies
    * sky130pdk bumped from <=0.8.0 to <=0.8.1
    * spectre bumped from <=0.9.0 to <=0.9.1
    * spice bumped from <=0.7.0 to <=0.7.1

* The following workspace dependencies were updated
  * dependencies
    * codegen bumped from 0.10.1 to 0.10.2

## [0.10.3](https://github.com/ucb-substrate/substrate2/compare/substrate-v0.10.2...substrate-v0.10.3) (2025-02-18)


### Features

* **stdcells:** implement layout for sky130 stdcells ([#586](https://github.com/ucb-substrate/substrate2/issues/586)) ([6e438ec](https://github.com/ucb-substrate/substrate2/commit/6e438ecde6b092231b4f9b6f17e3004663c17f74))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * codegen bumped from 0.10.2 to 0.10.3
    * layir bumped from 0.2.1 to 0.2.2
    * geometry bumped from 0.7.1 to 0.7.2
    * gds bumped from 0.4.1 to 0.4.2
    * gdsconv bumped from 0.2.1 to 0.2.2
  * build-dependencies
    * examples bumped from 0.2.0 to 0.2.1

## [0.10.1](https://github.com/ucb-substrate/substrate2/compare/substrate-v0.10.0...substrate-v0.10.1) (2025-01-24)


### Dependencies

* update dependencies ([0b87032](https://github.com/ucb-substrate/substrate2/commit/0b8703276631fbb19a958453394c981d6b092441))
* update dependencies ([#538](https://github.com/ucb-substrate/substrate2/issues/538)) ([19438d6](https://github.com/ucb-substrate/substrate2/commit/19438d65ac7078a2a971b4147420364ca0717763))
* update deps, GH actions ([#551](https://github.com/ucb-substrate/substrate2/issues/551)) ([357e82a](https://github.com/ucb-substrate/substrate2/commit/357e82ae0a01317d3ad5afb33b5290d3ac10cd7a))
* The following workspace dependencies were updated
  * dependencies
    * config bumped from 0.4.0 to 0.4.1
    * cache bumped from 0.7.0 to 0.7.1
    * codegen bumped from 0.10.0 to 0.10.1
    * layir bumped from 0.2.0 to 0.2.1
    * geometry bumped from 0.7.0 to 0.7.1
    * gds bumped from 0.4.0 to 0.4.1
    * gdsconv bumped from 0.2.0 to 0.2.1
    * enumify bumped from 0.2.0 to 0.2.1
    * scir bumped from 0.9.0 to 0.9.1
    * type_dispatch bumped from 0.5.0 to 0.5.1

## [0.10.0](https://github.com/ucb-substrate/substrate2/compare/substrate-v0.9.0...substrate-v0.10.0) (2025-01-23)


### Features

* **ac:** implement Save for Substrate types in Spectre AC sim, fix lints ([#471](https://github.com/ucb-substrate/substrate2/issues/471)) ([9825520](https://github.com/ucb-substrate/substrate2/commit/98255207569cc00bd9ddc35419c2df1e48f1999c))
* **atoll:** Hierarchical ATOLL and configurable via spacing ([#374](https://github.com/ucb-substrate/substrate2/issues/374)) ([542b9a9](https://github.com/ucb-substrate/substrate2/commit/542b9a956d5c993908e33d3e707fc6bdb97d2c84))
* **atoll:** implement first cut ATOLL implementation ([#357](https://github.com/ucb-substrate/substrate2/issues/357)) ([372b927](https://github.com/ucb-substrate/substrate2/commit/372b9275c9d9c5cd58603f5a462a5e4b66b64cf7))
* **blackboxing:** support Substrate and SCIR blackboxes ([#135](https://github.com/ucb-substrate/substrate2/issues/135)) ([049a598](https://github.com/ucb-substrate/substrate2/commit/049a598e2b8d11228c63f03dc878fc4c56e036a6))
* **cache-config:** allow configuration of cache via config files ([#192](https://github.com/ucb-substrate/substrate2/issues/192)) ([0461402](https://github.com/ucb-substrate/substrate2/commit/0461402edfc1ec0886bbb25cf5471ee8480754fc))
* **cache:** implement persistent caching ([#171](https://github.com/ucb-substrate/substrate2/issues/171)) ([1f8ea24](https://github.com/ucb-substrate/substrate2/commit/1f8ea24f805085392bfd1a2067bb8774d0fa4ae4))
* **cadence:** add support for Pegasus and Quantus ([#462](https://github.com/ucb-substrate/substrate2/issues/462)) ([953e4cb](https://github.com/ucb-substrate/substrate2/commit/953e4cb761c510668f65f4825f1be3914db45e3c))
* **cdl:** add CDL parser ([#419](https://github.com/ucb-substrate/substrate2/issues/419)) ([23f0dab](https://github.com/ucb-substrate/substrate2/commit/23f0dab7b7a94cbe8960371b89d15211bddf51da))
* **codegen:** codegen for layout types, example layouts ([#469](https://github.com/ucb-substrate/substrate2/issues/469)) ([255af05](https://github.com/ucb-substrate/substrate2/commit/255af05657c01fcb0b4ff1e6eb0a54244dfeca32))
* **codegen:** derive Block macro adds required trait bounds by default ([#249](https://github.com/ucb-substrate/substrate2/issues/249)) ([892bef5](https://github.com/ucb-substrate/substrate2/commit/892bef585548264e3fcdcc2e6523a2321c6c6897))
* **codegen:** derive macro for implementing FromSaved ([#243](https://github.com/ucb-substrate/substrate2/issues/243)) ([48acae0](https://github.com/ucb-substrate/substrate2/commit/48acae0fb8915c4f968223268c92077f2deda979))
* **codegen:** implement derive proc macro for layout hard macros ([#200](https://github.com/ucb-substrate/substrate2/issues/200)) ([5138224](https://github.com/ucb-substrate/substrate2/commit/5138224013f537e678dfb20204e964852ed40ccb))
* **codegen:** insert appropriate bounds in Io, SchematicType, LayoutType proc macros ([#251](https://github.com/ucb-substrate/substrate2/issues/251)) ([33dcc79](https://github.com/ucb-substrate/substrate2/commit/33dcc797fdbeb21ad046093e655acf965fd99321))
* **corners:** require specifying corner by default ([#221](https://github.com/ucb-substrate/substrate2/issues/221)) ([4c2c3e4](https://github.com/ucb-substrate/substrate2/commit/4c2c3e4a3cd8b7e68921baf3af8b87f1da048936))
* **def:** utilities for exporting def orientations ([#434](https://github.com/ucb-substrate/substrate2/issues/434)) ([43a2b29](https://github.com/ucb-substrate/substrate2/commit/43a2b2906231cd46f08e2c4aface260d34abac62))
* **docs:** add code examples to documentation ([#65](https://github.com/ucb-substrate/substrate2/issues/65)) ([bfafd05](https://github.com/ucb-substrate/substrate2/commit/bfafd050c1b68d2e9e29e760ca3ff939e26aaeca))
* **docs:** fix user docs and update dev docs ([#480](https://github.com/ucb-substrate/substrate2/issues/480)) ([f727a1e](https://github.com/ucb-substrate/substrate2/commit/f727a1e7bd2a795ace1c51c3d6e02f3673d07a29))
* **docs:** reorganize docs and add code snippets ([#216](https://github.com/ucb-substrate/substrate2/issues/216)) ([d7c457d](https://github.com/ucb-substrate/substrate2/commit/d7c457d4e5c1d4846549a0e6df958243042285db))
* **docs:** update tutorials and revamp documentation website ([#315](https://github.com/ucb-substrate/substrate2/issues/315)) ([49bdf7f](https://github.com/ucb-substrate/substrate2/commit/49bdf7ff61e2fdbf19022697d518ad7fbafb465f))
* **docs:** versioned documentation between HEAD and release ([#470](https://github.com/ucb-substrate/substrate2/issues/470)) ([968182b](https://github.com/ucb-substrate/substrate2/commit/968182bf8f8d8b4cf923c0fd66f1ca1b32b12b16))
* **dspf:** propagate nested nodes from DSPF instances ([#407](https://github.com/ucb-substrate/substrate2/issues/407)) ([8455bd2](https://github.com/ucb-substrate/substrate2/commit/8455bd2a523bb872dc1ce3fc0e89a185108dca3c))
* **errors:** add error message for unconnected scir bindings ([#365](https://github.com/ucb-substrate/substrate2/issues/365)) ([acb25d5](https://github.com/ucb-substrate/substrate2/commit/acb25d5bd555d144e1edc7d3ef5009bf3d4c8e2a))
* **executors:** executor API and local executor implementation ([#161](https://github.com/ucb-substrate/substrate2/issues/161)) ([c372d51](https://github.com/ucb-substrate/substrate2/commit/c372d511e1c67ad976dc86958c87e9ad13bdfde4))
* **executors:** LSF (bsub) executor implementation ([#162](https://github.com/ucb-substrate/substrate2/issues/162)) ([ff8404a](https://github.com/ucb-substrate/substrate2/commit/ff8404abf75e6d6a4c82109adde0bcac48b6f33f))
* **gds-import:** implement GDS to RawCell importer ([#196](https://github.com/ucb-substrate/substrate2/issues/196)) ([fc37eeb](https://github.com/ucb-substrate/substrate2/commit/fc37eeb6bac10779491b98bcadcc0eeaeb7d8ec5))
* **gds:** add support for 1D GDS paths ([#422](https://github.com/ucb-substrate/substrate2/issues/422)) ([2034f8e](https://github.com/ucb-substrate/substrate2/commit/2034f8e75d51feecbe669d95191ec0bf05de60bf))
* **gds:** add support for square endcaps ([#438](https://github.com/ucb-substrate/substrate2/issues/438)) ([662a7dd](https://github.com/ucb-substrate/substrate2/commit/662a7dd5c34b6aca8b40fb29ac5f3bc59a65d56e))
* **gds:** gds reexport test ([#199](https://github.com/ucb-substrate/substrate2/issues/199)) ([93d3cd5](https://github.com/ucb-substrate/substrate2/commit/93d3cd555c1cb4a76a8845f4401e98d327b5d674))
* **geometry:** implemented contains for polygon ([#292](https://github.com/ucb-substrate/substrate2/issues/292)) ([708053a](https://github.com/ucb-substrate/substrate2/commit/708053adfb9f3783fc03895ede7348ace51730f0))
* **grid:** add ATOLL LCM routing grid and layer stack definition ([#338](https://github.com/ucb-substrate/substrate2/issues/338)) ([1e1ad90](https://github.com/ucb-substrate/substrate2/commit/1e1ad90d02b50dd0dd15516c306971241bf30b7c))
* **ics:** spectre initial conditions ([#275](https://github.com/ucb-substrate/substrate2/issues/275)) ([ce3724e](https://github.com/ucb-substrate/substrate2/commit/ce3724e9e907f3eb3653dbf39f763865914235e3))
* **impl-dispatch:** remove impl dispatch in favor of trait bounds ([#283](https://github.com/ucb-substrate/substrate2/issues/283)) ([d954115](https://github.com/ucb-substrate/substrate2/commit/d9541152db52aebde928e41c0d800453e906d62b))
* **io:** add diff pair io ([#344](https://github.com/ucb-substrate/substrate2/issues/344)) ([556d2ef](https://github.com/ucb-substrate/substrate2/commit/556d2ef202b6b6b8469d5a92bd3d0632b41234e9))
* **io:** composable port directions and runtime connection checking ([#231](https://github.com/ucb-substrate/substrate2/issues/231)) ([e1e367a](https://github.com/ucb-substrate/substrate2/commit/e1e367a2b8940319cb4f804888746a094f06e161))
* **ios:** panic when shorting IOs ([#234](https://github.com/ucb-substrate/substrate2/issues/234)) ([62ff08c](https://github.com/ucb-substrate/substrate2/commit/62ff08cfce531a4a7446813868f9c40e15c1c120))
* **layout-api:** initial implementation of layout API ([#61](https://github.com/ucb-substrate/substrate2/issues/61)) ([c4cdac7](https://github.com/ucb-substrate/substrate2/commit/c4cdac728fd4d4ef5defb97b3c1e1660ee78d672))
* **layout:** add `Bbox` implementation for `PortGeometry` ([#382](https://github.com/ucb-substrate/substrate2/issues/382)) ([e295119](https://github.com/ucb-substrate/substrate2/commit/e295119357318b1e0398bf57393b1a7405178ce6))
* **layout:** import LayIR cells into Substrate ([#460](https://github.com/ucb-substrate/substrate2/issues/460)) ([d623e4c](https://github.com/ucb-substrate/substrate2/commit/d623e4ccc5a9b555b49e59ae2f1d529d6c02299e))
* **layout:** rename `HasLayout` and `HasLayoutImpl` ([#227](https://github.com/ucb-substrate/substrate2/issues/227)) ([2cf1f7d](https://github.com/ucb-substrate/substrate2/commit/2cf1f7d435549df26ff15370e7324e9df76e0e4f))
* **layout:** simplified layout API, LayIR integration ([#459](https://github.com/ucb-substrate/substrate2/issues/459)) ([183d347](https://github.com/ucb-substrate/substrate2/commit/183d347c19e6fe98cf870be4716e7249f23bd423))
* **layouts:** support exporting layouts with multiple top cells ([#425](https://github.com/ucb-substrate/substrate2/issues/425)) ([991e467](https://github.com/ucb-substrate/substrate2/commit/991e4676d81d23c4e618991a5cadbb71e8df7c8e))
* **lut:** add basic 1D and 2D lookup tables ([#396](https://github.com/ucb-substrate/substrate2/issues/396)) ([b6c945a](https://github.com/ucb-substrate/substrate2/commit/b6c945a6e595f3df53de788da9967cb5e07be622))
* **macros:** refactor derive NestedData, start organizing tests ([#477](https://github.com/ucb-substrate/substrate2/issues/477)) ([aca48ef](https://github.com/ucb-substrate/substrate2/commit/aca48ef7a49c959e35ec4614345a55e667ff5146))
* **macros:** refactor macro reexports ([#250](https://github.com/ucb-substrate/substrate2/issues/250)) ([a332717](https://github.com/ucb-substrate/substrate2/commit/a332717e549fdea50306067e1c92dc60293aed4c))
* **montecarlo:** add Monte Carlo simulation support to Spectre plugin ([#347](https://github.com/ucb-substrate/substrate2/issues/347)) ([cc9dfe4](https://github.com/ucb-substrate/substrate2/commit/cc9dfe42db5be1a8aaeaf3fb81992a0ad7251ef8))
* **mos:** add sky130pdk transistor blocks ([#126](https://github.com/ucb-substrate/substrate2/issues/126)) ([3e9ee79](https://github.com/ucb-substrate/substrate2/commit/3e9ee7935e030ca3e5c4d56f19ccafc27445a6f0))
* **mos:** add standard 4-terminal MosIo ([8678716](https://github.com/ucb-substrate/substrate2/commit/86787160c49a1ac7c011d08ce1b9d7851bdfa0d8))
* **mos:** layout for sky130 1.8V nmos/pmos, fix geometry macros ([#478](https://github.com/ucb-substrate/substrate2/issues/478)) ([55f17b7](https://github.com/ucb-substrate/substrate2/commit/55f17b72ab90e12efb57d97fdad6b4e5373c30e2))
* **netlister:** reduce duplicate code between spectre and SPICE netlisters ([#261](https://github.com/ucb-substrate/substrate2/issues/261)) ([5ba3623](https://github.com/ucb-substrate/substrate2/commit/5ba36230e653e4dc77819c5c50b527311768cd83))
* **netlists:** consistent Spectre/Spice netlist API ([#349](https://github.com/ucb-substrate/substrate2/issues/349)) ([2f9fabf](https://github.com/ucb-substrate/substrate2/commit/2f9fabf336fa1048d759e78834979ef892fc0bcf))
* **netlists:** use consistent ordering via indexmap ([#266](https://github.com/ucb-substrate/substrate2/issues/266)) ([f275c19](https://github.com/ucb-substrate/substrate2/commit/f275c19396ed4f7d255836822ff72b808f89cde7)), closes [#265](https://github.com/ucb-substrate/substrate2/issues/265)
* **ngspice:** create ngspice simulator ([#274](https://github.com/ucb-substrate/substrate2/issues/274)) ([0205300](https://github.com/ucb-substrate/substrate2/commit/02053006bc26d0b3d9e1d380def89836d7921857))
* **organization:** rename substrate to substrate_api, set up codegen crate ([#67](https://github.com/ucb-substrate/substrate2/issues/67)) ([e07f099](https://github.com/ucb-substrate/substrate2/commit/e07f09949551fd08e3f58b6ffb7d9a8c67b76ae9))
* **parameters:** substrate schematic primitives support parameters ([#233](https://github.com/ucb-substrate/substrate2/issues/233)) ([5dabcb2](https://github.com/ucb-substrate/substrate2/commit/5dabcb270cab0d259b7301d67f77de6d55261092))
* **pdk:** add PDK trait and update context ([#68](https://github.com/ucb-substrate/substrate2/issues/68)) ([a8fbd14](https://github.com/ucb-substrate/substrate2/commit/a8fbd14a4b81e504c781e0656edce81853039afb))
* **pdk:** remove `PdkData` object to clean up interface ([#218](https://github.com/ucb-substrate/substrate2/issues/218)) ([1dd166a](https://github.com/ucb-substrate/substrate2/commit/1dd166a8f23e7b3c011c01b5c8527b8c5494ddea))
* **pdks:** implement `supported_pdks` macro and add examples ([#72](https://github.com/ucb-substrate/substrate2/issues/72)) ([5f4312f](https://github.com/ucb-substrate/substrate2/commit/5f4312f5220ae6023d78d8f4e585032147195a75))
* **pdks:** PDKs store the names of schematic primitives ([#185](https://github.com/ucb-substrate/substrate2/issues/185)) ([3446ba8](https://github.com/ucb-substrate/substrate2/commit/3446ba869f564f844b39ee524b52106954a293c5))
* **pex:** magic-netgen pex mapping, reorganize pex tests ([#467](https://github.com/ucb-substrate/substrate2/issues/467)) ([e32802b](https://github.com/ucb-substrate/substrate2/commit/e32802bfc567f3dea50cc86b11576f7d6863fac2))
* **polygon:** polygon implemented in geometry ([#263](https://github.com/ucb-substrate/substrate2/issues/263)) ([4508570](https://github.com/ucb-substrate/substrate2/commit/45085706a30a12f4af6c5e3f642ca55b4c32dd24))
* **primitives:** add 2-terminal capacitor primitive ([#262](https://github.com/ucb-substrate/substrate2/issues/262)) ([bc622b9](https://github.com/ucb-substrate/substrate2/commit/bc622b936a77719dbf92f76fdc3cbfbae61e9021))
* **primitives:** add built-in resistor and capacitor schematic blocks ([bc622b9](https://github.com/ucb-substrate/substrate2/commit/bc622b936a77719dbf92f76fdc3cbfbae61e9021))
* **primitives:** revamp schematic primitives ([#291](https://github.com/ucb-substrate/substrate2/issues/291)) ([e5ba06a](https://github.com/ucb-substrate/substrate2/commit/e5ba06ab10008b72e78397ad70781caa6bc61791))
* **primitives:** support parameters in SCIR primitive devices ([#232](https://github.com/ucb-substrate/substrate2/issues/232)) ([a8f5b45](https://github.com/ucb-substrate/substrate2/commit/a8f5b45a00b77d050f6a812c469e19da3305e064))
* **proc-macros:** allow missing docs on generated structs ([8678716](https://github.com/ucb-substrate/substrate2/commit/86787160c49a1ac7c011d08ce1b9d7851bdfa0d8))
* **proc-macros:** codegen for schematic hard macros ([#191](https://github.com/ucb-substrate/substrate2/issues/191)) ([50240b1](https://github.com/ucb-substrate/substrate2/commit/50240b167876873c4133315d35298b44e8eeac51))
* **proc-macros:** macros respect field and struct visibilities ([8678716](https://github.com/ucb-substrate/substrate2/commit/86787160c49a1ac7c011d08ce1b9d7851bdfa0d8))
* **proc-macros:** proc macros find substrate crate location ([#125](https://github.com/ucb-substrate/substrate2/issues/125)) ([8678716](https://github.com/ucb-substrate/substrate2/commit/86787160c49a1ac7c011d08ce1b9d7851bdfa0d8))
* **refactor:** rename Has_ and Has_Data ([#282](https://github.com/ucb-substrate/substrate2/issues/282)) ([2018153](https://github.com/ucb-substrate/substrate2/commit/2018153686dd7ef3df0e10874db3c656ca245026))
* **refactor:** significantly refactor IO APIs ([#348](https://github.com/ucb-substrate/substrate2/issues/348)) ([c85d043](https://github.com/ucb-substrate/substrate2/commit/c85d04334a0ba1740f9990b91fb55ab1f2ef77c5))
* **remote-cache:** add initial implementation of remote-cache ([#166](https://github.com/ucb-substrate/substrate2/issues/166)) ([7d90aab](https://github.com/ucb-substrate/substrate2/commit/7d90aab47c282cf90e814ffce357a1e694c0c357))
* **reorg:** move substrate-api into substrate ([#155](https://github.com/ucb-substrate/substrate2/issues/155)) ([e902a1b](https://github.com/ucb-substrate/substrate2/commit/e902a1b603cca6c719770c5cd742e081bfd33e51))
* **repo:** reorganize repo ([#207](https://github.com/ucb-substrate/substrate2/issues/207)) ([54a6b43](https://github.com/ucb-substrate/substrate2/commit/54a6b43079d283a29bc0aa9e18dc6230b56fa385))
* **save-api:** add typed API for saving arbitrary signals ([#228](https://github.com/ucb-substrate/substrate2/issues/228)) ([046be02](https://github.com/ucb-substrate/substrate2/commit/046be02acbedc7fa2bb4896b92ec17babd80eee5))
* **schematic:** associated type schema and bundle primitives ([#455](https://github.com/ucb-substrate/substrate2/issues/455)) ([f5fde78](https://github.com/ucb-substrate/substrate2/commit/f5fde78824ce9ed0be494ef68d71620181bf6b48))
* **schematic:** rename bundle traits ([#458](https://github.com/ucb-substrate/substrate2/issues/458)) ([ed98443](https://github.com/ucb-substrate/substrate2/commit/ed9844318cbd7176a781fff0076d8b3385d408b5))
* **schematics:** add `instantiate_connected_named` ([#447](https://github.com/ucb-substrate/substrate2/issues/447)) ([6c31948](https://github.com/ucb-substrate/substrate2/commit/6c31948d07b682c395a7c6188f3df6de67a3177b))
* **schematics:** allow explicit instance naming ([#444](https://github.com/ucb-substrate/substrate2/issues/444)) ([163b9eb](https://github.com/ucb-substrate/substrate2/commit/163b9eb10b895d69de8898a2951d0a64155da869))
* **schematics:** blackboxes can reference nodes ([#208](https://github.com/ucb-substrate/substrate2/issues/208)) ([d998b4a](https://github.com/ucb-substrate/substrate2/commit/d998b4a133d47d0123768dfb3c27f8ee32ed9db9))
* **schematics:** expose number of elems from ArrayData ([#381](https://github.com/ucb-substrate/substrate2/issues/381)) ([3422a39](https://github.com/ucb-substrate/substrate2/commit/3422a39bcab63ee2082e7c07a48f133c180a36ac))
* **schematics:** rename `HasSchematic` and `HasSchematicImpl` ([#226](https://github.com/ucb-substrate/substrate2/issues/226)) ([a2b9c78](https://github.com/ucb-substrate/substrate2/commit/a2b9c78ea6ff56983e9a02aeafe655e92852c264))
* **schematics:** support SCIR netlist exports with multiple top cells ([#424](https://github.com/ucb-substrate/substrate2/issues/424)) ([fc40421](https://github.com/ucb-substrate/substrate2/commit/fc40421dc973fac623133a219e092bb67ef8220a))
* **schematics:** user-specified schematic hierarchy flattening ([#222](https://github.com/ucb-substrate/substrate2/issues/222)) ([251f377](https://github.com/ucb-substrate/substrate2/commit/251f37778526d2f1c08a2b3c66f72ffe273021fa))
* **scir-instances:** allow Substrate users to instantiate raw SCIR instances ([#184](https://github.com/ucb-substrate/substrate2/issues/184)) ([8fd5192](https://github.com/ucb-substrate/substrate2/commit/8fd5192fd2017ab04e9e3220612d0a132702bb2e))
* **scir:** expose port directions, update docs ([#426](https://github.com/ucb-substrate/substrate2/issues/426)) ([fd883b7](https://github.com/ucb-substrate/substrate2/commit/fd883b7ca803f7b45d4736a7b4b460e602b84704))
* **scir:** SCIR lib imports merge only the instantiated cell ([#437](https://github.com/ucb-substrate/substrate2/issues/437)) ([7a0b285](https://github.com/ucb-substrate/substrate2/commit/7a0b285446b224569d430a2764e3a4e6d30ee031))
* **sim:** allow setting temp in Spectre sims ([#401](https://github.com/ucb-substrate/substrate2/issues/401)) ([0557fce](https://github.com/ucb-substrate/substrate2/commit/0557fceb1f0da4799914b0ea4a1e0919aed97bc7))
* **simulation:** automatically generate saved data ([#457](https://github.com/ucb-substrate/substrate2/issues/457)) ([2c936d0](https://github.com/ucb-substrate/substrate2/commit/2c936d00e927b99b624f29e6450826e90f68f9bf))
* **simulation:** implement save for nested instances ([#476](https://github.com/ucb-substrate/substrate2/issues/476)) ([a47d905](https://github.com/ucb-substrate/substrate2/commit/a47d905097c6c196153b53f142ca7e1ffba5eb51))
* **simulation:** improve simulation APIs ([#320](https://github.com/ucb-substrate/substrate2/issues/320)) ([4ed59a1](https://github.com/ucb-substrate/substrate2/commit/4ed59a1283f9546e8336cc96015bd87c55682777))
* **simulation:** proc macros for implementing Supports on tuples ([#163](https://github.com/ucb-substrate/substrate2/issues/163)) ([bf77832](https://github.com/ucb-substrate/substrate2/commit/bf778329d6e9fd317bea789d093c4c7d8790f5ac))
* **simulation:** simplify SCIR paths for data access ([#143](https://github.com/ucb-substrate/substrate2/issues/143)) ([d42e6f9](https://github.com/ucb-substrate/substrate2/commit/d42e6f9b1d4236a9024d4a4b839319749033b8d3))
* **simulation:** support transient simulation in spectre ([#133](https://github.com/ucb-substrate/substrate2/issues/133)) ([4605862](https://github.com/ucb-substrate/substrate2/commit/460586252e3695ae32b0ab8d83b90023125d1a33))
* **simulation:** testbench schematic components ([#136](https://github.com/ucb-substrate/substrate2/issues/136)) ([97e6b0f](https://github.com/ucb-substrate/substrate2/commit/97e6b0ffd5ea7abd2a547952d5c963745854ed75))
* **sky130:** Fix ATOLL plugin implementation ([#376](https://github.com/ucb-substrate/substrate2/issues/376)) ([aef1ed1](https://github.com/ucb-substrate/substrate2/commit/aef1ed10e6104d55a5fdf755ae4c26955d647a42))
* **slices:** use `SliceOne` instead of `Slice` where possible ([#253](https://github.com/ucb-substrate/substrate2/issues/253)) ([8eba8ed](https://github.com/ucb-substrate/substrate2/commit/8eba8ed5aad0aa4911ae31f4521d297487256087))
* **spectre:** support AC simulation ([#390](https://github.com/ucb-substrate/substrate2/issues/390)) ([dc3584a](https://github.com/ucb-substrate/substrate2/commit/dc3584a50ff8ebed525566a86d82033cf87d7b29))
* **spectre:** vsource uses primitives instead of being blackboxed ([5dabcb2](https://github.com/ucb-substrate/substrate2/commit/5dabcb270cab0d259b7301d67f77de6d55261092))
* **stdcells:** add standard cell support to Sky130 PDK ([#323](https://github.com/ucb-substrate/substrate2/issues/323)) ([0b2048e](https://github.com/ucb-substrate/substrate2/commit/0b2048ed44d89c5de87380cac48a4bbff2b4c20a))
* **terminals:** add support for terminal paths ([#236](https://github.com/ucb-substrate/substrate2/issues/236)) ([3fba7f6](https://github.com/ucb-substrate/substrate2/commit/3fba7f6227bbf2efcaf79d849c79175e44d783a4))
* **testing:** add test for terminal path API ([#245](https://github.com/ucb-substrate/substrate2/issues/245)) ([de55691](https://github.com/ucb-substrate/substrate2/commit/de556912ba4460a26d2b89510070976b8d8afcfe))
* **tests:** add SCIR and SPICE netlister blackbox tests ([049a598](https://github.com/ucb-substrate/substrate2/commit/049a598e2b8d11228c63f03dc878fc4c56e036a6))
* **tests:** fix compilation and lint errors ([#482](https://github.com/ucb-substrate/substrate2/issues/482)) ([b55d04e](https://github.com/ucb-substrate/substrate2/commit/b55d04ecd2472f9f72b926ba5286f0d928bc2691))
* **tests:** reorganize tests and documentation ([#464](https://github.com/ucb-substrate/substrate2/issues/464)) ([928b9b7](https://github.com/ucb-substrate/substrate2/commit/928b9b7c45dc334ca11d86e4564edc58bf6db6f2))
* **tiling:** array and grid tiling API ([#201](https://github.com/ucb-substrate/substrate2/issues/201)) ([b3b7c2b](https://github.com/ucb-substrate/substrate2/commit/b3b7c2bfb7ba72198872d0f08ded3e0bc757479d))
* **tracks:** uniform and enumerated track manager ([#295](https://github.com/ucb-substrate/substrate2/issues/295)) ([ed5cceb](https://github.com/ucb-substrate/substrate2/commit/ed5cceb27bb1fa2525c88c32e766312880390dcc))
* **transform:** default to Manhattan transformations ([#452](https://github.com/ucb-substrate/substrate2/issues/452)) ([3d8a410](https://github.com/ucb-substrate/substrate2/commit/3d8a4109febb11616d550c8cd6373e8f605b2e28))
* **transform:** make transformations use integers instead of floats ([#451](https://github.com/ucb-substrate/substrate2/issues/451)) ([aa9764e](https://github.com/ucb-substrate/substrate2/commit/aa9764e8b63b0a344d5e12ad3c678849c5c8ebea))
* **tutorial:** implement sky130 inverter layout tutorial ([#481](https://github.com/ucb-substrate/substrate2/issues/481)) ([440ab0e](https://github.com/ucb-substrate/substrate2/commit/440ab0e6ac33a8396c10f09637242efa32cfca62))
* **tutorial:** PEX testbench for inverter tutorial (open and CDS PDKs) ([#484](https://github.com/ucb-substrate/substrate2/issues/484)) ([454b169](https://github.com/ucb-substrate/substrate2/commit/454b1691862850976e4ce36baa5070bd165d957e))
* **type-dispatch:** add helper crate for dispatching types ([#225](https://github.com/ucb-substrate/substrate2/issues/225)) ([13ee1aa](https://github.com/ucb-substrate/substrate2/commit/13ee1aa1b287ed0c147549003c0af815b849577b))
* **validation:** create type for unvalidated SCIR library ([#252](https://github.com/ucb-substrate/substrate2/issues/252)) ([1550a22](https://github.com/ucb-substrate/substrate2/commit/1550a22b9a1c9f7cd9717feaa45d00487cc8848e))
* **validation:** SCIR driver analysis and validation ([#239](https://github.com/ucb-substrate/substrate2/issues/239)) ([5a91448](https://github.com/ucb-substrate/substrate2/commit/5a914489294bed06be1bd34aaa1036e4357d9a52))
* **views:** view API for improved codegen ([#463](https://github.com/ucb-substrate/substrate2/issues/463)) ([b75328c](https://github.com/ucb-substrate/substrate2/commit/b75328c9a4840ed9200a9035e28e27ac9265770f))
* **waveform:** support generic waveform datatypes ([#379](https://github.com/ucb-substrate/substrate2/issues/379)) ([93e59fd](https://github.com/ucb-substrate/substrate2/commit/93e59fd8c005e2f7f2aeece9a637dff337e4ce68))


### Bug Fixes

* **atoll:** abstract/autorouter fixes and APIs ([#398](https://github.com/ucb-substrate/substrate2/issues/398)) ([4dfac76](https://github.com/ucb-substrate/substrate2/commit/4dfac76647347ca8fc0131adb7ec5b066a1685de))
* **atoll:** Use ATOLL virtual layer for abstract bounding box ([#389](https://github.com/ucb-substrate/substrate2/issues/389)) ([d1060af](https://github.com/ucb-substrate/substrate2/commit/d1060af4c116351f0e55adc341f72b12b57b631f))
* **ci:** fix broken substrate Cargo.toml and update API release docs deploy ([#518](https://github.com/ucb-substrate/substrate2/issues/518)) ([62c3e9e](https://github.com/ucb-substrate/substrate2/commit/62c3e9edb5efbd40c36daad25587aa0894ab5dd9))
* **ci:** fix doc tests for substrate crate ([#158](https://github.com/ucb-substrate/substrate2/issues/158)) ([d7e9437](https://github.com/ucb-substrate/substrate2/commit/d7e943734b1eadfe64deabb7602f5bbf41cd8806))
* **ci:** use head_ref instead of ref and fix gdsconv version ([#498](https://github.com/ucb-substrate/substrate2/issues/498)) ([bc5d66e](https://github.com/ucb-substrate/substrate2/commit/bc5d66e5aad82ea79436e2fb3ec33e960a58f7b6))
* **codegen:** update codegen to use fewer structs ([#461](https://github.com/ucb-substrate/substrate2/issues/461)) ([c371be5](https://github.com/ucb-substrate/substrate2/commit/c371be59adebb9482095284034d41a6905c431d4))
* **deps:** bump rust to version 1.75.0 ([#362](https://github.com/ucb-substrate/substrate2/issues/362)) ([e1e82c9](https://github.com/ucb-substrate/substrate2/commit/e1e82c94cdf6ba4426f3f73f29dca40674a7f064))
* **deps:** fix dependencies and documentation ([#66](https://github.com/ucb-substrate/substrate2/issues/66)) ([a60ffc6](https://github.com/ucb-substrate/substrate2/commit/a60ffc6c5501200d56a6e76db0c1c2f7ef9cd086))
* **deps:** remove opacity from substrate and deps ([#288](https://github.com/ucb-substrate/substrate2/issues/288)) ([a8c97b3](https://github.com/ucb-substrate/substrate2/commit/a8c97b30b4d075343903fa580437e9a099a745a2))
* **docs:** fix snippet publishing ([#512](https://github.com/ucb-substrate/substrate2/issues/512)) ([456f8bf](https://github.com/ucb-substrate/substrate2/commit/456f8bfe659d4fa2a05f6d56394a6171c4fd34dd))
* **docs:** remove Cargo.tomls in CI to allow publishing of API docs ([#515](https://github.com/ucb-substrate/substrate2/issues/515)) ([2d14f50](https://github.com/ucb-substrate/substrate2/commit/2d14f50add396a1d775428b273df7d8d022aea05))
* **gds:** fix `GdsArrayRef` import ([#418](https://github.com/ucb-substrate/substrate2/issues/418)) ([51bbe93](https://github.com/ucb-substrate/substrate2/commit/51bbe93982f4278b947dce4ec5d6ce3c5fd8ad85))
* **gds:** fix GDS unit checks during import ([#397](https://github.com/ucb-substrate/substrate2/issues/397)) ([c943004](https://github.com/ucb-substrate/substrate2/commit/c943004cd479abcfdde54796e71959e2cc1511e7))
* **gds:** fix user units for GDS export ([#342](https://github.com/ucb-substrate/substrate2/issues/342)) ([d7c25c0](https://github.com/ucb-substrate/substrate2/commit/d7c25c00fe1e171ddc6dacfb816d0b85e74fd761))
* **gds:** use u16 instead of u8 for GDS layerspecs ([#339](https://github.com/ucb-substrate/substrate2/issues/339)) ([4d1fce2](https://github.com/ucb-substrate/substrate2/commit/4d1fce25f9493c6975d43dba96ccaa4c0cf4a686))
* **generics:** change `Deserialize&lt;'static&gt;` bounds to `DeserializeOwned` ([#259](https://github.com/ucb-substrate/substrate2/issues/259)) ([8015063](https://github.com/ucb-substrate/substrate2/commit/80150630b094a04a75cfc5b681255b80caf4f895))
* **io:** schematic nodes should not be Default ([#378](https://github.com/ucb-substrate/substrate2/issues/378)) ([863da3c](https://github.com/ucb-substrate/substrate2/commit/863da3cd3fbd27dd0b3bca1ba67f98c77b1f89d4))
* **layout:** fix issues in GDS export and ATOLL API ([#341](https://github.com/ucb-substrate/substrate2/issues/341)) ([08930b1](https://github.com/ucb-substrate/substrate2/commit/08930b1b25d018c20758986e206dc8882df782af))
* **re-exports:** move all re-exports to substrate ([#132](https://github.com/ucb-substrate/substrate2/issues/132)) ([8b3d867](https://github.com/ucb-substrate/substrate2/commit/8b3d867c7b76a16f422a38a04f5643eb050f14e6))
* **schematic:** correctly deduplicate SCIR cell names during export ([#435](https://github.com/ucb-substrate/substrate2/issues/435)) ([48af6fc](https://github.com/ucb-substrate/substrate2/commit/48af6fcd360fe9f2e8246ed0198945bfbae72724))
* **schematics:** add derives for ConvertedNodePath ([#399](https://github.com/ucb-substrate/substrate2/issues/399)) ([d50848b](https://github.com/ucb-substrate/substrate2/commit/d50848b9fe4911d127278359109e930b177cd367))
* **schematics:** clean up SCIR export code ([#224](https://github.com/ucb-substrate/substrate2/issues/224)) ([79d6501](https://github.com/ucb-substrate/substrate2/commit/79d6501f855fc3410f63c2355596c535584e5922))
* **schematics:** fix bugs with instance naming, cell ID allocation ([#445](https://github.com/ucb-substrate/substrate2/issues/445)) ([e7da085](https://github.com/ucb-substrate/substrate2/commit/e7da08583fefe96625017d32c03fc3cdd39aa9b4))
* **scir:** remove use of opacity from SCIR ([#286](https://github.com/ucb-substrate/substrate2/issues/286)) ([5e38b28](https://github.com/ucb-substrate/substrate2/commit/5e38b288629b5f2d6d3ca372418a331b6bd98e5e))
* **sim:** add `Sky130CommercialSchema` and simplify trait bounds ([#351](https://github.com/ucb-substrate/substrate2/issues/351)) ([c95e5c0](https://github.com/ucb-substrate/substrate2/commit/c95e5c08e5fc3bf6e34e00731ab4e38e9e586c01))
* **waveform:** fix waveform `sample_at` bugs ([#442](https://github.com/ucb-substrate/substrate2/issues/442)) ([dac7b53](https://github.com/ucb-substrate/substrate2/commit/dac7b5367f6890c9917952ef56a8e72be8fe5077))
* **waveforms:** add derive implementations to `WaveformRef` ([#394](https://github.com/ucb-substrate/substrate2/issues/394)) ([fd016a5](https://github.com/ucb-substrate/substrate2/commit/fd016a58d4d0c8046150bdb7e57d4566d33975ac))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * config bumped from 0.3.0 to 0.4.0
    * snippets bumped from 0.6.0 to 0.7.0
    * cache bumped from 0.6.0 to 0.7.0
    * codegen bumped from 0.9.0 to 0.10.0
    * layir bumped from 0.1.0 to 0.2.0
    * geometry bumped from 0.6.0 to 0.7.0
    * gds bumped from 0.3.1 to 0.4.0
    * gdsconv bumped from 0.1.0 to 0.2.0
    * enumify bumped from 0.1.1 to 0.2.0
    * scir bumped from 0.8.0 to 0.9.0
    * pathtree bumped from 0.2.0 to 0.3.0
    * type_dispatch bumped from 0.4.0 to 0.5.0
    * uniquify bumped from 0.3.0 to 0.4.0
  * build-dependencies
    * snippets bumped from 0.6.0 to 0.7.0
    * examples bumped from 0.1.2 to 0.2.0

## [0.9.0](https://github.com/ucb-substrate/substrate2/compare/substrate-v0.8.1...substrate-v0.9.0) (2025-01-22)


### Features

* **ac:** implement Save for Substrate types in Spectre AC sim, fix lints ([#471](https://github.com/ucb-substrate/substrate2/issues/471)) ([9825520](https://github.com/ucb-substrate/substrate2/commit/98255207569cc00bd9ddc35419c2df1e48f1999c))
* **atoll:** Hierarchical ATOLL and configurable via spacing ([#374](https://github.com/ucb-substrate/substrate2/issues/374)) ([542b9a9](https://github.com/ucb-substrate/substrate2/commit/542b9a956d5c993908e33d3e707fc6bdb97d2c84))
* **atoll:** implement first cut ATOLL implementation ([#357](https://github.com/ucb-substrate/substrate2/issues/357)) ([372b927](https://github.com/ucb-substrate/substrate2/commit/372b9275c9d9c5cd58603f5a462a5e4b66b64cf7))
* **cadence:** add support for Pegasus and Quantus ([#462](https://github.com/ucb-substrate/substrate2/issues/462)) ([953e4cb](https://github.com/ucb-substrate/substrate2/commit/953e4cb761c510668f65f4825f1be3914db45e3c))
* **cdl:** add CDL parser ([#419](https://github.com/ucb-substrate/substrate2/issues/419)) ([23f0dab](https://github.com/ucb-substrate/substrate2/commit/23f0dab7b7a94cbe8960371b89d15211bddf51da))
* **codegen:** codegen for layout types, example layouts ([#469](https://github.com/ucb-substrate/substrate2/issues/469)) ([255af05](https://github.com/ucb-substrate/substrate2/commit/255af05657c01fcb0b4ff1e6eb0a54244dfeca32))
* **def:** utilities for exporting def orientations ([#434](https://github.com/ucb-substrate/substrate2/issues/434)) ([43a2b29](https://github.com/ucb-substrate/substrate2/commit/43a2b2906231cd46f08e2c4aface260d34abac62))
* **docs:** fix user docs and update dev docs ([#480](https://github.com/ucb-substrate/substrate2/issues/480)) ([f727a1e](https://github.com/ucb-substrate/substrate2/commit/f727a1e7bd2a795ace1c51c3d6e02f3673d07a29))
* **docs:** versioned documentation between HEAD and release ([#470](https://github.com/ucb-substrate/substrate2/issues/470)) ([968182b](https://github.com/ucb-substrate/substrate2/commit/968182bf8f8d8b4cf923c0fd66f1ca1b32b12b16))
* **dspf:** propagate nested nodes from DSPF instances ([#407](https://github.com/ucb-substrate/substrate2/issues/407)) ([8455bd2](https://github.com/ucb-substrate/substrate2/commit/8455bd2a523bb872dc1ce3fc0e89a185108dca3c))
* **errors:** add error message for unconnected scir bindings ([#365](https://github.com/ucb-substrate/substrate2/issues/365)) ([acb25d5](https://github.com/ucb-substrate/substrate2/commit/acb25d5bd555d144e1edc7d3ef5009bf3d4c8e2a))
* **gds:** add support for 1D GDS paths ([#422](https://github.com/ucb-substrate/substrate2/issues/422)) ([2034f8e](https://github.com/ucb-substrate/substrate2/commit/2034f8e75d51feecbe669d95191ec0bf05de60bf))
* **gds:** add support for square endcaps ([#438](https://github.com/ucb-substrate/substrate2/issues/438)) ([662a7dd](https://github.com/ucb-substrate/substrate2/commit/662a7dd5c34b6aca8b40fb29ac5f3bc59a65d56e))
* **grid:** add ATOLL LCM routing grid and layer stack definition ([#338](https://github.com/ucb-substrate/substrate2/issues/338)) ([1e1ad90](https://github.com/ucb-substrate/substrate2/commit/1e1ad90d02b50dd0dd15516c306971241bf30b7c))
* **io:** add diff pair io ([#344](https://github.com/ucb-substrate/substrate2/issues/344)) ([556d2ef](https://github.com/ucb-substrate/substrate2/commit/556d2ef202b6b6b8469d5a92bd3d0632b41234e9))
* **layout:** add `Bbox` implementation for `PortGeometry` ([#382](https://github.com/ucb-substrate/substrate2/issues/382)) ([e295119](https://github.com/ucb-substrate/substrate2/commit/e295119357318b1e0398bf57393b1a7405178ce6))
* **layout:** import LayIR cells into Substrate ([#460](https://github.com/ucb-substrate/substrate2/issues/460)) ([d623e4c](https://github.com/ucb-substrate/substrate2/commit/d623e4ccc5a9b555b49e59ae2f1d529d6c02299e))
* **layout:** simplified layout API, LayIR integration ([#459](https://github.com/ucb-substrate/substrate2/issues/459)) ([183d347](https://github.com/ucb-substrate/substrate2/commit/183d347c19e6fe98cf870be4716e7249f23bd423))
* **layouts:** support exporting layouts with multiple top cells ([#425](https://github.com/ucb-substrate/substrate2/issues/425)) ([991e467](https://github.com/ucb-substrate/substrate2/commit/991e4676d81d23c4e618991a5cadbb71e8df7c8e))
* **lut:** add basic 1D and 2D lookup tables ([#396](https://github.com/ucb-substrate/substrate2/issues/396)) ([b6c945a](https://github.com/ucb-substrate/substrate2/commit/b6c945a6e595f3df53de788da9967cb5e07be622))
* **macros:** refactor derive NestedData, start organizing tests ([#477](https://github.com/ucb-substrate/substrate2/issues/477)) ([aca48ef](https://github.com/ucb-substrate/substrate2/commit/aca48ef7a49c959e35ec4614345a55e667ff5146))
* **montecarlo:** add Monte Carlo simulation support to Spectre plugin ([#347](https://github.com/ucb-substrate/substrate2/issues/347)) ([cc9dfe4](https://github.com/ucb-substrate/substrate2/commit/cc9dfe42db5be1a8aaeaf3fb81992a0ad7251ef8))
* **mos:** layout for sky130 1.8V nmos/pmos, fix geometry macros ([#478](https://github.com/ucb-substrate/substrate2/issues/478)) ([55f17b7](https://github.com/ucb-substrate/substrate2/commit/55f17b72ab90e12efb57d97fdad6b4e5373c30e2))
* **netlists:** consistent Spectre/Spice netlist API ([#349](https://github.com/ucb-substrate/substrate2/issues/349)) ([2f9fabf](https://github.com/ucb-substrate/substrate2/commit/2f9fabf336fa1048d759e78834979ef892fc0bcf))
* **pex:** magic-netgen pex mapping, reorganize pex tests ([#467](https://github.com/ucb-substrate/substrate2/issues/467)) ([e32802b](https://github.com/ucb-substrate/substrate2/commit/e32802bfc567f3dea50cc86b11576f7d6863fac2))
* **refactor:** significantly refactor IO APIs ([#348](https://github.com/ucb-substrate/substrate2/issues/348)) ([c85d043](https://github.com/ucb-substrate/substrate2/commit/c85d04334a0ba1740f9990b91fb55ab1f2ef77c5))
* **schematic:** associated type schema and bundle primitives ([#455](https://github.com/ucb-substrate/substrate2/issues/455)) ([f5fde78](https://github.com/ucb-substrate/substrate2/commit/f5fde78824ce9ed0be494ef68d71620181bf6b48))
* **schematic:** rename bundle traits ([#458](https://github.com/ucb-substrate/substrate2/issues/458)) ([ed98443](https://github.com/ucb-substrate/substrate2/commit/ed9844318cbd7176a781fff0076d8b3385d408b5))
* **schematics:** add `instantiate_connected_named` ([#447](https://github.com/ucb-substrate/substrate2/issues/447)) ([6c31948](https://github.com/ucb-substrate/substrate2/commit/6c31948d07b682c395a7c6188f3df6de67a3177b))
* **schematics:** allow explicit instance naming ([#444](https://github.com/ucb-substrate/substrate2/issues/444)) ([163b9eb](https://github.com/ucb-substrate/substrate2/commit/163b9eb10b895d69de8898a2951d0a64155da869))
* **schematics:** expose number of elems from ArrayData ([#381](https://github.com/ucb-substrate/substrate2/issues/381)) ([3422a39](https://github.com/ucb-substrate/substrate2/commit/3422a39bcab63ee2082e7c07a48f133c180a36ac))
* **schematics:** support SCIR netlist exports with multiple top cells ([#424](https://github.com/ucb-substrate/substrate2/issues/424)) ([fc40421](https://github.com/ucb-substrate/substrate2/commit/fc40421dc973fac623133a219e092bb67ef8220a))
* **scir:** expose port directions, update docs ([#426](https://github.com/ucb-substrate/substrate2/issues/426)) ([fd883b7](https://github.com/ucb-substrate/substrate2/commit/fd883b7ca803f7b45d4736a7b4b460e602b84704))
* **scir:** SCIR lib imports merge only the instantiated cell ([#437](https://github.com/ucb-substrate/substrate2/issues/437)) ([7a0b285](https://github.com/ucb-substrate/substrate2/commit/7a0b285446b224569d430a2764e3a4e6d30ee031))
* **sim:** allow setting temp in Spectre sims ([#401](https://github.com/ucb-substrate/substrate2/issues/401)) ([0557fce](https://github.com/ucb-substrate/substrate2/commit/0557fceb1f0da4799914b0ea4a1e0919aed97bc7))
* **simulation:** automatically generate saved data ([#457](https://github.com/ucb-substrate/substrate2/issues/457)) ([2c936d0](https://github.com/ucb-substrate/substrate2/commit/2c936d00e927b99b624f29e6450826e90f68f9bf))
* **simulation:** implement save for nested instances ([#476](https://github.com/ucb-substrate/substrate2/issues/476)) ([a47d905](https://github.com/ucb-substrate/substrate2/commit/a47d905097c6c196153b53f142ca7e1ffba5eb51))
* **sky130:** Fix ATOLL plugin implementation ([#376](https://github.com/ucb-substrate/substrate2/issues/376)) ([aef1ed1](https://github.com/ucb-substrate/substrate2/commit/aef1ed10e6104d55a5fdf755ae4c26955d647a42))
* **spectre:** support AC simulation ([#390](https://github.com/ucb-substrate/substrate2/issues/390)) ([dc3584a](https://github.com/ucb-substrate/substrate2/commit/dc3584a50ff8ebed525566a86d82033cf87d7b29))
* **tests:** fix compilation and lint errors ([#482](https://github.com/ucb-substrate/substrate2/issues/482)) ([b55d04e](https://github.com/ucb-substrate/substrate2/commit/b55d04ecd2472f9f72b926ba5286f0d928bc2691))
* **tests:** reorganize tests and documentation ([#464](https://github.com/ucb-substrate/substrate2/issues/464)) ([928b9b7](https://github.com/ucb-substrate/substrate2/commit/928b9b7c45dc334ca11d86e4564edc58bf6db6f2))
* **transform:** default to Manhattan transformations ([#452](https://github.com/ucb-substrate/substrate2/issues/452)) ([3d8a410](https://github.com/ucb-substrate/substrate2/commit/3d8a4109febb11616d550c8cd6373e8f605b2e28))
* **transform:** make transformations use integers instead of floats ([#451](https://github.com/ucb-substrate/substrate2/issues/451)) ([aa9764e](https://github.com/ucb-substrate/substrate2/commit/aa9764e8b63b0a344d5e12ad3c678849c5c8ebea))
* **tutorial:** implement sky130 inverter layout tutorial ([#481](https://github.com/ucb-substrate/substrate2/issues/481)) ([440ab0e](https://github.com/ucb-substrate/substrate2/commit/440ab0e6ac33a8396c10f09637242efa32cfca62))
* **tutorial:** PEX testbench for inverter tutorial (open and CDS PDKs) ([#484](https://github.com/ucb-substrate/substrate2/issues/484)) ([454b169](https://github.com/ucb-substrate/substrate2/commit/454b1691862850976e4ce36baa5070bd165d957e))
* **views:** view API for improved codegen ([#463](https://github.com/ucb-substrate/substrate2/issues/463)) ([b75328c](https://github.com/ucb-substrate/substrate2/commit/b75328c9a4840ed9200a9035e28e27ac9265770f))
* **waveform:** support generic waveform datatypes ([#379](https://github.com/ucb-substrate/substrate2/issues/379)) ([93e59fd](https://github.com/ucb-substrate/substrate2/commit/93e59fd8c005e2f7f2aeece9a637dff337e4ce68))


### Bug Fixes

* **atoll:** abstract/autorouter fixes and APIs ([#398](https://github.com/ucb-substrate/substrate2/issues/398)) ([4dfac76](https://github.com/ucb-substrate/substrate2/commit/4dfac76647347ca8fc0131adb7ec5b066a1685de))
* **atoll:** Use ATOLL virtual layer for abstract bounding box ([#389](https://github.com/ucb-substrate/substrate2/issues/389)) ([d1060af](https://github.com/ucb-substrate/substrate2/commit/d1060af4c116351f0e55adc341f72b12b57b631f))
* **ci:** use head_ref instead of ref and fix gdsconv version ([#498](https://github.com/ucb-substrate/substrate2/issues/498)) ([bc5d66e](https://github.com/ucb-substrate/substrate2/commit/bc5d66e5aad82ea79436e2fb3ec33e960a58f7b6))
* **codegen:** update codegen to use fewer structs ([#461](https://github.com/ucb-substrate/substrate2/issues/461)) ([c371be5](https://github.com/ucb-substrate/substrate2/commit/c371be59adebb9482095284034d41a6905c431d4))
* **deps:** bump rust to version 1.75.0 ([#362](https://github.com/ucb-substrate/substrate2/issues/362)) ([e1e82c9](https://github.com/ucb-substrate/substrate2/commit/e1e82c94cdf6ba4426f3f73f29dca40674a7f064))
* **gds:** fix `GdsArrayRef` import ([#418](https://github.com/ucb-substrate/substrate2/issues/418)) ([51bbe93](https://github.com/ucb-substrate/substrate2/commit/51bbe93982f4278b947dce4ec5d6ce3c5fd8ad85))
* **gds:** fix GDS unit checks during import ([#397](https://github.com/ucb-substrate/substrate2/issues/397)) ([c943004](https://github.com/ucb-substrate/substrate2/commit/c943004cd479abcfdde54796e71959e2cc1511e7))
* **gds:** fix user units for GDS export ([#342](https://github.com/ucb-substrate/substrate2/issues/342)) ([d7c25c0](https://github.com/ucb-substrate/substrate2/commit/d7c25c00fe1e171ddc6dacfb816d0b85e74fd761))
* **gds:** use u16 instead of u8 for GDS layerspecs ([#339](https://github.com/ucb-substrate/substrate2/issues/339)) ([4d1fce2](https://github.com/ucb-substrate/substrate2/commit/4d1fce25f9493c6975d43dba96ccaa4c0cf4a686))
* **io:** schematic nodes should not be Default ([#378](https://github.com/ucb-substrate/substrate2/issues/378)) ([863da3c](https://github.com/ucb-substrate/substrate2/commit/863da3cd3fbd27dd0b3bca1ba67f98c77b1f89d4))
* **layout:** fix issues in GDS export and ATOLL API ([#341](https://github.com/ucb-substrate/substrate2/issues/341)) ([08930b1](https://github.com/ucb-substrate/substrate2/commit/08930b1b25d018c20758986e206dc8882df782af))
* **schematic:** correctly deduplicate SCIR cell names during export ([#435](https://github.com/ucb-substrate/substrate2/issues/435)) ([48af6fc](https://github.com/ucb-substrate/substrate2/commit/48af6fcd360fe9f2e8246ed0198945bfbae72724))
* **schematics:** add derives for ConvertedNodePath ([#399](https://github.com/ucb-substrate/substrate2/issues/399)) ([d50848b](https://github.com/ucb-substrate/substrate2/commit/d50848b9fe4911d127278359109e930b177cd367))
* **schematics:** fix bugs with instance naming, cell ID allocation ([#445](https://github.com/ucb-substrate/substrate2/issues/445)) ([e7da085](https://github.com/ucb-substrate/substrate2/commit/e7da08583fefe96625017d32c03fc3cdd39aa9b4))
* **sim:** add `Sky130CommercialSchema` and simplify trait bounds ([#351](https://github.com/ucb-substrate/substrate2/issues/351)) ([c95e5c0](https://github.com/ucb-substrate/substrate2/commit/c95e5c08e5fc3bf6e34e00731ab4e38e9e586c01))
* **waveform:** fix waveform `sample_at` bugs ([#442](https://github.com/ucb-substrate/substrate2/issues/442)) ([dac7b53](https://github.com/ucb-substrate/substrate2/commit/dac7b5367f6890c9917952ef56a8e72be8fe5077))
* **waveforms:** add derive implementations to `WaveformRef` ([#394](https://github.com/ucb-substrate/substrate2/issues/394)) ([fd016a5](https://github.com/ucb-substrate/substrate2/commit/fd016a58d4d0c8046150bdb7e57d4566d33975ac))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * config bumped from 0.2.5 to 0.3.0
    * snippets bumped from 0.5.1 to 0.6.0
    * cache bumped from 0.5.0 to 0.6.0
    * codegen bumped from 0.8.1 to 0.9.0
    * layir bumped from 0.0.0 to 0.1.0
    * geometry bumped from 0.5.0 to 0.6.0
    * gds bumped from 0.3.0 to 0.3.1
    * gdsconv bumped from 0.0.0 to 0.1.0
    * enumify bumped from 0.1.0 to 0.1.1
    * scir bumped from 0.7.0 to 0.8.0
    * type_dispatch bumped from 0.3.0 to 0.4.0
    * uniquify bumped from 0.2.0 to 0.3.0
  * build-dependencies
    * snippets bumped from 0.5.1 to 0.6.0

## [0.8.0](https://github.com/ucb-substrate/substrate2/compare/substrate-v0.7.1...substrate-v0.8.0) (2023-11-25)


### Features

* **docs:** update tutorials and revamp documentation website ([#315](https://github.com/ucb-substrate/substrate2/issues/315)) ([49bdf7f](https://github.com/ucb-substrate/substrate2/commit/49bdf7ff61e2fdbf19022697d518ad7fbafb465f))
* **simulation:** improve simulation APIs ([#320](https://github.com/ucb-substrate/substrate2/issues/320)) ([4ed59a1](https://github.com/ucb-substrate/substrate2/commit/4ed59a1283f9546e8336cc96015bd87c55682777))
* **stdcells:** add standard cell support to Sky130 PDK ([#323](https://github.com/ucb-substrate/substrate2/issues/323)) ([0b2048e](https://github.com/ucb-substrate/substrate2/commit/0b2048ed44d89c5de87380cac48a4bbff2b4c20a))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * config bumped from 0.2.4 to 0.2.5
    * examples bumped from 0.4.1 to 0.5.0
    * cache bumped from 0.4.0 to 0.5.0
    * codegen bumped from 0.7.1 to 0.8.0
  * dev-dependencies
    * sky130pdk bumped from <=0.7.1 to <=0.8.0
    * spectre bumped from <=0.8.0 to <=0.9.0
    * spice bumped from <=0.6.0 to <=0.7.0

## [0.7.0](https://github.com/ucb-substrate/substrate2/compare/substrate-v0.6.1...substrate-v0.7.0) (2023-11-02)


### Features

* **geometry:** implemented contains for polygon ([#292](https://github.com/ucb-substrate/substrate2/issues/292)) ([708053a](https://github.com/ucb-substrate/substrate2/commit/708053adfb9f3783fc03895ede7348ace51730f0))
* **ics:** spectre initial conditions ([#275](https://github.com/ucb-substrate/substrate2/issues/275)) ([ce3724e](https://github.com/ucb-substrate/substrate2/commit/ce3724e9e907f3eb3653dbf39f763865914235e3))
* **impl-dispatch:** remove impl dispatch in favor of trait bounds ([#283](https://github.com/ucb-substrate/substrate2/issues/283)) ([d954115](https://github.com/ucb-substrate/substrate2/commit/d9541152db52aebde928e41c0d800453e906d62b))
* **netlister:** reduce duplicate code between spectre and SPICE netlisters ([#261](https://github.com/ucb-substrate/substrate2/issues/261)) ([5ba3623](https://github.com/ucb-substrate/substrate2/commit/5ba36230e653e4dc77819c5c50b527311768cd83))
* **netlists:** use consistent ordering via indexmap ([#266](https://github.com/ucb-substrate/substrate2/issues/266)) ([f275c19](https://github.com/ucb-substrate/substrate2/commit/f275c19396ed4f7d255836822ff72b808f89cde7)), closes [#265](https://github.com/ucb-substrate/substrate2/issues/265)
* **ngspice:** create ngspice simulator ([#274](https://github.com/ucb-substrate/substrate2/issues/274)) ([0205300](https://github.com/ucb-substrate/substrate2/commit/02053006bc26d0b3d9e1d380def89836d7921857))
* **polygon:** polygon implemented in geometry ([#263](https://github.com/ucb-substrate/substrate2/issues/263)) ([4508570](https://github.com/ucb-substrate/substrate2/commit/45085706a30a12f4af6c5e3f642ca55b4c32dd24))
* **primitives:** add 2-terminal capacitor primitive ([#262](https://github.com/ucb-substrate/substrate2/issues/262)) ([bc622b9](https://github.com/ucb-substrate/substrate2/commit/bc622b936a77719dbf92f76fdc3cbfbae61e9021))
* **primitives:** add built-in resistor and capacitor schematic blocks ([bc622b9](https://github.com/ucb-substrate/substrate2/commit/bc622b936a77719dbf92f76fdc3cbfbae61e9021))
* **primitives:** revamp schematic primitives ([#291](https://github.com/ucb-substrate/substrate2/issues/291)) ([e5ba06a](https://github.com/ucb-substrate/substrate2/commit/e5ba06ab10008b72e78397ad70781caa6bc61791))
* **refactor:** rename Has_ and Has_Data ([#282](https://github.com/ucb-substrate/substrate2/issues/282)) ([2018153](https://github.com/ucb-substrate/substrate2/commit/2018153686dd7ef3df0e10874db3c656ca245026))
* **tracks:** uniform and enumerated track manager ([#295](https://github.com/ucb-substrate/substrate2/issues/295)) ([ed5cceb](https://github.com/ucb-substrate/substrate2/commit/ed5cceb27bb1fa2525c88c32e766312880390dcc))


### Bug Fixes

* **deps:** remove opacity from substrate and deps ([#288](https://github.com/ucb-substrate/substrate2/issues/288)) ([a8c97b3](https://github.com/ucb-substrate/substrate2/commit/a8c97b30b4d075343903fa580437e9a099a745a2))
* **scir:** remove use of opacity from SCIR ([#286](https://github.com/ucb-substrate/substrate2/issues/286)) ([5e38b28](https://github.com/ucb-substrate/substrate2/commit/5e38b288629b5f2d6d3ca372418a331b6bd98e5e))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * config bumped from 0.2.3 to 0.2.4
    * examples bumped from 0.3.1 to 0.4.0
    * cache bumped from 0.3.1 to 0.4.0
    * codegen bumped from 0.6.1 to 0.7.0
    * geometry bumped from 0.4.0 to 0.5.0
    * enumify bumped from 0.0.0 to 0.1.0
    * scir bumped from 0.5.0 to 0.6.0
  * dev-dependencies
    * sky130pdk bumped from <=0.6.1 to <=0.7.0
    * spectre bumped from <=0.6.1 to <=0.7.0
    * spice bumped from <=0.4.0 to <=0.5.0

## [0.6.1](https://github.com/substrate-labs/substrate2/compare/substrate-v0.6.0...substrate-v0.6.1) (2023-08-08)


### Bug Fixes

* **generics:** change `Deserialize&lt;'static&gt;` bounds to `DeserializeOwned` ([#259](https://github.com/substrate-labs/substrate2/issues/259)) ([8015063](https://github.com/substrate-labs/substrate2/commit/80150630b094a04a75cfc5b681255b80caf4f895))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * examples bumped from 0.3.0 to 0.3.1
    * codegen bumped from 0.6.0 to 0.6.1
  * dev-dependencies
    * sky130pdk bumped from <=0.6.0 to <=0.6.1
    * spectre bumped from <=0.6.0 to <=0.6.1

## [0.6.0](https://github.com/substrate-labs/substrate2/compare/substrate-v0.5.0...substrate-v0.6.0) (2023-08-08)


### Features

* **codegen:** derive Block macro adds required trait bounds by default ([#249](https://github.com/substrate-labs/substrate2/issues/249)) ([892bef5](https://github.com/substrate-labs/substrate2/commit/892bef585548264e3fcdcc2e6523a2321c6c6897))
* **codegen:** insert appropriate bounds in Io, SchematicType, LayoutType proc macros ([#251](https://github.com/substrate-labs/substrate2/issues/251)) ([33dcc79](https://github.com/substrate-labs/substrate2/commit/33dcc797fdbeb21ad046093e655acf965fd99321))
* **macros:** refactor macro reexports ([#250](https://github.com/substrate-labs/substrate2/issues/250)) ([a332717](https://github.com/substrate-labs/substrate2/commit/a332717e549fdea50306067e1c92dc60293aed4c))
* **slices:** use `SliceOne` instead of `Slice` where possible ([#253](https://github.com/substrate-labs/substrate2/issues/253)) ([8eba8ed](https://github.com/substrate-labs/substrate2/commit/8eba8ed5aad0aa4911ae31f4521d297487256087))
* **testing:** add test for terminal path API ([#245](https://github.com/substrate-labs/substrate2/issues/245)) ([de55691](https://github.com/substrate-labs/substrate2/commit/de556912ba4460a26d2b89510070976b8d8afcfe))
* **validation:** create type for unvalidated SCIR library ([#252](https://github.com/substrate-labs/substrate2/issues/252)) ([1550a22](https://github.com/substrate-labs/substrate2/commit/1550a22b9a1c9f7cd9717feaa45d00487cc8848e))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * config bumped from 0.2.2 to 0.2.3
    * examples bumped from 0.2.0 to 0.3.0
    * cache bumped from 0.3.0 to 0.3.1
    * codegen bumped from 0.5.0 to 0.6.0
    * geometry bumped from 0.3.0 to 0.4.0
    * scir bumped from 0.4.0 to 0.5.0
    * spice bumped from 0.3.0 to 0.4.0
    * type_dispatch bumped from 0.2.0 to 0.3.0
  * dev-dependencies
    * sky130pdk bumped from <=0.5.0 to <=0.6.0
    * spectre bumped from <=0.5.0 to <=0.6.0

## [0.5.0](https://github.com/substrate-labs/substrate2/compare/substrate-v0.4.0...substrate-v0.5.0) (2023-08-05)


### Features

* **codegen:** derive macro for implementing FromSaved ([#243](https://github.com/substrate-labs/substrate2/issues/243)) ([48acae0](https://github.com/substrate-labs/substrate2/commit/48acae0fb8915c4f968223268c92077f2deda979))
* **terminals:** add support for terminal paths ([#236](https://github.com/substrate-labs/substrate2/issues/236)) ([3fba7f6](https://github.com/substrate-labs/substrate2/commit/3fba7f6227bbf2efcaf79d849c79175e44d783a4))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * config bumped from 0.2.1 to 0.2.2
    * examples bumped from 0.1.0 to 0.2.0
    * cache bumped from 0.2.1 to 0.3.0
    * codegen bumped from 0.4.0 to 0.5.0
    * scir bumped from 0.3.0 to 0.4.0
    * pathtree bumped from 0.1.0 to 0.2.0
    * spice bumped from 0.2.0 to 0.3.0
    * type_dispatch bumped from 0.1.0 to 0.2.0
  * dev-dependencies
    * sky130pdk bumped from <=0.4.0 to <=0.5.0
    * spectre bumped from <=0.4.0 to <=0.5.0

## [0.4.0](https://github.com/substrate-labs/substrate2/compare/substrate-v0.3.0...substrate-v0.4.0) (2023-08-04)


### Features

* **corners:** require specifying corner by default ([#221](https://github.com/substrate-labs/substrate2/issues/221)) ([4c2c3e4](https://github.com/substrate-labs/substrate2/commit/4c2c3e4a3cd8b7e68921baf3af8b87f1da048936))
* **docs:** reorganize docs and add code snippets ([#216](https://github.com/substrate-labs/substrate2/issues/216)) ([d7c457d](https://github.com/substrate-labs/substrate2/commit/d7c457d4e5c1d4846549a0e6df958243042285db))
* **io:** composable port directions and runtime connection checking ([#231](https://github.com/substrate-labs/substrate2/issues/231)) ([e1e367a](https://github.com/substrate-labs/substrate2/commit/e1e367a2b8940319cb4f804888746a094f06e161))
* **ios:** panic when shorting IOs ([#234](https://github.com/substrate-labs/substrate2/issues/234)) ([62ff08c](https://github.com/substrate-labs/substrate2/commit/62ff08cfce531a4a7446813868f9c40e15c1c120))
* **layout:** rename `HasLayout` and `HasLayoutImpl` ([#227](https://github.com/substrate-labs/substrate2/issues/227)) ([2cf1f7d](https://github.com/substrate-labs/substrate2/commit/2cf1f7d435549df26ff15370e7324e9df76e0e4f))
* **parameters:** substrate schematic primitives support parameters ([#233](https://github.com/substrate-labs/substrate2/issues/233)) ([5dabcb2](https://github.com/substrate-labs/substrate2/commit/5dabcb270cab0d259b7301d67f77de6d55261092))
* **pdk:** remove `PdkData` object to clean up interface ([#218](https://github.com/substrate-labs/substrate2/issues/218)) ([1dd166a](https://github.com/substrate-labs/substrate2/commit/1dd166a8f23e7b3c011c01b5c8527b8c5494ddea))
* **primitives:** support parameters in SCIR primitive devices ([#232](https://github.com/substrate-labs/substrate2/issues/232)) ([a8f5b45](https://github.com/substrate-labs/substrate2/commit/a8f5b45a00b77d050f6a812c469e19da3305e064))
* **repo:** reorganize repo ([#207](https://github.com/substrate-labs/substrate2/issues/207)) ([54a6b43](https://github.com/substrate-labs/substrate2/commit/54a6b43079d283a29bc0aa9e18dc6230b56fa385))
* **save-api:** add typed API for saving arbitrary signals ([#228](https://github.com/substrate-labs/substrate2/issues/228)) ([046be02](https://github.com/substrate-labs/substrate2/commit/046be02acbedc7fa2bb4896b92ec17babd80eee5))
* **schematics:** blackboxes can reference nodes ([#208](https://github.com/substrate-labs/substrate2/issues/208)) ([d998b4a](https://github.com/substrate-labs/substrate2/commit/d998b4a133d47d0123768dfb3c27f8ee32ed9db9))
* **schematics:** rename `HasSchematic` and `HasSchematicImpl` ([#226](https://github.com/substrate-labs/substrate2/issues/226)) ([a2b9c78](https://github.com/substrate-labs/substrate2/commit/a2b9c78ea6ff56983e9a02aeafe655e92852c264))
* **schematics:** user-specified schematic hierarchy flattening ([#222](https://github.com/substrate-labs/substrate2/issues/222)) ([251f377](https://github.com/substrate-labs/substrate2/commit/251f37778526d2f1c08a2b3c66f72ffe273021fa))
* **spectre:** vsource uses primitives instead of being blackboxed ([5dabcb2](https://github.com/substrate-labs/substrate2/commit/5dabcb270cab0d259b7301d67f77de6d55261092))
* **type-dispatch:** add helper crate for dispatching types ([#225](https://github.com/substrate-labs/substrate2/issues/225)) ([13ee1aa](https://github.com/substrate-labs/substrate2/commit/13ee1aa1b287ed0c147549003c0af815b849577b))
* **validation:** SCIR driver analysis and validation ([#239](https://github.com/substrate-labs/substrate2/issues/239)) ([5a91448](https://github.com/substrate-labs/substrate2/commit/5a914489294bed06be1bd34aaa1036e4357d9a52))


### Bug Fixes

* **schematics:** clean up SCIR export code ([#224](https://github.com/substrate-labs/substrate2/issues/224)) ([79d6501](https://github.com/substrate-labs/substrate2/commit/79d6501f855fc3410f63c2355596c535584e5922))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * config bumped from 0.2.0 to 0.2.1
    * examples bumped from 0.0.0 to 0.1.0
    * cache bumped from 0.2.0 to 0.2.1
    * codegen bumped from 0.3.0 to 0.4.0
    * gds bumped from 0.2.0 to 0.3.0
    * scir bumped from 0.2.0 to 0.3.0
    * spice bumped from 0.1.0 to 0.2.0
    * type_dispatch bumped from 0.0.0 to 0.1.0
  * dev-dependencies
    * sky130pdk bumped from <=0.3.0 to <=0.4.0

## [0.3.0](https://github.com/substrate-labs/substrate2/compare/substrate-v0.2.0...substrate-v0.3.0) (2023-07-23)


### Features

* **cache-config:** allow configuration of cache via config files ([#192](https://github.com/substrate-labs/substrate2/issues/192)) ([0461402](https://github.com/substrate-labs/substrate2/commit/0461402edfc1ec0886bbb25cf5471ee8480754fc))
* **cache:** implement persistent caching ([#171](https://github.com/substrate-labs/substrate2/issues/171)) ([1f8ea24](https://github.com/substrate-labs/substrate2/commit/1f8ea24f805085392bfd1a2067bb8774d0fa4ae4))
* **codegen:** implement derive proc macro for layout hard macros ([#200](https://github.com/substrate-labs/substrate2/issues/200)) ([5138224](https://github.com/substrate-labs/substrate2/commit/5138224013f537e678dfb20204e964852ed40ccb))
* **executors:** executor API and local executor implementation ([#161](https://github.com/substrate-labs/substrate2/issues/161)) ([c372d51](https://github.com/substrate-labs/substrate2/commit/c372d511e1c67ad976dc86958c87e9ad13bdfde4))
* **executors:** LSF (bsub) executor implementation ([#162](https://github.com/substrate-labs/substrate2/issues/162)) ([ff8404a](https://github.com/substrate-labs/substrate2/commit/ff8404abf75e6d6a4c82109adde0bcac48b6f33f))
* **gds-import:** implement GDS to RawCell importer ([#196](https://github.com/substrate-labs/substrate2/issues/196)) ([fc37eeb](https://github.com/substrate-labs/substrate2/commit/fc37eeb6bac10779491b98bcadcc0eeaeb7d8ec5))
* **gds:** gds reexport test ([#199](https://github.com/substrate-labs/substrate2/issues/199)) ([93d3cd5](https://github.com/substrate-labs/substrate2/commit/93d3cd555c1cb4a76a8845f4401e98d327b5d674))
* **pdks:** PDKs store the names of schematic primitives ([#185](https://github.com/substrate-labs/substrate2/issues/185)) ([3446ba8](https://github.com/substrate-labs/substrate2/commit/3446ba869f564f844b39ee524b52106954a293c5))
* **proc-macros:** codegen for schematic hard macros ([#191](https://github.com/substrate-labs/substrate2/issues/191)) ([50240b1](https://github.com/substrate-labs/substrate2/commit/50240b167876873c4133315d35298b44e8eeac51))
* **remote-cache:** add initial implementation of remote-cache ([#166](https://github.com/substrate-labs/substrate2/issues/166)) ([7d90aab](https://github.com/substrate-labs/substrate2/commit/7d90aab47c282cf90e814ffce357a1e694c0c357))
* **scir-instances:** allow Substrate users to instantiate raw SCIR instances ([#184](https://github.com/substrate-labs/substrate2/issues/184)) ([8fd5192](https://github.com/substrate-labs/substrate2/commit/8fd5192fd2017ab04e9e3220612d0a132702bb2e))
* **simulation:** proc macros for implementing Supports on tuples ([#163](https://github.com/substrate-labs/substrate2/issues/163)) ([bf77832](https://github.com/substrate-labs/substrate2/commit/bf778329d6e9fd317bea789d093c4c7d8790f5ac))
* **tiling:** array and grid tiling API ([#201](https://github.com/substrate-labs/substrate2/issues/201)) ([b3b7c2b](https://github.com/substrate-labs/substrate2/commit/b3b7c2bfb7ba72198872d0f08ded3e0bc757479d))


### Bug Fixes

* **ci:** fix doc tests for substrate crate ([#158](https://github.com/substrate-labs/substrate2/issues/158)) ([d7e9437](https://github.com/substrate-labs/substrate2/commit/d7e943734b1eadfe64deabb7602f5bbf41cd8806))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * config bumped from 0.1.0 to 0.2.0
    * cache bumped from 0.1.0 to 0.2.0
    * codegen bumped from 0.2.0 to 0.3.0
    * geometry bumped from 0.2.0 to 0.3.0
    * gds bumped from 0.1.0 to 0.2.0
    * scir bumped from 0.1.0 to 0.2.0
    * uniquify bumped from 0.1.0 to 0.2.0
    * spice bumped from 0.0.0 to 0.1.0
  * dev-dependencies
    * sky130pdk bumped from <=0.2.0 to <=0.3.0

## [0.2.0](https://github.com/substrate-labs/substrate2/compare/substrate-v0.1.1...substrate-v0.2.0) (2023-07-07)


### Features

* **reorg:** move substrate-api into substrate ([#155](https://github.com/substrate-labs/substrate2/issues/155)) ([e902a1b](https://github.com/substrate-labs/substrate2/commit/e902a1b603cca6c719770c5cd742e081bfd33e51))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * codegen bumped from 0.1.0 to 0.2.0

## 0.1.0 (2023-07-07)


### Features

* **blackboxing:** support Substrate and SCIR blackboxes ([#135](https://github.com/substrate-labs/substrate2/issues/135)) ([049a598](https://github.com/substrate-labs/substrate2/commit/049a598e2b8d11228c63f03dc878fc4c56e036a6))
* **docs:** add code examples to documentation ([#65](https://github.com/substrate-labs/substrate2/issues/65)) ([bfafd05](https://github.com/substrate-labs/substrate2/commit/bfafd050c1b68d2e9e29e760ca3ff939e26aaeca))
* **layout-api:** initial implementation of layout API ([#61](https://github.com/substrate-labs/substrate2/issues/61)) ([c4cdac7](https://github.com/substrate-labs/substrate2/commit/c4cdac728fd4d4ef5defb97b3c1e1660ee78d672))
* **mos:** add sky130pdk transistor blocks ([#126](https://github.com/substrate-labs/substrate2/issues/126)) ([3e9ee79](https://github.com/substrate-labs/substrate2/commit/3e9ee7935e030ca3e5c4d56f19ccafc27445a6f0))
* **mos:** add standard 4-terminal MosIo ([8678716](https://github.com/substrate-labs/substrate2/commit/86787160c49a1ac7c011d08ce1b9d7851bdfa0d8))
* **organization:** rename substrate to substrate_api, set up codegen crate ([#67](https://github.com/substrate-labs/substrate2/issues/67)) ([e07f099](https://github.com/substrate-labs/substrate2/commit/e07f09949551fd08e3f58b6ffb7d9a8c67b76ae9))
* **pdk:** add PDK trait and update context ([#68](https://github.com/substrate-labs/substrate2/issues/68)) ([a8fbd14](https://github.com/substrate-labs/substrate2/commit/a8fbd14a4b81e504c781e0656edce81853039afb))
* **pdks:** implement `supported_pdks` macro and add examples ([#72](https://github.com/substrate-labs/substrate2/issues/72)) ([5f4312f](https://github.com/substrate-labs/substrate2/commit/5f4312f5220ae6023d78d8f4e585032147195a75))
* **proc-macros:** allow missing docs on generated structs ([8678716](https://github.com/substrate-labs/substrate2/commit/86787160c49a1ac7c011d08ce1b9d7851bdfa0d8))
* **proc-macros:** macros respect field and struct visibilities ([8678716](https://github.com/substrate-labs/substrate2/commit/86787160c49a1ac7c011d08ce1b9d7851bdfa0d8))
* **proc-macros:** proc macros find substrate crate location ([#125](https://github.com/substrate-labs/substrate2/issues/125)) ([8678716](https://github.com/substrate-labs/substrate2/commit/86787160c49a1ac7c011d08ce1b9d7851bdfa0d8))
* **simulation:** simplify SCIR paths for data access ([#143](https://github.com/substrate-labs/substrate2/issues/143)) ([d42e6f9](https://github.com/substrate-labs/substrate2/commit/d42e6f9b1d4236a9024d4a4b839319749033b8d3))
* **simulation:** support transient simulation in spectre ([#133](https://github.com/substrate-labs/substrate2/issues/133)) ([4605862](https://github.com/substrate-labs/substrate2/commit/460586252e3695ae32b0ab8d83b90023125d1a33))
* **simulation:** testbench schematic components ([#136](https://github.com/substrate-labs/substrate2/issues/136)) ([97e6b0f](https://github.com/substrate-labs/substrate2/commit/97e6b0ffd5ea7abd2a547952d5c963745854ed75))
* **tests:** add SCIR and SPICE netlister blackbox tests ([049a598](https://github.com/substrate-labs/substrate2/commit/049a598e2b8d11228c63f03dc878fc4c56e036a6))


### Bug Fixes

* **deps:** fix dependencies and documentation ([#66](https://github.com/substrate-labs/substrate2/issues/66)) ([a60ffc6](https://github.com/substrate-labs/substrate2/commit/a60ffc6c5501200d56a6e76db0c1c2f7ef9cd086))
* **re-exports:** move all re-exports to substrate ([#132](https://github.com/substrate-labs/substrate2/issues/132)) ([8b3d867](https://github.com/substrate-labs/substrate2/commit/8b3d867c7b76a16f422a38a04f5643eb050f14e6))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * geometry bumped from 0.1.0 to 0.2.0
    * codegen bumped from 0.0.0 to 0.1.0
    * substrate_api bumped from 0.0.0 to 0.1.0
    * scir bumped from 0.0.0 to 0.1.0
