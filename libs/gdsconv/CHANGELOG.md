# Changelog

## [0.2.2](https://github.com/ucb-substrate/substrate2/compare/gdsconv-v0.2.1...gdsconv-v0.2.2) (2025-02-18)


### Features

* **gds:** convert GDS to generic layer type via FromGds trait ([#590](https://github.com/ucb-substrate/substrate2/issues/590)) ([1b98f28](https://github.com/ucb-substrate/substrate2/commit/1b98f289b4cd5b94f4248691b35bad8ec73b83c5))


### Bug Fixes

* **gds:** do not throw error on ports with missing labels ([#595](https://github.com/ucb-substrate/substrate2/issues/595)) ([014ef80](https://github.com/ucb-substrate/substrate2/commit/014ef80536c3e5a217da02344d44b3f524132105))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * layir bumped from 0.2.1 to 0.2.2
    * gds bumped from 0.4.1 to 0.4.2
    * geometry bumped from 0.7.1 to 0.7.2

## [0.2.1](https://github.com/ucb-substrate/substrate2/compare/gdsconv-v0.2.0...gdsconv-v0.2.1) (2025-01-24)


### Dependencies

* update dependencies ([0b87032](https://github.com/ucb-substrate/substrate2/commit/0b8703276631fbb19a958453394c981d6b092441))
* The following workspace dependencies were updated
  * dependencies
    * layir bumped from 0.2.0 to 0.2.1
    * gds bumped from 0.4.0 to 0.4.1
    * geometry bumped from 0.7.0 to 0.7.1

## [0.2.0](https://github.com/ucb-substrate/substrate2/compare/gdsconv-v0.1.0...gdsconv-v0.2.0) (2025-01-23)


### Features

* **ac:** implement Save for Substrate types in Spectre AC sim, fix lints ([#471](https://github.com/ucb-substrate/substrate2/issues/471)) ([9825520](https://github.com/ucb-substrate/substrate2/commit/98255207569cc00bd9ddc35419c2df1e48f1999c))
* **layir:** initial LayIR implementation ([#456](https://github.com/ucb-substrate/substrate2/issues/456)) ([4f76d41](https://github.com/ucb-substrate/substrate2/commit/4f76d41c86fd0c57e525f40c976b5eeb0bbd4c68))
* **layout:** simplified layout API, LayIR integration ([#459](https://github.com/ucb-substrate/substrate2/issues/459)) ([183d347](https://github.com/ucb-substrate/substrate2/commit/183d347c19e6fe98cf870be4716e7249f23bd423))
* **tests:** fix compilation and lint errors ([#482](https://github.com/ucb-substrate/substrate2/issues/482)) ([b55d04e](https://github.com/ucb-substrate/substrate2/commit/b55d04ecd2472f9f72b926ba5286f0d928bc2691))
* **tests:** reorganize tests and documentation ([#464](https://github.com/ucb-substrate/substrate2/issues/464)) ([928b9b7](https://github.com/ucb-substrate/substrate2/commit/928b9b7c45dc334ca11d86e4564edc58bf6db6f2))


### Bug Fixes

* **ci:** use head_ref instead of ref and fix gdsconv version ([#498](https://github.com/ucb-substrate/substrate2/issues/498)) ([bc5d66e](https://github.com/ucb-substrate/substrate2/commit/bc5d66e5aad82ea79436e2fb3ec33e960a58f7b6))
* **deps:** add missing `registry=substrate` for in-tree dependencies ([#517](https://github.com/ucb-substrate/substrate2/issues/517)) ([505d95c](https://github.com/ucb-substrate/substrate2/commit/505d95c17c5997166c1987cbc30e344fdd4c78fb))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * layir bumped from 0.1.0 to 0.2.0
    * gds bumped from 0.3.1 to 0.4.0
    * geometry bumped from 0.6.0 to 0.7.0

## 0.1.0 (2025-01-22)


### Features

* **ac:** implement Save for Substrate types in Spectre AC sim, fix lints ([#471](https://github.com/ucb-substrate/substrate2/issues/471)) ([9825520](https://github.com/ucb-substrate/substrate2/commit/98255207569cc00bd9ddc35419c2df1e48f1999c))
* **layir:** initial LayIR implementation ([#456](https://github.com/ucb-substrate/substrate2/issues/456)) ([4f76d41](https://github.com/ucb-substrate/substrate2/commit/4f76d41c86fd0c57e525f40c976b5eeb0bbd4c68))
* **layout:** simplified layout API, LayIR integration ([#459](https://github.com/ucb-substrate/substrate2/issues/459)) ([183d347](https://github.com/ucb-substrate/substrate2/commit/183d347c19e6fe98cf870be4716e7249f23bd423))
* **tests:** fix compilation and lint errors ([#482](https://github.com/ucb-substrate/substrate2/issues/482)) ([b55d04e](https://github.com/ucb-substrate/substrate2/commit/b55d04ecd2472f9f72b926ba5286f0d928bc2691))
* **tests:** reorganize tests and documentation ([#464](https://github.com/ucb-substrate/substrate2/issues/464)) ([928b9b7](https://github.com/ucb-substrate/substrate2/commit/928b9b7c45dc334ca11d86e4564edc58bf6db6f2))


### Bug Fixes

* **ci:** use head_ref instead of ref and fix gdsconv version ([#498](https://github.com/ucb-substrate/substrate2/issues/498)) ([bc5d66e](https://github.com/ucb-substrate/substrate2/commit/bc5d66e5aad82ea79436e2fb3ec33e960a58f7b6))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * layir bumped from 0.0.0 to 0.1.0
    * gds bumped from 0.3.0 to 0.3.1
    * geometry bumped from 0.5.0 to 0.6.0
