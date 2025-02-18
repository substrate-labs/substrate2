# Changelog

## [0.2.2](https://github.com/ucb-substrate/substrate2/compare/layir-v0.2.1...layir-v0.2.2) (2025-02-18)


### Features

* **gds:** convert GDS to generic layer type via FromGds trait ([#590](https://github.com/ucb-substrate/substrate2/issues/590)) ([1b98f28](https://github.com/ucb-substrate/substrate2/commit/1b98f289b4cd5b94f4248691b35bad8ec73b83c5))
* **gds:** support importing GDS libraries into sky130 ([#583](https://github.com/ucb-substrate/substrate2/issues/583)) ([5e3181b](https://github.com/ucb-substrate/substrate2/commit/5e3181b1307e32a017126028fc15a13255129195))
* **stdcells:** implement layout for sky130 stdcells ([#586](https://github.com/ucb-substrate/substrate2/issues/586)) ([6e438ec](https://github.com/ucb-substrate/substrate2/commit/6e438ecde6b092231b4f9b6f17e3004663c17f74))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * geometry bumped from 0.7.1 to 0.7.2

## [0.2.1](https://github.com/ucb-substrate/substrate2/compare/layir-v0.2.0...layir-v0.2.1) (2025-01-24)


### Dependencies

* update dependencies ([0b87032](https://github.com/ucb-substrate/substrate2/commit/0b8703276631fbb19a958453394c981d6b092441))
* The following workspace dependencies were updated
  * dependencies
    * geometry bumped from 0.7.0 to 0.7.1

## [0.2.0](https://github.com/ucb-substrate/substrate2/compare/layir-v0.1.0...layir-v0.2.0) (2025-01-23)


### Features

* **layir:** initial LayIR implementation ([#456](https://github.com/ucb-substrate/substrate2/issues/456)) ([4f76d41](https://github.com/ucb-substrate/substrate2/commit/4f76d41c86fd0c57e525f40c976b5eeb0bbd4c68))
* **layout:** simplified layout API, LayIR integration ([#459](https://github.com/ucb-substrate/substrate2/issues/459)) ([183d347](https://github.com/ucb-substrate/substrate2/commit/183d347c19e6fe98cf870be4716e7249f23bd423))
* **mos:** layout for sky130 1.8V nmos/pmos, fix geometry macros ([#478](https://github.com/ucb-substrate/substrate2/issues/478)) ([55f17b7](https://github.com/ucb-substrate/substrate2/commit/55f17b72ab90e12efb57d97fdad6b4e5373c30e2))
* **tests:** reorganize tests and documentation ([#464](https://github.com/ucb-substrate/substrate2/issues/464)) ([928b9b7](https://github.com/ucb-substrate/substrate2/commit/928b9b7c45dc334ca11d86e4564edc58bf6db6f2))


### Bug Fixes

* **ci:** fix ci for Substrate v2.1 ([#490](https://github.com/ucb-substrate/substrate2/issues/490)) ([cc09d71](https://github.com/ucb-substrate/substrate2/commit/cc09d7199b41fb2986d1d733aa3678db49464f70))
* **deps:** add missing `registry=substrate` for in-tree dependencies ([#517](https://github.com/ucb-substrate/substrate2/issues/517)) ([505d95c](https://github.com/ucb-substrate/substrate2/commit/505d95c17c5997166c1987cbc30e344fdd4c78fb))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * diagnostics bumped from 0.3.0 to 0.4.0
    * uniquify bumped from 0.3.0 to 0.4.0
    * enumify bumped from 0.1.1 to 0.2.0
    * geometry bumped from 0.6.0 to 0.7.0

## 0.1.0 (2025-01-22)


### Features

* **layir:** initial LayIR implementation ([#456](https://github.com/ucb-substrate/substrate2/issues/456)) ([4f76d41](https://github.com/ucb-substrate/substrate2/commit/4f76d41c86fd0c57e525f40c976b5eeb0bbd4c68))
* **layout:** simplified layout API, LayIR integration ([#459](https://github.com/ucb-substrate/substrate2/issues/459)) ([183d347](https://github.com/ucb-substrate/substrate2/commit/183d347c19e6fe98cf870be4716e7249f23bd423))
* **mos:** layout for sky130 1.8V nmos/pmos, fix geometry macros ([#478](https://github.com/ucb-substrate/substrate2/issues/478)) ([55f17b7](https://github.com/ucb-substrate/substrate2/commit/55f17b72ab90e12efb57d97fdad6b4e5373c30e2))
* **tests:** reorganize tests and documentation ([#464](https://github.com/ucb-substrate/substrate2/issues/464)) ([928b9b7](https://github.com/ucb-substrate/substrate2/commit/928b9b7c45dc334ca11d86e4564edc58bf6db6f2))


### Bug Fixes

* **ci:** fix ci for Substrate v2.1 ([#490](https://github.com/ucb-substrate/substrate2/issues/490)) ([cc09d71](https://github.com/ucb-substrate/substrate2/commit/cc09d7199b41fb2986d1d733aa3678db49464f70))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * uniquify bumped from 0.2.0 to 0.3.0
    * enumify bumped from 0.1.0 to 0.1.1
    * geometry bumped from 0.5.0 to 0.6.0
