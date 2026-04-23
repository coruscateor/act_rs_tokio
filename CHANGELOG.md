# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) (post version 0.2.0),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Version 0.2.0 (__/04/2026)

### Added



### Changed

commit 90cfcf115fd382775f4786188ab93a12a4ae6a9a

- Updated the readme

commit 023fb3b78702fc93e98f2a2d4c04c71d822edd8d

- Changed the package version to "0.2.0-beta".

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



### Removed



### Added

commit 023fb3b78702fc93e98f2a2d4c04c71d822edd8d

- Added the task_actor_build_state_with_spawn, task_actor_build_state_with_spawn_flexible, task_actor_build_state_with_spawn_catch_unwind, task_actor_build_state_with_spawn_flexible, task_actor_build_state_and_catch_unwind, task_actor_build_state_with_spawn_catch_unwind, task_actor_catch_unwind_flexible, task_actor_build_state_and_catch_unwind_flexible and task_actor_build_state_with_spawn_catch_unwind_flexible test functions to the task_actor_macro_tests module.

- Added the impl_task_actor_build_state_with_spawn, impl_task_actor_build_state_with_spawn_flexible, impl_task_actor_build_state_with_spawn_catch_unwind,
impl_task_actor_catch_unwind_flexible and impl_task_actor_build_state_with_spawn_catch_unwind_flexible macros.

commit 88f7e0601c386cbf7a235d1efbfea9de6c1cdc67

 - Added pastey to the package dev-dependencies section.

- Added the task_actor_macro_tests module with TestActorState, TestActorStateBuilder, TestActorFlowState, TestActorFlowStateBuilder, TestPaincHander structs, without_builder and with_builder functions and task_actor, task_actor_build_state, task_actor_flexible, task_actor_build_state_flexible and task_actor_catch_unwind test functions.



### Fixed



### Security



## Version 0.1.0 (08/08/2025)

- Initial release
