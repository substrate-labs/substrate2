# Changelog

* The following workspace dependencies were updated
  * dependencies
    * substrate_api bumped from 0.1.0 to 0.1.1
  * dev-dependencies
    * substrate bumped from 0.1.0 to 0.1.1

* The following workspace dependencies were updated
  * dependencies
    * examples bumped from 0.4.0 to 0.4.1
  * dev-dependencies
    * substrate bumped from <=0.7.0 to <=0.7.1
    * sky130pdk bumped from <=0.7.0 to <=0.7.1
    * spectre bumped from <=0.7.0 to <=0.8.0
    * spice bumped from <=0.5.0 to <=0.6.0

* The following workspace dependencies were updated
  * dependencies
    * examples bumped from 0.5.0 to 0.5.1
  * dev-dependencies
    * substrate bumped from <=0.8.0 to <=0.8.1
    * sky130pdk bumped from <=0.8.0 to <=0.8.1
    * spectre bumped from <=0.9.0 to <=0.9.1
    * spice bumped from <=0.7.0 to <=0.7.1

* The following workspace dependencies were updated
  * dev-dependencies
    * substrate bumped from <=0.10.2 to <=0.10.3
  * build-dependencies
    * examples bumped from 0.2.0 to 0.2.1

## [0.10.2](https://github.com/ucb-substrate/substrate2/compare/codegen-v0.10.1...codegen-v0.10.2) (2025-02-02)


### Bug Fixes

* **deps:** update deps to latest versions ([#562](https://github.com/ucb-substrate/substrate2/issues/562)) ([fe86a45](https://github.com/ucb-substrate/substrate2/commit/fe86a4552ae1238495f26b51443d7729b54398f0))


### Dependencies

* The following workspace dependencies were updated
  * dev-dependencies
    * substrate bumped from <=0.10.1 to <=0.10.2

## [0.10.1](https://github.com/ucb-substrate/substrate2/compare/codegen-v0.10.0...codegen-v0.10.1) (2025-01-24)


### Dependencies

* update dependencies ([0b87032](https://github.com/ucb-substrate/substrate2/commit/0b8703276631fbb19a958453394c981d6b092441))
* update dependencies ([#538](https://github.com/ucb-substrate/substrate2/issues/538)) ([19438d6](https://github.com/ucb-substrate/substrate2/commit/19438d65ac7078a2a971b4147420364ca0717763))
* update deps, GH actions ([#551](https://github.com/ucb-substrate/substrate2/issues/551)) ([357e82a](https://github.com/ucb-substrate/substrate2/commit/357e82ae0a01317d3ad5afb33b5290d3ac10cd7a))
* The following workspace dependencies were updated
  * dev-dependencies
    * substrate bumped from <=0.10.0 to <=0.10.1
    * scir bumped from <=0.9.0 to <=0.9.1

## [0.10.0](https://github.com/ucb-substrate/substrate2/compare/codegen-v0.9.0...codegen-v0.10.0) (2025-01-23)


### Features

* **ac:** implement Save for Substrate types in Spectre AC sim, fix lints ([#471](https://github.com/ucb-substrate/substrate2/issues/471)) ([9825520](https://github.com/ucb-substrate/substrate2/commit/98255207569cc00bd9ddc35419c2df1e48f1999c))
* **atoll:** implement first cut ATOLL implementation ([#357](https://github.com/ucb-substrate/substrate2/issues/357)) ([372b927](https://github.com/ucb-substrate/substrate2/commit/372b9275c9d9c5cd58603f5a462a5e4b66b64cf7))
* **cadence:** add support for Pegasus and Quantus ([#462](https://github.com/ucb-substrate/substrate2/issues/462)) ([953e4cb](https://github.com/ucb-substrate/substrate2/commit/953e4cb761c510668f65f4825f1be3914db45e3c))
* **codegen:** codegen for layout types, example layouts ([#469](https://github.com/ucb-substrate/substrate2/issues/469)) ([255af05](https://github.com/ucb-substrate/substrate2/commit/255af05657c01fcb0b4ff1e6eb0a54244dfeca32))
* **codegen:** derive Block macro adds required trait bounds by default ([#249](https://github.com/ucb-substrate/substrate2/issues/249)) ([892bef5](https://github.com/ucb-substrate/substrate2/commit/892bef585548264e3fcdcc2e6523a2321c6c6897))
* **codegen:** derive macro for implementing FromSaved ([#243](https://github.com/ucb-substrate/substrate2/issues/243)) ([48acae0](https://github.com/ucb-substrate/substrate2/commit/48acae0fb8915c4f968223268c92077f2deda979))
* **codegen:** implement derive proc macro for layout hard macros ([#200](https://github.com/ucb-substrate/substrate2/issues/200)) ([5138224](https://github.com/ucb-substrate/substrate2/commit/5138224013f537e678dfb20204e964852ed40ccb))
* **codegen:** insert appropriate bounds in Io, SchematicType, LayoutType proc macros ([#251](https://github.com/ucb-substrate/substrate2/issues/251)) ([33dcc79](https://github.com/ucb-substrate/substrate2/commit/33dcc797fdbeb21ad046093e655acf965fd99321))
* **corners:** require specifying corner by default ([#221](https://github.com/ucb-substrate/substrate2/issues/221)) ([4c2c3e4](https://github.com/ucb-substrate/substrate2/commit/4c2c3e4a3cd8b7e68921baf3af8b87f1da048936))
* **custom-layout-io:** add way to derive custom layout IOs ([#117](https://github.com/ucb-substrate/substrate2/issues/117)) ([61a8625](https://github.com/ucb-substrate/substrate2/commit/61a86251978fde6e8d1095d33f197d5702d085cc))
* **docs:** add docs for layout IO ([#131](https://github.com/ucb-substrate/substrate2/issues/131)) ([551d65e](https://github.com/ucb-substrate/substrate2/commit/551d65e440ae3c7a9ccbe5d35a7ed5cd93d0d6b3))
* **docs:** fix user docs and update dev docs ([#480](https://github.com/ucb-substrate/substrate2/issues/480)) ([f727a1e](https://github.com/ucb-substrate/substrate2/commit/f727a1e7bd2a795ace1c51c3d6e02f3673d07a29))
* **docs:** reorganize docs and add code snippets ([#216](https://github.com/ucb-substrate/substrate2/issues/216)) ([d7c457d](https://github.com/ucb-substrate/substrate2/commit/d7c457d4e5c1d4846549a0e6df958243042285db))
* **docs:** update tutorials and revamp documentation website ([#315](https://github.com/ucb-substrate/substrate2/issues/315)) ([49bdf7f](https://github.com/ucb-substrate/substrate2/commit/49bdf7ff61e2fdbf19022697d518ad7fbafb465f))
* **docs:** versioned documentation between HEAD and release ([#470](https://github.com/ucb-substrate/substrate2/issues/470)) ([968182b](https://github.com/ucb-substrate/substrate2/commit/968182bf8f8d8b4cf923c0fd66f1ca1b32b12b16))
* **gds-export:** add GDS crate and utilities for accessing GDS layers ([#87](https://github.com/ucb-substrate/substrate2/issues/87)) ([5cf11cd](https://github.com/ucb-substrate/substrate2/commit/5cf11cd0ff80d637ca7210a603625a3b950cdaa4))
* **gds-export:** implement GDS export of Substrate cells ([#97](https://github.com/ucb-substrate/substrate2/issues/97)) ([ae5ca3d](https://github.com/ucb-substrate/substrate2/commit/ae5ca3d0356848eb8e080a7714667193bb9d28fb))
* **gds-import:** implement GDS to RawCell importer ([#196](https://github.com/ucb-substrate/substrate2/issues/196)) ([fc37eeb](https://github.com/ucb-substrate/substrate2/commit/fc37eeb6bac10779491b98bcadcc0eeaeb7d8ec5))
* **impl-dispatch:** remove impl dispatch in favor of trait bounds ([#283](https://github.com/ucb-substrate/substrate2/issues/283)) ([d954115](https://github.com/ucb-substrate/substrate2/commit/d9541152db52aebde928e41c0d800453e906d62b))
* **layer-api:** add layer IDs to shapes ([#85](https://github.com/ucb-substrate/substrate2/issues/85)) ([df7064d](https://github.com/ucb-substrate/substrate2/commit/df7064d0268d1ef7d2ec8bfb5b66434a9b19e819))
* **layer-api:** initial layer API and codegen ([#84](https://github.com/ucb-substrate/substrate2/issues/84)) ([42bd94c](https://github.com/ucb-substrate/substrate2/commit/42bd94c1f1d5e0b013a9b479bf100c68cf9de9a1))
* **layer-families:** implement layer families and clean up codegen ([#127](https://github.com/ucb-substrate/substrate2/issues/127)) ([06f50b8](https://github.com/ucb-substrate/substrate2/commit/06f50b8236ba40f405d7a5e20987a28e01f69f7c))
* **layout-io:** initial layout port API implementation ([#111](https://github.com/ucb-substrate/substrate2/issues/111)) ([ecc8838](https://github.com/ucb-substrate/substrate2/commit/ecc8838678c98f137aca6f4955d89ba350540b44))
* **layout-ports:** initial implementation of layout port traits ([3c0527a](https://github.com/ucb-substrate/substrate2/commit/3c0527a749b2ef7f3b42e46ce66d9f9bed3ff947))
* **layout:** rename `HasLayout` and `HasLayoutImpl` ([#227](https://github.com/ucb-substrate/substrate2/issues/227)) ([2cf1f7d](https://github.com/ucb-substrate/substrate2/commit/2cf1f7d435549df26ff15370e7324e9df76e0e4f))
* **layout:** simplified layout API, LayIR integration ([#459](https://github.com/ucb-substrate/substrate2/issues/459)) ([183d347](https://github.com/ucb-substrate/substrate2/commit/183d347c19e6fe98cf870be4716e7249f23bd423))
* **macros:** refactor derive NestedData, start organizing tests ([#477](https://github.com/ucb-substrate/substrate2/issues/477)) ([aca48ef](https://github.com/ucb-substrate/substrate2/commit/aca48ef7a49c959e35ec4614345a55e667ff5146))
* **macros:** refactor macro reexports ([#250](https://github.com/ucb-substrate/substrate2/issues/250)) ([a332717](https://github.com/ucb-substrate/substrate2/commit/a332717e549fdea50306067e1c92dc60293aed4c))
* **macros:** support ref, mut ref, and owned receiver styles ([#468](https://github.com/ucb-substrate/substrate2/issues/468)) ([b285476](https://github.com/ucb-substrate/substrate2/commit/b285476d3ac378522a1b40ae4e22a69f5e580fda))
* **montecarlo:** add Monte Carlo simulation support to Spectre plugin ([#347](https://github.com/ucb-substrate/substrate2/issues/347)) ([cc9dfe4](https://github.com/ucb-substrate/substrate2/commit/cc9dfe42db5be1a8aaeaf3fb81992a0ad7251ef8))
* **mos:** add standard 4-terminal MosIo ([8678716](https://github.com/ucb-substrate/substrate2/commit/86787160c49a1ac7c011d08ce1b9d7851bdfa0d8))
* **netlister:** reduce duplicate code between spectre and SPICE netlisters ([#261](https://github.com/ucb-substrate/substrate2/issues/261)) ([5ba3623](https://github.com/ucb-substrate/substrate2/commit/5ba36230e653e4dc77819c5c50b527311768cd83))
* **organization:** rename substrate to substrate_api, set up codegen crate ([#67](https://github.com/ucb-substrate/substrate2/issues/67)) ([e07f099](https://github.com/ucb-substrate/substrate2/commit/e07f09949551fd08e3f58b6ffb7d9a8c67b76ae9))
* **parameters:** substrate schematic primitives support parameters ([#233](https://github.com/ucb-substrate/substrate2/issues/233)) ([5dabcb2](https://github.com/ucb-substrate/substrate2/commit/5dabcb270cab0d259b7301d67f77de6d55261092))
* **pdk:** add PDK trait and update context ([#68](https://github.com/ucb-substrate/substrate2/issues/68)) ([a8fbd14](https://github.com/ucb-substrate/substrate2/commit/a8fbd14a4b81e504c781e0656edce81853039afb))
* **pdk:** remove `PdkData` object to clean up interface ([#218](https://github.com/ucb-substrate/substrate2/issues/218)) ([1dd166a](https://github.com/ucb-substrate/substrate2/commit/1dd166a8f23e7b3c011c01b5c8527b8c5494ddea))
* **pdks:** implement `supported_pdks` macro and add examples ([#72](https://github.com/ucb-substrate/substrate2/issues/72)) ([5f4312f](https://github.com/ucb-substrate/substrate2/commit/5f4312f5220ae6023d78d8f4e585032147195a75))
* **primitives:** revamp schematic primitives ([#291](https://github.com/ucb-substrate/substrate2/issues/291)) ([e5ba06a](https://github.com/ucb-substrate/substrate2/commit/e5ba06ab10008b72e78397ad70781caa6bc61791))
* **proc-macros:** add derive Block proc macro ([#151](https://github.com/ucb-substrate/substrate2/issues/151)) ([e2c2f02](https://github.com/ucb-substrate/substrate2/commit/e2c2f02771611ad4a79b3c9516fa1defabc20a66))
* **proc-macros:** allow missing docs on generated structs ([8678716](https://github.com/ucb-substrate/substrate2/commit/86787160c49a1ac7c011d08ce1b9d7851bdfa0d8))
* **proc-macros:** codegen for schematic hard macros ([#191](https://github.com/ucb-substrate/substrate2/issues/191)) ([50240b1](https://github.com/ucb-substrate/substrate2/commit/50240b167876873c4133315d35298b44e8eeac51))
* **proc-macros:** derive macros for geometry traits ([#164](https://github.com/ucb-substrate/substrate2/issues/164)) ([a86074a](https://github.com/ucb-substrate/substrate2/commit/a86074a69b714b1be551ae00c775beb04c13f776))
* **proc-macros:** macros respect field and struct visibilities ([8678716](https://github.com/ucb-substrate/substrate2/commit/86787160c49a1ac7c011d08ce1b9d7851bdfa0d8))
* **proc-macros:** proc macros find substrate crate location ([#125](https://github.com/ucb-substrate/substrate2/issues/125)) ([8678716](https://github.com/ucb-substrate/substrate2/commit/86787160c49a1ac7c011d08ce1b9d7851bdfa0d8))
* **proc-macros:** support enums, tuple structs, etc. ([#165](https://github.com/ucb-substrate/substrate2/issues/165)) ([bda83f7](https://github.com/ucb-substrate/substrate2/commit/bda83f7c3049178024b114eb4e1bf65c6a128998))
* **proc-macros:** support generics in derive schematic/layout data ([#169](https://github.com/ucb-substrate/substrate2/issues/169)) ([5bc11d8](https://github.com/ucb-substrate/substrate2/commit/5bc11d8eee266c21247694299285b6147631166e))
* **refactor:** rename Has_ and Has_Data ([#282](https://github.com/ucb-substrate/substrate2/issues/282)) ([2018153](https://github.com/ucb-substrate/substrate2/commit/2018153686dd7ef3df0e10874db3c656ca245026))
* **refactor:** significantly refactor IO APIs ([#348](https://github.com/ucb-substrate/substrate2/issues/348)) ([c85d043](https://github.com/ucb-substrate/substrate2/commit/c85d04334a0ba1740f9990b91fb55ab1f2ef77c5))
* **reorg:** move substrate-api into substrate ([#155](https://github.com/ucb-substrate/substrate2/issues/155)) ([e902a1b](https://github.com/ucb-substrate/substrate2/commit/e902a1b603cca6c719770c5cd742e081bfd33e51))
* **schematic:** associated type schema and bundle primitives ([#455](https://github.com/ucb-substrate/substrate2/issues/455)) ([f5fde78](https://github.com/ucb-substrate/substrate2/commit/f5fde78824ce9ed0be494ef68d71620181bf6b48))
* **schematic:** nested node and instance access ([#134](https://github.com/ucb-substrate/substrate2/issues/134)) ([3d0e9ce](https://github.com/ucb-substrate/substrate2/commit/3d0e9ce96b66072cd9b7982c582fa2d67ed8f406))
* **schematic:** rename bundle traits ([#458](https://github.com/ucb-substrate/substrate2/issues/458)) ([ed98443](https://github.com/ucb-substrate/substrate2/commit/ed9844318cbd7176a781fff0076d8b3385d408b5))
* **schematics:** implement node naming trees, with codegen ([#105](https://github.com/ucb-substrate/substrate2/issues/105)) ([5ef8e4b](https://github.com/ucb-substrate/substrate2/commit/5ef8e4b8cdd20a274d1a4dadda8e186bed004763))
* **schematics:** implement proc macro to derive AnalogIo ([#99](https://github.com/ucb-substrate/substrate2/issues/99)) ([2320c99](https://github.com/ucb-substrate/substrate2/commit/2320c99e9852d4698c5b336de0af7ebe7cc94204))
* **schematics:** rename `HasSchematic` and `HasSchematicImpl` ([#226](https://github.com/ucb-substrate/substrate2/issues/226)) ([a2b9c78](https://github.com/ucb-substrate/substrate2/commit/a2b9c78ea6ff56983e9a02aeafe655e92852c264))
* **schematics:** user-specified schematic hierarchy flattening ([#222](https://github.com/ucb-substrate/substrate2/issues/222)) ([251f377](https://github.com/ucb-substrate/substrate2/commit/251f37778526d2f1c08a2b3c66f72ffe273021fa))
* **simulation:** automatically generate saved data ([#457](https://github.com/ucb-substrate/substrate2/issues/457)) ([2c936d0](https://github.com/ucb-substrate/substrate2/commit/2c936d00e927b99b624f29e6450826e90f68f9bf))
* **simulation:** implement save for nested instances ([#476](https://github.com/ucb-substrate/substrate2/issues/476)) ([a47d905](https://github.com/ucb-substrate/substrate2/commit/a47d905097c6c196153b53f142ca7e1ffba5eb51))
* **simulation:** improve simulation APIs ([#320](https://github.com/ucb-substrate/substrate2/issues/320)) ([4ed59a1](https://github.com/ucb-substrate/substrate2/commit/4ed59a1283f9546e8336cc96015bd87c55682777))
* **simulation:** proc macros for implementing Supports on tuples ([#163](https://github.com/ucb-substrate/substrate2/issues/163)) ([bf77832](https://github.com/ucb-substrate/substrate2/commit/bf778329d6e9fd317bea789d093c4c7d8790f5ac))
* **spectre:** vsource uses primitives instead of being blackboxed ([5dabcb2](https://github.com/ucb-substrate/substrate2/commit/5dabcb270cab0d259b7301d67f77de6d55261092))
* **stdcells:** add standard cell support to Sky130 PDK ([#323](https://github.com/ucb-substrate/substrate2/issues/323)) ([0b2048e](https://github.com/ucb-substrate/substrate2/commit/0b2048ed44d89c5de87380cac48a4bbff2b4c20a))
* **terminals:** add support for terminal paths ([#236](https://github.com/ucb-substrate/substrate2/issues/236)) ([3fba7f6](https://github.com/ucb-substrate/substrate2/commit/3fba7f6227bbf2efcaf79d849c79175e44d783a4))
* **tests:** fix compilation and lint errors ([#482](https://github.com/ucb-substrate/substrate2/issues/482)) ([b55d04e](https://github.com/ucb-substrate/substrate2/commit/b55d04ecd2472f9f72b926ba5286f0d928bc2691))
* **tests:** reorganize tests and documentation ([#464](https://github.com/ucb-substrate/substrate2/issues/464)) ([928b9b7](https://github.com/ucb-substrate/substrate2/commit/928b9b7c45dc334ca11d86e4564edc58bf6db6f2))
* **tiling:** array and grid tiling API ([#201](https://github.com/ucb-substrate/substrate2/issues/201)) ([b3b7c2b](https://github.com/ucb-substrate/substrate2/commit/b3b7c2bfb7ba72198872d0f08ded3e0bc757479d))
* **transform:** default to Manhattan transformations ([#452](https://github.com/ucb-substrate/substrate2/issues/452)) ([3d8a410](https://github.com/ucb-substrate/substrate2/commit/3d8a4109febb11616d550c8cd6373e8f605b2e28))
* **type-dispatch:** add helper crate for dispatching types ([#225](https://github.com/ucb-substrate/substrate2/issues/225)) ([13ee1aa](https://github.com/ucb-substrate/substrate2/commit/13ee1aa1b287ed0c147549003c0af815b849577b))
* **views:** view API for improved codegen ([#463](https://github.com/ucb-substrate/substrate2/issues/463)) ([b75328c](https://github.com/ucb-substrate/substrate2/commit/b75328c9a4840ed9200a9035e28e27ac9265770f))


### Bug Fixes

* **ci:** add workaround for dev deps ([180c924](https://github.com/ucb-substrate/substrate2/commit/180c92434b38a5da8d5d1f0494faae6a0b227c26))
* **ci:** test another workaround for dev deps ([c15bc6d](https://github.com/ucb-substrate/substrate2/commit/c15bc6d30afc02512237223db5f31cd9cb089ede))
* **codegen:** update codegen to use fewer structs ([#461](https://github.com/ucb-substrate/substrate2/issues/461)) ([c371be5](https://github.com/ucb-substrate/substrate2/commit/c371be59adebb9482095284034d41a6905c431d4))
* **deps:** update rust crate syn to v2 ([#79](https://github.com/ucb-substrate/substrate2/issues/79)) ([eee3593](https://github.com/ucb-substrate/substrate2/commit/eee35938247f2660c15b0165b6ba3d609d7091b8))
* **docs:** fix snippet publishing ([#512](https://github.com/ucb-substrate/substrate2/issues/512)) ([456f8bf](https://github.com/ucb-substrate/substrate2/commit/456f8bfe659d4fa2a05f6d56394a6171c4fd34dd))
* **docs:** remove Cargo.tomls in CI to allow publishing of API docs ([#515](https://github.com/ucb-substrate/substrate2/issues/515)) ([2d14f50](https://github.com/ucb-substrate/substrate2/commit/2d14f50add396a1d775428b273df7d8d022aea05))
* **gds:** use u16 instead of u8 for GDS layerspecs ([#339](https://github.com/ucb-substrate/substrate2/issues/339)) ([4d1fce2](https://github.com/ucb-substrate/substrate2/commit/4d1fce25f9493c6975d43dba96ccaa4c0cf4a686))
* **generics:** change `Deserialize&lt;'static&gt;` bounds to `DeserializeOwned` ([#259](https://github.com/ucb-substrate/substrate2/issues/259)) ([8015063](https://github.com/ucb-substrate/substrate2/commit/80150630b094a04a75cfc5b681255b80caf4f895))
* **tests:** increase cache server wait time ([#167](https://github.com/ucb-substrate/substrate2/issues/167)) ([b0db3aa](https://github.com/ucb-substrate/substrate2/commit/b0db3aa6285367de1650e972c9cf7e2185a68250))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * snippets bumped from 0.6.0 to 0.7.0
    * macrotools bumped from 0.1.0 to 0.2.0
  * dev-dependencies
    * substrate bumped from <=0.9.0 to <=0.10.0
    * scir bumped from <=0.8.0 to <=0.9.0
  * build-dependencies
    * snippets bumped from 0.6.0 to 0.7.0
    * examples bumped from 0.1.2 to 0.2.0

## [0.9.0](https://github.com/ucb-substrate/substrate2/compare/codegen-v0.8.1...codegen-v0.9.0) (2025-01-22)


### Features

* **ac:** implement Save for Substrate types in Spectre AC sim, fix lints ([#471](https://github.com/ucb-substrate/substrate2/issues/471)) ([9825520](https://github.com/ucb-substrate/substrate2/commit/98255207569cc00bd9ddc35419c2df1e48f1999c))
* **atoll:** implement first cut ATOLL implementation ([#357](https://github.com/ucb-substrate/substrate2/issues/357)) ([372b927](https://github.com/ucb-substrate/substrate2/commit/372b9275c9d9c5cd58603f5a462a5e4b66b64cf7))
* **cadence:** add support for Pegasus and Quantus ([#462](https://github.com/ucb-substrate/substrate2/issues/462)) ([953e4cb](https://github.com/ucb-substrate/substrate2/commit/953e4cb761c510668f65f4825f1be3914db45e3c))
* **codegen:** codegen for layout types, example layouts ([#469](https://github.com/ucb-substrate/substrate2/issues/469)) ([255af05](https://github.com/ucb-substrate/substrate2/commit/255af05657c01fcb0b4ff1e6eb0a54244dfeca32))
* **docs:** fix user docs and update dev docs ([#480](https://github.com/ucb-substrate/substrate2/issues/480)) ([f727a1e](https://github.com/ucb-substrate/substrate2/commit/f727a1e7bd2a795ace1c51c3d6e02f3673d07a29))
* **docs:** versioned documentation between HEAD and release ([#470](https://github.com/ucb-substrate/substrate2/issues/470)) ([968182b](https://github.com/ucb-substrate/substrate2/commit/968182bf8f8d8b4cf923c0fd66f1ca1b32b12b16))
* **layout:** simplified layout API, LayIR integration ([#459](https://github.com/ucb-substrate/substrate2/issues/459)) ([183d347](https://github.com/ucb-substrate/substrate2/commit/183d347c19e6fe98cf870be4716e7249f23bd423))
* **macros:** refactor derive NestedData, start organizing tests ([#477](https://github.com/ucb-substrate/substrate2/issues/477)) ([aca48ef](https://github.com/ucb-substrate/substrate2/commit/aca48ef7a49c959e35ec4614345a55e667ff5146))
* **macros:** support ref, mut ref, and owned receiver styles ([#468](https://github.com/ucb-substrate/substrate2/issues/468)) ([b285476](https://github.com/ucb-substrate/substrate2/commit/b285476d3ac378522a1b40ae4e22a69f5e580fda))
* **montecarlo:** add Monte Carlo simulation support to Spectre plugin ([#347](https://github.com/ucb-substrate/substrate2/issues/347)) ([cc9dfe4](https://github.com/ucb-substrate/substrate2/commit/cc9dfe42db5be1a8aaeaf3fb81992a0ad7251ef8))
* **refactor:** significantly refactor IO APIs ([#348](https://github.com/ucb-substrate/substrate2/issues/348)) ([c85d043](https://github.com/ucb-substrate/substrate2/commit/c85d04334a0ba1740f9990b91fb55ab1f2ef77c5))
* **schematic:** associated type schema and bundle primitives ([#455](https://github.com/ucb-substrate/substrate2/issues/455)) ([f5fde78](https://github.com/ucb-substrate/substrate2/commit/f5fde78824ce9ed0be494ef68d71620181bf6b48))
* **schematic:** rename bundle traits ([#458](https://github.com/ucb-substrate/substrate2/issues/458)) ([ed98443](https://github.com/ucb-substrate/substrate2/commit/ed9844318cbd7176a781fff0076d8b3385d408b5))
* **simulation:** automatically generate saved data ([#457](https://github.com/ucb-substrate/substrate2/issues/457)) ([2c936d0](https://github.com/ucb-substrate/substrate2/commit/2c936d00e927b99b624f29e6450826e90f68f9bf))
* **simulation:** implement save for nested instances ([#476](https://github.com/ucb-substrate/substrate2/issues/476)) ([a47d905](https://github.com/ucb-substrate/substrate2/commit/a47d905097c6c196153b53f142ca7e1ffba5eb51))
* **tests:** fix compilation and lint errors ([#482](https://github.com/ucb-substrate/substrate2/issues/482)) ([b55d04e](https://github.com/ucb-substrate/substrate2/commit/b55d04ecd2472f9f72b926ba5286f0d928bc2691))
* **tests:** reorganize tests and documentation ([#464](https://github.com/ucb-substrate/substrate2/issues/464)) ([928b9b7](https://github.com/ucb-substrate/substrate2/commit/928b9b7c45dc334ca11d86e4564edc58bf6db6f2))
* **transform:** default to Manhattan transformations ([#452](https://github.com/ucb-substrate/substrate2/issues/452)) ([3d8a410](https://github.com/ucb-substrate/substrate2/commit/3d8a4109febb11616d550c8cd6373e8f605b2e28))
* **views:** view API for improved codegen ([#463](https://github.com/ucb-substrate/substrate2/issues/463)) ([b75328c](https://github.com/ucb-substrate/substrate2/commit/b75328c9a4840ed9200a9035e28e27ac9265770f))


### Bug Fixes

* **codegen:** update codegen to use fewer structs ([#461](https://github.com/ucb-substrate/substrate2/issues/461)) ([c371be5](https://github.com/ucb-substrate/substrate2/commit/c371be59adebb9482095284034d41a6905c431d4))
* **gds:** use u16 instead of u8 for GDS layerspecs ([#339](https://github.com/ucb-substrate/substrate2/issues/339)) ([4d1fce2](https://github.com/ucb-substrate/substrate2/commit/4d1fce25f9493c6975d43dba96ccaa4c0cf4a686))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * snippets bumped from 0.5.1 to 0.6.0
    * macrotools bumped from 0.0.0 to 0.1.0
  * dev-dependencies
    * substrate bumped from <=0.8.1 to <=0.9.0
    * scir bumped from 0.7.0 to <=0.8.0
  * build-dependencies
    * snippets bumped from 0.5.1 to 0.6.0

## [0.8.0](https://github.com/ucb-substrate/substrate2/compare/codegen-v0.7.1...codegen-v0.8.0) (2023-11-25)


### Features

* **docs:** update tutorials and revamp documentation website ([#315](https://github.com/ucb-substrate/substrate2/issues/315)) ([49bdf7f](https://github.com/ucb-substrate/substrate2/commit/49bdf7ff61e2fdbf19022697d518ad7fbafb465f))
* **simulation:** improve simulation APIs ([#320](https://github.com/ucb-substrate/substrate2/issues/320)) ([4ed59a1](https://github.com/ucb-substrate/substrate2/commit/4ed59a1283f9546e8336cc96015bd87c55682777))
* **stdcells:** add standard cell support to Sky130 PDK ([#323](https://github.com/ucb-substrate/substrate2/issues/323)) ([0b2048e](https://github.com/ucb-substrate/substrate2/commit/0b2048ed44d89c5de87380cac48a4bbff2b4c20a))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * examples bumped from 0.4.1 to 0.5.0
  * dev-dependencies
    * substrate bumped from <=0.7.1 to <=0.8.0
    * sky130pdk bumped from <=0.7.1 to <=0.8.0
    * spectre bumped from <=0.8.0 to <=0.9.0
    * spice bumped from <=0.6.0 to <=0.7.0

## [0.7.0](https://github.com/ucb-substrate/substrate2/compare/codegen-v0.6.1...codegen-v0.7.0) (2023-11-02)


### Features

* **impl-dispatch:** remove impl dispatch in favor of trait bounds ([#283](https://github.com/ucb-substrate/substrate2/issues/283)) ([d954115](https://github.com/ucb-substrate/substrate2/commit/d9541152db52aebde928e41c0d800453e906d62b))
* **netlister:** reduce duplicate code between spectre and SPICE netlisters ([#261](https://github.com/ucb-substrate/substrate2/issues/261)) ([5ba3623](https://github.com/ucb-substrate/substrate2/commit/5ba36230e653e4dc77819c5c50b527311768cd83))
* **primitives:** revamp schematic primitives ([#291](https://github.com/ucb-substrate/substrate2/issues/291)) ([e5ba06a](https://github.com/ucb-substrate/substrate2/commit/e5ba06ab10008b72e78397ad70781caa6bc61791))
* **refactor:** rename Has_ and Has_Data ([#282](https://github.com/ucb-substrate/substrate2/issues/282)) ([2018153](https://github.com/ucb-substrate/substrate2/commit/2018153686dd7ef3df0e10874db3c656ca245026))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * examples bumped from 0.3.1 to 0.4.0
  * dev-dependencies
    * substrate bumped from <=0.6.1 to <=0.7.0
    * sky130pdk bumped from <=0.6.1 to <=0.7.0
    * spectre bumped from <=0.6.1 to <=0.7.0
    * spice bumped from <=0.4.0 to <=0.5.0

## [0.6.1](https://github.com/substrate-labs/substrate2/compare/codegen-v0.6.0...codegen-v0.6.1) (2023-08-08)


### Bug Fixes

* **generics:** change `Deserialize&lt;'static&gt;` bounds to `DeserializeOwned` ([#259](https://github.com/substrate-labs/substrate2/issues/259)) ([8015063](https://github.com/substrate-labs/substrate2/commit/80150630b094a04a75cfc5b681255b80caf4f895))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * examples bumped from 0.3.0 to 0.3.1
  * dev-dependencies
    * substrate bumped from <=0.6.0 to <=0.6.1
    * sky130pdk bumped from <=0.6.0 to <=0.6.1
    * spectre bumped from <=0.6.0 to <=0.6.1

## [0.6.0](https://github.com/substrate-labs/substrate2/compare/codegen-v0.5.0...codegen-v0.6.0) (2023-08-08)


### Features

* **codegen:** derive Block macro adds required trait bounds by default ([#249](https://github.com/substrate-labs/substrate2/issues/249)) ([892bef5](https://github.com/substrate-labs/substrate2/commit/892bef585548264e3fcdcc2e6523a2321c6c6897))
* **codegen:** insert appropriate bounds in Io, SchematicType, LayoutType proc macros ([#251](https://github.com/substrate-labs/substrate2/issues/251)) ([33dcc79](https://github.com/substrate-labs/substrate2/commit/33dcc797fdbeb21ad046093e655acf965fd99321))
* **macros:** refactor macro reexports ([#250](https://github.com/substrate-labs/substrate2/issues/250)) ([a332717](https://github.com/substrate-labs/substrate2/commit/a332717e549fdea50306067e1c92dc60293aed4c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * examples bumped from 0.2.0 to 0.3.0
    * type_dispatch bumped from 0.2.0 to 0.3.0
  * dev-dependencies
    * substrate bumped from <=0.5.0 to <=0.6.0
    * sky130pdk bumped from <=0.5.0 to <=0.6.0
    * spectre bumped from <=0.5.0 to <=0.6.0

## [0.5.0](https://github.com/substrate-labs/substrate2/compare/codegen-v0.4.0...codegen-v0.5.0) (2023-08-05)


### Features

* **codegen:** derive macro for implementing FromSaved ([#243](https://github.com/substrate-labs/substrate2/issues/243)) ([48acae0](https://github.com/substrate-labs/substrate2/commit/48acae0fb8915c4f968223268c92077f2deda979))
* **terminals:** add support for terminal paths ([#236](https://github.com/substrate-labs/substrate2/issues/236)) ([3fba7f6](https://github.com/substrate-labs/substrate2/commit/3fba7f6227bbf2efcaf79d849c79175e44d783a4))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * examples bumped from 0.1.0 to 0.2.0
  * dev-dependencies
    * substrate bumped from <=0.4.0 to <=0.5.0
    * sky130pdk bumped from <=0.4.0 to <=0.5.0
    * spectre bumped from <=0.4.0 to <=0.5.0

## [0.4.0](https://github.com/substrate-labs/substrate2/compare/codegen-v0.3.0...codegen-v0.4.0) (2023-08-04)


### Features

* **corners:** require specifying corner by default ([#221](https://github.com/substrate-labs/substrate2/issues/221)) ([4c2c3e4](https://github.com/substrate-labs/substrate2/commit/4c2c3e4a3cd8b7e68921baf3af8b87f1da048936))
* **docs:** reorganize docs and add code snippets ([#216](https://github.com/substrate-labs/substrate2/issues/216)) ([d7c457d](https://github.com/substrate-labs/substrate2/commit/d7c457d4e5c1d4846549a0e6df958243042285db))
* **layout:** rename `HasLayout` and `HasLayoutImpl` ([#227](https://github.com/substrate-labs/substrate2/issues/227)) ([2cf1f7d](https://github.com/substrate-labs/substrate2/commit/2cf1f7d435549df26ff15370e7324e9df76e0e4f))
* **parameters:** substrate schematic primitives support parameters ([#233](https://github.com/substrate-labs/substrate2/issues/233)) ([5dabcb2](https://github.com/substrate-labs/substrate2/commit/5dabcb270cab0d259b7301d67f77de6d55261092))
* **pdk:** remove `PdkData` object to clean up interface ([#218](https://github.com/substrate-labs/substrate2/issues/218)) ([1dd166a](https://github.com/substrate-labs/substrate2/commit/1dd166a8f23e7b3c011c01b5c8527b8c5494ddea))
* **schematics:** rename `HasSchematic` and `HasSchematicImpl` ([#226](https://github.com/substrate-labs/substrate2/issues/226)) ([a2b9c78](https://github.com/substrate-labs/substrate2/commit/a2b9c78ea6ff56983e9a02aeafe655e92852c264))
* **schematics:** user-specified schematic hierarchy flattening ([#222](https://github.com/substrate-labs/substrate2/issues/222)) ([251f377](https://github.com/substrate-labs/substrate2/commit/251f37778526d2f1c08a2b3c66f72ffe273021fa))
* **spectre:** vsource uses primitives instead of being blackboxed ([5dabcb2](https://github.com/substrate-labs/substrate2/commit/5dabcb270cab0d259b7301d67f77de6d55261092))
* **type-dispatch:** add helper crate for dispatching types ([#225](https://github.com/substrate-labs/substrate2/issues/225)) ([13ee1aa](https://github.com/substrate-labs/substrate2/commit/13ee1aa1b287ed0c147549003c0af815b849577b))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * examples bumped from <=0.0.0 to 0.1.0
  * dev-dependencies
    * substrate bumped from <=0.3.0 to <=0.4.0
    * sky130pdk bumped from <=0.3.0 to <=0.4.0

## [0.3.0](https://github.com/substrate-labs/substrate2/compare/codegen-v0.2.0...codegen-v0.3.0) (2023-07-23)


### Features

* **codegen:** implement derive proc macro for layout hard macros ([#200](https://github.com/substrate-labs/substrate2/issues/200)) ([5138224](https://github.com/substrate-labs/substrate2/commit/5138224013f537e678dfb20204e964852ed40ccb))
* **gds-import:** implement GDS to RawCell importer ([#196](https://github.com/substrate-labs/substrate2/issues/196)) ([fc37eeb](https://github.com/substrate-labs/substrate2/commit/fc37eeb6bac10779491b98bcadcc0eeaeb7d8ec5))
* **proc-macros:** codegen for schematic hard macros ([#191](https://github.com/substrate-labs/substrate2/issues/191)) ([50240b1](https://github.com/substrate-labs/substrate2/commit/50240b167876873c4133315d35298b44e8eeac51))
* **proc-macros:** derive macros for geometry traits ([#164](https://github.com/substrate-labs/substrate2/issues/164)) ([a86074a](https://github.com/substrate-labs/substrate2/commit/a86074a69b714b1be551ae00c775beb04c13f776))
* **proc-macros:** support enums, tuple structs, etc. ([#165](https://github.com/substrate-labs/substrate2/issues/165)) ([bda83f7](https://github.com/substrate-labs/substrate2/commit/bda83f7c3049178024b114eb4e1bf65c6a128998))
* **proc-macros:** support generics in derive schematic/layout data ([#169](https://github.com/substrate-labs/substrate2/issues/169)) ([5bc11d8](https://github.com/substrate-labs/substrate2/commit/5bc11d8eee266c21247694299285b6147631166e))
* **simulation:** proc macros for implementing Supports on tuples ([#163](https://github.com/substrate-labs/substrate2/issues/163)) ([bf77832](https://github.com/substrate-labs/substrate2/commit/bf778329d6e9fd317bea789d093c4c7d8790f5ac))
* **tiling:** array and grid tiling API ([#201](https://github.com/substrate-labs/substrate2/issues/201)) ([b3b7c2b](https://github.com/substrate-labs/substrate2/commit/b3b7c2bfb7ba72198872d0f08ded3e0bc757479d))


### Bug Fixes

* **ci:** add workaround for dev deps ([180c924](https://github.com/substrate-labs/substrate2/commit/180c92434b38a5da8d5d1f0494faae6a0b227c26))
* **ci:** test another workaround for dev deps ([c15bc6d](https://github.com/substrate-labs/substrate2/commit/c15bc6d30afc02512237223db5f31cd9cb089ede))
* **tests:** increase cache server wait time ([#167](https://github.com/substrate-labs/substrate2/issues/167)) ([b0db3aa](https://github.com/substrate-labs/substrate2/commit/b0db3aa6285367de1650e972c9cf7e2185a68250))


### Dependencies

* The following workspace dependencies were updated
  * dev-dependencies
    * substrate bumped from <=0.2.0 to <=0.3.0
    * sky130pdk bumped from <=0.2.0 to <=0.3.0

## [0.2.0](https://github.com/substrate-labs/substrate2/compare/codegen-v0.1.1...codegen-v0.2.0) (2023-07-07)


### Features

* **reorg:** move substrate-api into substrate ([#155](https://github.com/substrate-labs/substrate2/issues/155)) ([e902a1b](https://github.com/substrate-labs/substrate2/commit/e902a1b603cca6c719770c5cd742e081bfd33e51))


### Dependencies

* The following workspace dependencies were updated
  * dev-dependencies
    * substrate bumped from <=0.1.1 to 0.2.0

## 0.1.0 (2023-07-07)


### Features

* **custom-layout-io:** add way to derive custom layout IOs ([#117](https://github.com/substrate-labs/substrate2/issues/117)) ([61a8625](https://github.com/substrate-labs/substrate2/commit/61a86251978fde6e8d1095d33f197d5702d085cc))
* **docs:** add docs for layout IO ([#131](https://github.com/substrate-labs/substrate2/issues/131)) ([551d65e](https://github.com/substrate-labs/substrate2/commit/551d65e440ae3c7a9ccbe5d35a7ed5cd93d0d6b3))
* **gds-export:** add GDS crate and utilities for accessing GDS layers ([#87](https://github.com/substrate-labs/substrate2/issues/87)) ([5cf11cd](https://github.com/substrate-labs/substrate2/commit/5cf11cd0ff80d637ca7210a603625a3b950cdaa4))
* **gds-export:** implement GDS export of Substrate cells ([#97](https://github.com/substrate-labs/substrate2/issues/97)) ([ae5ca3d](https://github.com/substrate-labs/substrate2/commit/ae5ca3d0356848eb8e080a7714667193bb9d28fb))
* **layer-api:** add layer IDs to shapes ([#85](https://github.com/substrate-labs/substrate2/issues/85)) ([df7064d](https://github.com/substrate-labs/substrate2/commit/df7064d0268d1ef7d2ec8bfb5b66434a9b19e819))
* **layer-api:** initial layer API and codegen ([#84](https://github.com/substrate-labs/substrate2/issues/84)) ([42bd94c](https://github.com/substrate-labs/substrate2/commit/42bd94c1f1d5e0b013a9b479bf100c68cf9de9a1))
* **layer-families:** implement layer families and clean up codegen ([#127](https://github.com/substrate-labs/substrate2/issues/127)) ([06f50b8](https://github.com/substrate-labs/substrate2/commit/06f50b8236ba40f405d7a5e20987a28e01f69f7c))
* **layout-io:** initial layout port API implementation ([#111](https://github.com/substrate-labs/substrate2/issues/111)) ([ecc8838](https://github.com/substrate-labs/substrate2/commit/ecc8838678c98f137aca6f4955d89ba350540b44))
* **layout-ports:** initial implementation of layout port traits ([3c0527a](https://github.com/substrate-labs/substrate2/commit/3c0527a749b2ef7f3b42e46ce66d9f9bed3ff947))
* **mos:** add standard 4-terminal MosIo ([8678716](https://github.com/substrate-labs/substrate2/commit/86787160c49a1ac7c011d08ce1b9d7851bdfa0d8))
* **organization:** rename substrate to substrate_api, set up codegen crate ([#67](https://github.com/substrate-labs/substrate2/issues/67)) ([e07f099](https://github.com/substrate-labs/substrate2/commit/e07f09949551fd08e3f58b6ffb7d9a8c67b76ae9))
* **pdk:** add PDK trait and update context ([#68](https://github.com/substrate-labs/substrate2/issues/68)) ([a8fbd14](https://github.com/substrate-labs/substrate2/commit/a8fbd14a4b81e504c781e0656edce81853039afb))
* **pdks:** implement `supported_pdks` macro and add examples ([#72](https://github.com/substrate-labs/substrate2/issues/72)) ([5f4312f](https://github.com/substrate-labs/substrate2/commit/5f4312f5220ae6023d78d8f4e585032147195a75))
* **proc-macros:** add derive Block proc macro ([#151](https://github.com/substrate-labs/substrate2/issues/151)) ([e2c2f02](https://github.com/substrate-labs/substrate2/commit/e2c2f02771611ad4a79b3c9516fa1defabc20a66))
* **proc-macros:** allow missing docs on generated structs ([8678716](https://github.com/substrate-labs/substrate2/commit/86787160c49a1ac7c011d08ce1b9d7851bdfa0d8))
* **proc-macros:** macros respect field and struct visibilities ([8678716](https://github.com/substrate-labs/substrate2/commit/86787160c49a1ac7c011d08ce1b9d7851bdfa0d8))
* **proc-macros:** proc macros find substrate crate location ([#125](https://github.com/substrate-labs/substrate2/issues/125)) ([8678716](https://github.com/substrate-labs/substrate2/commit/86787160c49a1ac7c011d08ce1b9d7851bdfa0d8))
* **schematic:** nested node and instance access ([#134](https://github.com/substrate-labs/substrate2/issues/134)) ([3d0e9ce](https://github.com/substrate-labs/substrate2/commit/3d0e9ce96b66072cd9b7982c582fa2d67ed8f406))
* **schematics:** implement node naming trees, with codegen ([#105](https://github.com/substrate-labs/substrate2/issues/105)) ([5ef8e4b](https://github.com/substrate-labs/substrate2/commit/5ef8e4b8cdd20a274d1a4dadda8e186bed004763))
* **schematics:** implement proc macro to derive AnalogIo ([#99](https://github.com/substrate-labs/substrate2/issues/99)) ([2320c99](https://github.com/substrate-labs/substrate2/commit/2320c99e9852d4698c5b336de0af7ebe7cc94204))


### Bug Fixes

* **deps:** update rust crate syn to v2 ([#79](https://github.com/substrate-labs/substrate2/issues/79)) ([eee3593](https://github.com/substrate-labs/substrate2/commit/eee35938247f2660c15b0165b6ba3d609d7091b8))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * substrate_api bumped from 0.0.0 to 0.1.0
  * dev-dependencies
    * substrate bumped from 0.0.0 to 0.1.0
