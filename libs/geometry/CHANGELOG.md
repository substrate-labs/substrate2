# Changelog

## [0.7.2](https://github.com/ucb-substrate/substrate2/compare/geometry-v0.7.1...geometry-v0.7.2) (2025-02-18)


### Bug Fixes

* **polygon:** fix implementation of `Contains&lt;Point&gt;` for `Polygon` ([#592](https://github.com/ucb-substrate/substrate2/issues/592)) ([585f4e3](https://github.com/ucb-substrate/substrate2/commit/585f4e37a35d18e30348b86da598cb31710de19e))

## [0.7.1](https://github.com/ucb-substrate/substrate2/compare/geometry-v0.7.0...geometry-v0.7.1) (2025-01-24)


### Dependencies

* update dependencies ([0b87032](https://github.com/ucb-substrate/substrate2/commit/0b8703276631fbb19a958453394c981d6b092441))
* The following workspace dependencies were updated
  * dependencies
    * geometry_macros bumped from 0.1.0 to 0.1.1

## [0.7.0](https://github.com/ucb-substrate/substrate2/compare/geometry-v0.6.0...geometry-v0.7.0) (2025-01-23)


### Features

* **ac:** implement Save for Substrate types in Spectre AC sim, fix lints ([#471](https://github.com/ucb-substrate/substrate2/issues/471)) ([9825520](https://github.com/ucb-substrate/substrate2/commit/98255207569cc00bd9ddc35419c2df1e48f1999c))
* **atoll:** implement first cut ATOLL implementation ([#357](https://github.com/ucb-substrate/substrate2/issues/357)) ([372b927](https://github.com/ucb-substrate/substrate2/commit/372b9275c9d9c5cd58603f5a462a5e4b66b64cf7))
* **bbox:** add bbox_rect method ([#373](https://github.com/ucb-substrate/substrate2/issues/373)) ([55b2632](https://github.com/ucb-substrate/substrate2/commit/55b2632a3c1e1ad260b61c6545143a2b16ef1150))
* **blackboxing:** support Substrate and SCIR blackboxes ([#135](https://github.com/ucb-substrate/substrate2/issues/135)) ([049a598](https://github.com/ucb-substrate/substrate2/commit/049a598e2b8d11228c63f03dc878fc4c56e036a6))
* **def:** utilities for exporting def orientations ([#434](https://github.com/ucb-substrate/substrate2/issues/434)) ([43a2b29](https://github.com/ucb-substrate/substrate2/commit/43a2b2906231cd46f08e2c4aface260d34abac62))
* **dirs:** add `Dirs` struct ([#371](https://github.com/ucb-substrate/substrate2/issues/371)) ([6d6b834](https://github.com/ucb-substrate/substrate2/commit/6d6b8347eea60ed1fccaed16623d146c3bd0727e))
* **gds-import:** implement GDS to RawCell importer ([#196](https://github.com/ucb-substrate/substrate2/issues/196)) ([fc37eeb](https://github.com/ucb-substrate/substrate2/commit/fc37eeb6bac10779491b98bcadcc0eeaeb7d8ec5))
* **gds:** gds reexport test ([#199](https://github.com/ucb-substrate/substrate2/issues/199)) ([93d3cd5](https://github.com/ucb-substrate/substrate2/commit/93d3cd555c1cb4a76a8845f4401e98d327b5d674))
* **geometry:** implemented contains for polygon ([#292](https://github.com/ucb-substrate/substrate2/issues/292)) ([708053a](https://github.com/ucb-substrate/substrate2/commit/708053adfb9f3783fc03895ede7348ace51730f0))
* **geometry:** support for rectangular rings ([#408](https://github.com/ucb-substrate/substrate2/issues/408)) ([6fc0f36](https://github.com/ucb-substrate/substrate2/commit/6fc0f361f2215968f698281bfaf37d03d3ec131e))
* **grid:** add ATOLL LCM routing grid and layer stack definition ([#338](https://github.com/ucb-substrate/substrate2/issues/338)) ([1e1ad90](https://github.com/ucb-substrate/substrate2/commit/1e1ad90d02b50dd0dd15516c306971241bf30b7c))
* **layir:** initial LayIR implementation ([#456](https://github.com/ucb-substrate/substrate2/issues/456)) ([4f76d41](https://github.com/ucb-substrate/substrate2/commit/4f76d41c86fd0c57e525f40c976b5eeb0bbd4c68))
* **macros:** refactor macro reexports ([#250](https://github.com/ucb-substrate/substrate2/issues/250)) ([a332717](https://github.com/ucb-substrate/substrate2/commit/a332717e549fdea50306067e1c92dc60293aed4c))
* **netlists:** use consistent ordering via indexmap ([#266](https://github.com/ucb-substrate/substrate2/issues/266)) ([f275c19](https://github.com/ucb-substrate/substrate2/commit/f275c19396ed4f7d255836822ff72b808f89cde7)), closes [#265](https://github.com/ucb-substrate/substrate2/issues/265)
* **polygon:** polygon implemented in geometry ([#263](https://github.com/ucb-substrate/substrate2/issues/263)) ([4508570](https://github.com/ucb-substrate/substrate2/commit/45085706a30a12f4af6c5e3f642ca55b4c32dd24))
* **proc-macros:** derive macros for geometry traits ([#164](https://github.com/ucb-substrate/substrate2/issues/164)) ([a86074a](https://github.com/ucb-substrate/substrate2/commit/a86074a69b714b1be551ae00c775beb04c13f776))
* **tests:** add SCIR and SPICE netlister blackbox tests ([049a598](https://github.com/ucb-substrate/substrate2/commit/049a598e2b8d11228c63f03dc878fc4c56e036a6))
* **tests:** fix compilation and lint errors ([#482](https://github.com/ucb-substrate/substrate2/issues/482)) ([b55d04e](https://github.com/ucb-substrate/substrate2/commit/b55d04ecd2472f9f72b926ba5286f0d928bc2691))
* **tests:** reorganize tests and documentation ([#464](https://github.com/ucb-substrate/substrate2/issues/464)) ([928b9b7](https://github.com/ucb-substrate/substrate2/commit/928b9b7c45dc334ca11d86e4564edc58bf6db6f2))
* **tiling:** array and grid tiling API ([#201](https://github.com/ucb-substrate/substrate2/issues/201)) ([b3b7c2b](https://github.com/ucb-substrate/substrate2/commit/b3b7c2bfb7ba72198872d0f08ded3e0bc757479d))
* **transform:** default to Manhattan transformations ([#452](https://github.com/ucb-substrate/substrate2/issues/452)) ([3d8a410](https://github.com/ucb-substrate/substrate2/commit/3d8a4109febb11616d550c8cd6373e8f605b2e28))
* **transform:** make transformations use integers instead of floats ([#451](https://github.com/ucb-substrate/substrate2/issues/451)) ([aa9764e](https://github.com/ucb-substrate/substrate2/commit/aa9764e8b63b0a344d5e12ad3c678849c5c8ebea))
* **tutorial:** implement sky130 inverter layout tutorial ([#481](https://github.com/ucb-substrate/substrate2/issues/481)) ([440ab0e](https://github.com/ucb-substrate/substrate2/commit/440ab0e6ac33a8396c10f09637242efa32cfca62))


### Bug Fixes

* **deps:** bump rust to version 1.75.0 ([#362](https://github.com/ucb-substrate/substrate2/issues/362)) ([e1e82c9](https://github.com/ucb-substrate/substrate2/commit/e1e82c94cdf6ba4426f3f73f29dca40674a7f064))
* **deps:** update rust crate num-rational to 0.4 ([#294](https://github.com/ucb-substrate/substrate2/issues/294)) ([fc8f5ce](https://github.com/ucb-substrate/substrate2/commit/fc8f5ce9f35eb074acff45115e44ffbd37e0d237))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * geometry_macros bumped from 0.0.2 to 0.0.3

## [0.6.0](https://github.com/ucb-substrate/substrate2/compare/geometry-v0.5.0...geometry-v0.6.0) (2025-01-22)


### Features

* **ac:** implement Save for Substrate types in Spectre AC sim, fix lints ([#471](https://github.com/ucb-substrate/substrate2/issues/471)) ([9825520](https://github.com/ucb-substrate/substrate2/commit/98255207569cc00bd9ddc35419c2df1e48f1999c))
* **atoll:** implement first cut ATOLL implementation ([#357](https://github.com/ucb-substrate/substrate2/issues/357)) ([372b927](https://github.com/ucb-substrate/substrate2/commit/372b9275c9d9c5cd58603f5a462a5e4b66b64cf7))
* **bbox:** add bbox_rect method ([#373](https://github.com/ucb-substrate/substrate2/issues/373)) ([55b2632](https://github.com/ucb-substrate/substrate2/commit/55b2632a3c1e1ad260b61c6545143a2b16ef1150))
* **def:** utilities for exporting def orientations ([#434](https://github.com/ucb-substrate/substrate2/issues/434)) ([43a2b29](https://github.com/ucb-substrate/substrate2/commit/43a2b2906231cd46f08e2c4aface260d34abac62))
* **dirs:** add `Dirs` struct ([#371](https://github.com/ucb-substrate/substrate2/issues/371)) ([6d6b834](https://github.com/ucb-substrate/substrate2/commit/6d6b8347eea60ed1fccaed16623d146c3bd0727e))
* **geometry:** support for rectangular rings ([#408](https://github.com/ucb-substrate/substrate2/issues/408)) ([6fc0f36](https://github.com/ucb-substrate/substrate2/commit/6fc0f361f2215968f698281bfaf37d03d3ec131e))
* **grid:** add ATOLL LCM routing grid and layer stack definition ([#338](https://github.com/ucb-substrate/substrate2/issues/338)) ([1e1ad90](https://github.com/ucb-substrate/substrate2/commit/1e1ad90d02b50dd0dd15516c306971241bf30b7c))
* **layir:** initial LayIR implementation ([#456](https://github.com/ucb-substrate/substrate2/issues/456)) ([4f76d41](https://github.com/ucb-substrate/substrate2/commit/4f76d41c86fd0c57e525f40c976b5eeb0bbd4c68))
* **tests:** fix compilation and lint errors ([#482](https://github.com/ucb-substrate/substrate2/issues/482)) ([b55d04e](https://github.com/ucb-substrate/substrate2/commit/b55d04ecd2472f9f72b926ba5286f0d928bc2691))
* **tests:** reorganize tests and documentation ([#464](https://github.com/ucb-substrate/substrate2/issues/464)) ([928b9b7](https://github.com/ucb-substrate/substrate2/commit/928b9b7c45dc334ca11d86e4564edc58bf6db6f2))
* **transform:** default to Manhattan transformations ([#452](https://github.com/ucb-substrate/substrate2/issues/452)) ([3d8a410](https://github.com/ucb-substrate/substrate2/commit/3d8a4109febb11616d550c8cd6373e8f605b2e28))
* **transform:** make transformations use integers instead of floats ([#451](https://github.com/ucb-substrate/substrate2/issues/451)) ([aa9764e](https://github.com/ucb-substrate/substrate2/commit/aa9764e8b63b0a344d5e12ad3c678849c5c8ebea))
* **tutorial:** implement sky130 inverter layout tutorial ([#481](https://github.com/ucb-substrate/substrate2/issues/481)) ([440ab0e](https://github.com/ucb-substrate/substrate2/commit/440ab0e6ac33a8396c10f09637242efa32cfca62))


### Bug Fixes

* **deps:** bump rust to version 1.75.0 ([#362](https://github.com/ucb-substrate/substrate2/issues/362)) ([e1e82c9](https://github.com/ucb-substrate/substrate2/commit/e1e82c94cdf6ba4426f3f73f29dca40674a7f064))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * geometry_macros bumped from 0.0.1 to 0.0.2

## [0.5.0](https://github.com/ucb-substrate/substrate2/compare/geometry-v0.4.0...geometry-v0.5.0) (2023-11-02)


### Features

* **geometry:** implemented contains for polygon ([#292](https://github.com/ucb-substrate/substrate2/issues/292)) ([708053a](https://github.com/ucb-substrate/substrate2/commit/708053adfb9f3783fc03895ede7348ace51730f0))
* **netlists:** use consistent ordering via indexmap ([#266](https://github.com/ucb-substrate/substrate2/issues/266)) ([f275c19](https://github.com/ucb-substrate/substrate2/commit/f275c19396ed4f7d255836822ff72b808f89cde7)), closes [#265](https://github.com/ucb-substrate/substrate2/issues/265)
* **polygon:** polygon implemented in geometry ([#263](https://github.com/ucb-substrate/substrate2/issues/263)) ([4508570](https://github.com/ucb-substrate/substrate2/commit/45085706a30a12f4af6c5e3f642ca55b4c32dd24))


### Bug Fixes

* **deps:** update rust crate num-rational to 0.4 ([#294](https://github.com/ucb-substrate/substrate2/issues/294)) ([fc8f5ce](https://github.com/ucb-substrate/substrate2/commit/fc8f5ce9f35eb074acff45115e44ffbd37e0d237))

## [0.4.0](https://github.com/substrate-labs/substrate2/compare/geometry-v0.3.0...geometry-v0.4.0) (2023-08-08)


### Features

* **macros:** refactor macro reexports ([#250](https://github.com/substrate-labs/substrate2/issues/250)) ([a332717](https://github.com/substrate-labs/substrate2/commit/a332717e549fdea50306067e1c92dc60293aed4c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * geometry_macros bumped from 0.0.0 to 0.0.1

## [0.3.0](https://github.com/substrate-labs/substrate2/compare/geometry-v0.2.0...geometry-v0.3.0) (2023-07-23)


### Features

* **gds-import:** implement GDS to RawCell importer ([#196](https://github.com/substrate-labs/substrate2/issues/196)) ([fc37eeb](https://github.com/substrate-labs/substrate2/commit/fc37eeb6bac10779491b98bcadcc0eeaeb7d8ec5))
* **gds:** gds reexport test ([#199](https://github.com/substrate-labs/substrate2/issues/199)) ([93d3cd5](https://github.com/substrate-labs/substrate2/commit/93d3cd555c1cb4a76a8845f4401e98d327b5d674))
* **proc-macros:** derive macros for geometry traits ([#164](https://github.com/substrate-labs/substrate2/issues/164)) ([a86074a](https://github.com/substrate-labs/substrate2/commit/a86074a69b714b1be551ae00c775beb04c13f776))
* **tiling:** array and grid tiling API ([#201](https://github.com/substrate-labs/substrate2/issues/201)) ([b3b7c2b](https://github.com/substrate-labs/substrate2/commit/b3b7c2bfb7ba72198872d0f08ded3e0bc757479d))

## [0.2.0](https://github.com/substrate-labs/substrate2/compare/geometry-v0.1.0...geometry-v0.2.0) (2023-07-07)


### Features

* **blackboxing:** support Substrate and SCIR blackboxes ([#135](https://github.com/substrate-labs/substrate2/issues/135)) ([049a598](https://github.com/substrate-labs/substrate2/commit/049a598e2b8d11228c63f03dc878fc4c56e036a6))
* **tests:** add SCIR and SPICE netlister blackbox tests ([049a598](https://github.com/substrate-labs/substrate2/commit/049a598e2b8d11228c63f03dc878fc4c56e036a6))

## 0.1.0 (2023-06-13)


### Features

* **geometry:** initial geometry implementation ([#23](https://github.com/substrate-labs/substrate2/issues/23)) ([0361062](https://github.com/substrate-labs/substrate2/commit/036106213afa965c245acbd41874148f99fabdbb))
