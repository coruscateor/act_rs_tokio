# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) (post version 0.2.0),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Version 0.2.0 (__/04/2026)

### Added

commit 27a3da91f0a4319adeee650f37daae42dc1a3a48

-- Updated and added some documentation.

- Added some documentation.

commit 19e4350229e49532d1a43f890d12587481e4be4e

- Added the impl_task_actor_catch_unwind macro.

- Added the impl_task_actor_build_state_and_catch_unwind macro.

commit 862f910975d6668f443056cf22e6d66453b56f33

- Added the spawn_build_state_and_catch_unwind method to the TaskActor implementation.

commit ac441e7aa1a17df71b351a24c15d026e2ceffae3

-- Added the futures dependency.

- Added the futures optional dependency.

-- Added the spawn_catch_unwind and spawn_build_and_catch_unwind methods to the BlockingActor implementation.

- Added the spawn_catch_unwind and spawn_build_state_and_catch_unwind methods to the BlockingActor implementation.

Renamed - Added in this version.

- Added the spawn_catch_unwind and run_catch_unwind methods to the TaskActor implementation.

commit 38a1aa88d78e1bfed890147c06ccd38fd234c603

- Added the impl_mac_task_actor_flexible and impl_mac_task_actor_with_state_builder_flexible macros.



### Changed

commit 90cfcf115fd382775f4786188ab93a12a4ae6a9a

- Updated the readme

commit 023fb3b78702fc93e98f2a2d4c04c71d822edd8d

-- Changed the package version to "0.2.0-beta".

- The act_rs dependency version has been updated to "0.5.0".

- The tokio dependency version has been updated to "1.52.1".

-- Other minor changes

?

commit 88f7e0601c386cbf7a235d1efbfea9de6c1cdc67

-- Other minor changes.

?

commit e2a04158592720dd890700f2acc37561fb7515af

- Renamed the mac_task_actors module to task_actor_macros.

-- Replaced the ActorStateUnwindSafeAsync trait bound with ActorStateAsync and UnwindSafe trait bounds in the spawn_catch_unwind and spawn_build_state_and_catch_unwind methods in the TaskActor implementation.

Added in this version.

-- Replaced the ActorStateBuilderUnwindSafeAsync trait bound with the ActorStateBuilderAsync<ST> and UnwindSafe trait bounds in the spawn_build_state_and_catch_unwind method in the TaskActor implementation.

Added in this version.

commit 27a3da91f0a4319adeee650f37daae42dc1a3a48

-- Updated and added some documentation.

- Updated some documentation.

commit 3f2e3c17af052f3083682306199cb87e13d7f9b2

-- Made the futures dependency optional.

Added in this version.

-- Made the spawn_catch_unwind and spawn_build_state_and_catch_unwind methods of the TaskActor implementation dependant on the inclusion of the futures feature.

Added in this version.

commit 8c6d21b5b90320047e5ca38b2fa7834fbbf40e7b

-- Replaced the FnOnce parameters with &Arc<Fn>s in the methods ending with “catch_unwind” in the BlockingActor implementation.

Added in this version.

- Moved the “proceed” bool declaration into the “if state.pre_run()” scope in the run method of the BlockingActor implementation.

- Moved the “proceed” bool meta-declarations into the “if state.pre_run_async().await” scopes in the run meta-methods of the existing actor-generating macros.

-- Made the actor-generating macros that include “catch_unwind” in their names compatible with panic handling.

Added in this version.

-- Made the TaskActor methods that include “catch_unwind” in their names compatible with panic handling.

Added in this version.

-- Moved the “proceed” bool meta-declarations into the “if state.pre_run_async().await” and “if AssertUnwindSafe(state.pre_run_async()).await” scopes in the run and run_catch_unwind methods of the TaskActor implementation respectively.

- Moved the “proceed” bool meta-declarations into the “if state.pre_run_async().await” and “if AssertUnwindSafe(state.pre_run_async()).await” scopes in the run methods of the TaskActor implementation.

commit 19e4350229e49532d1a43f890d12587481e4be4e

- Made the run static method public in the BlockingActor implementation.

- Renamed the impl_mac_task_actor macro to impl_task_actor.

- Updated documentation

-- Renamed the impl_mac_task_actor_with_state_builder to impl_task_actor_build_state.

Renamed from impl_mac_task_actor_built_state macro

- Renamed the spawn_and_build meta-method definition to spawn_and_build_state in the impl_task_actor_build_state macro.

- Renamed the impl_mac_task_actor_flexible macro to impl_task_actor_flexible.

Renamed - Added in this version.

-- Renamed the impl_mac_task_actor_with_state_builder_flexible macro to_task_actor_build_state_flexible.

Renamed - Added in this version.

-- Renamed the spawn_and_build meta-method definition to spawn_and_build_state in the to_task_actor_build_state_flexible macro.

Renamed - Added in this version.

-- Removed the bool parameter of the err_fn closure parameter of the spawn_build_state_and_catch_unwind method of the TaskActor implementation.

Added in this version.

commit 862f910975d6668f443056cf22e6d66453b56f33

-- Renamed the spawn_and_build method to spawn_and_build_state in the TaskActor implementation.

Renamed - Added in this version.

-- Went with FnOnce for handling panic results in spawn_catch_unwind.

Added in this version.

- Made the run method of the TaskActor implementation public.

-- Made the run_catch_unwind method of the TaskActor implementation public.

Added in this version.

commit 38a1aa88d78e1bfed890147c06ccd38fd234c603

-- Made the act_rs dependency point to a local repository.

-- Made async-trait an optional feature and updated the project to reflect this.

Added in this version.

-- Made the entering module private exposing only its contents.

- Made the entering module private.

-- Renamed the impl_mac_task_actor_built_state macro to impl_mac_task_actor_with_state_builder and changed its spawn meta-function to take a type which has an name that ends with “state”. Also updated its run function to deal with “state types” in same way the other actor oriented macros and structs do and the spawn_and_build meta-method was also added to this macro declaration.

Renamed - Added in this version.

commit 95bc72d87db73390e8e9ce62cea70d85a3209080

-- Updated the package version string to “0.2.0-alpha”.

-- Updated the act_rs dependency to point to a local repository.



### Removed

commit 38a1aa88d78e1bfed890147c06ccd38fd234c603

- Removed the runtime_enter, runtime_enter_param, runtime_enter_param_ref, runtime_enter_param_mut, handle_enter, handle_enter_param, handle_enter_param_ref and handle_enter_param_mut functions and some old code and comments from the entering module.

### Added

commit 023fb3b78702fc93e98f2a2d4c04c71d822edd8d

- Added the task_actor_build_state_with_spawn, task_actor_build_state_with_spawn_flexible, task_actor_build_state_with_spawn_catch_unwind, task_actor_build_state_with_spawn_flexible, task_actor_build_state_and_catch_unwind, task_actor_build_state_with_spawn_catch_unwind, task_actor_catch_unwind_flexible, task_actor_build_state_and_catch_unwind_flexible and task_actor_build_state_with_spawn_catch_unwind_flexible test functions to the task_actor_macro_tests module.

- Added the impl_task_actor_build_state_with_spawn, impl_task_actor_build_state_with_spawn_flexible, impl_task_actor_build_state_with_spawn_catch_unwind,
impl_task_actor_catch_unwind_flexible and impl_task_actor_build_state_with_spawn_catch_unwind_flexible macros.

commit 88f7e0601c386cbf7a235d1efbfea9de6c1cdc67

-- Added pastey to the package dev-dependencies section.

- Added pastey to the package dev-dependencies section of the cargo.toml file.

- Added the task_actor_macro_tests module with TestActorState, TestActorStateBuilder, TestActorFlowState, TestActorFlowStateBuilder, TestPaincHander structs, without_builder and with_builder functions and task_actor, task_actor_build_state, task_actor_flexible, task_actor_build_state_flexible and task_actor_catch_unwind test functions.

commit 95bc72d87db73390e8e9ce62cea70d85a3209080

-- Added a spawn_and_build method to the BlockingActor implementation.

- Added the spawn_and_build_state method to the BlockingActor implementation.

Renamed - Added in this version.

-- Added a spawn_and_build method to the TaskActor implementation.

- Added the spawn_and_build_state method to the TaskActor implementation.

Renamed - Added in this version.



### Fixed



### Security



## Version 0.1.0 (08/08/2025)

- Initial release
