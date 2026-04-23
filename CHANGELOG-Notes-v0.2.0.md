commit 90cfcf115fd382775f4786188ab93a12a4ae6a9a -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Apr 22 16:11:42 2026 +1200

    - Updated the readme

commit 023fb3b78702fc93e98f2a2d4c04c71d822edd8d -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Apr 21 21:02:08 2026 +1200

    - Changed the package version to "0.2.0-beta".
    
    - The act_rs dependency version has been updated to "0.5.0".
    
    - The tokio dependency version has been updated to "1.52.1".
    
    - Added the task_actor_build_state_with_spawn, task_actor_build_state_with_spawn_flexible, task_actor_build_state_with_spawn_catch_unwind, task_actor_build_state_with_spawn_flexible, task_actor_build_state_and_catch_unwind, task_actor_build_state_with_spawn_catch_unwind, task_actor_catch_unwind_flexible, task_actor_build_state_and_catch_unwind_flexible and task_actor_build_state_with_spawn_catch_unwind_flexible test functions to the task_actor_macro_tests module.
    
    - Other minor changes
    
    - Added the impl_task_actor_build_state_with_spawn, impl_task_actor_build_state_with_spawn_flexible, impl_task_actor_build_state_with_spawn_catch_unwind,
    impl_task_actor_catch_unwind_flexible and impl_task_actor_build_state_with_spawn_catch_unwind_flexible macros.

commit 88f7e0601c386cbf7a235d1efbfea9de6c1cdc67 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Mon Apr 20 20:58:22 2026 +1200

    - Added pastey to the package dev-dependencies section.
    
    - Added the task_actor_macro_tests module with TestActorState, TestActorStateBuilder, TestActorFlowState, TestActorFlowStateBuilder, TestPaincHander structs, without_builder and with_builder functions and task_actor, task_actor_build_state, task_actor_flexible, task_actor_build_state_flexible and task_actor_catch_unwind test functions.
    
    - Other minor changes.

commit e2a04158592720dd890700f2acc37561fb7515af -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Mon Apr 20 14:41:05 2026 +1200

    - Renamed the mac_task_actors module to task_actor_macros.
    
    - Replaced the ActorStateUnwindSafeAsync trait bound with ActorStateAsync and UnwindSafe trait bounds in the spawn_catch_unwind and spawn_build_state_and_catch_unwind methods in the TaskActor implementation.
    
    - Replaced the ActorStateBuilderUnwindSafeAsync trait bound with the ActorStateBuilderAsync<ST> and UnwindSafe trait bounds in the spawn_build_state_and_catch_unwind method in the TaskActor implementation.

commit 27a3da91f0a4319adeee650f37daae42dc1a3a48
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Apr 17 19:07:46 2026 +1200

    - Updated and added some documentation.

commit 3f2e3c17af052f3083682306199cb87e13d7f9b2
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Apr 17 16:02:33 2026 +1200

    - Made the futures dependency optional.
    
    - Made the spawn_catch_unwind and spawn_build_state_and_catch_unwind methods of the TaskActor implementation dependant on the inclusion of the futures feature.

commit 8c6d21b5b90320047e5ca38b2fa7834fbbf40e7b
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Apr 8 14:57:35 2026 +1200

    - Replaced the FnOnce parameters with &Arc<Fn>s in the methods ending with “catch_unwind” in the BlockingActor implementation.
    
    - Moved the “proceed” bool declaration into the “if state.pre_run()” scope in the run method of the BlockingActor implementation.
    
    - Moved the “proceed” bool meta-declarations into the “if state.pre_run_async().await” scopes in the run meta-methods of the existing actor-generating macros.
    
    - Made the actor-generating macros that include “catch_unwind” in their names compatible with panic handling.
    
    - Added the impl_task_actor_catch_unwind_flexible and impl_task_actor_build_state_and_catch_unwind_flexible macros.
    
    - Made the TaskActor methods that include “catch_unwind” in their names compatible with panic handling.
    
    - Moved the “proceed” bool meta-declarations into the “if state.pre_run_async().await” and “if AssertUnwindSafe(state.pre_run_async()).await” scopes in the run and run_catch_unwind methods of the TaskActor implementation respectively.

commit 19e4350229e49532d1a43f890d12587481e4be4e
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Thu Mar 26 18:37:59 2026 +1300

    - Made the run static method public in the BlockingActor implementation.
    
    - Renamed the impl_mac_task_actor macro to impl_task_actor.
    
    - Updated documentation
    
    - Renamed the impl_mac_task_actor_with_state_builder to impl_task_actor_build_state.
    
    - Renamed the spawn_and_build meta-method definition to spawn_and_build_state in the impl_task_actor_build_state macro.
    
    - Renamed the impl_mac_task_actor_flexible macro to impl_task_actor_flexible.
    
    - Renamed the impl_mac_task_actor_with_state_builder_flexible macro to_task_actor_build_state_flexible.
    
    - Renamed the spawn_and_build meta-method definition to spawn_and_build_state in the to_task_actor_build_state_flexible macro.
    
    - Added the impl_task_actor_catch_unwind macro.
    
    - Added the impl_task_actor_build_state_and_catch_unwind macro.
    
    - Removed the bool parameter of the err_fn closure parameter of the spawn_build_state_and_catch_unwind method of the TaskActor implementation.

commit 862f910975d6668f443056cf22e6d66453b56f33
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Sat Mar 21 18:13:58 2026 +1300

    - Renamed the spawn_and_build method to spawn_and_build_state in the TaskActor implementation.
    
    - Went with FnOnce for handling panic results in spawn_catch_unwind.
    
    - Added the spawn_build_state_and_catch_unwind method to the TaskActor implementation.
    
    - Made the run method of the TaskActor implementation public.
    
    - Made the run_catch_unwind method of the TaskActor implementation public.

commit ac441e7aa1a17df71b351a24c15d026e2ceffae3
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Mar 20 19:25:12 2026 +1300

    - Added the futures dependency.
    
    - Added the spawn_catch_unwind and spawn_build_and_catch_unwind methods to the BlockingActor implementation.
    
    - Added the spawn_catch_unwind and run_catch_unwind methods to the TaskActor implementation.
    
    WIP

commit 38a1aa88d78e1bfed890147c06ccd38fd234c603
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Mon Mar 16 16:05:44 2026 +1300

    - Made the act_rs dependency point to a local repository.
    
    - Made async-trait an optional feature and updated the project to reflect this.
    
    - Removed the runtime_enter, runtime_enter_param, runtime_enter_param_ref, runtime_enter_param_mut, handle_enter, handle_enter_param, handle_enter_param_ref and handle_enter_param_mut functions and some old code and comments from the entering module.
    
    - Made the entering module private exposing only its contents.
    
    - Renamed the impl_mac_task_actor_built_state macro to impl_mac_task_actor_with_state_builder and changed its spawn meta-function to take a type which has an name that ends with “state”. Also updated its run function to deal with “state types” in same way the other actor oriented macros and structs do and the spawn_and_build meta-method was also added to this macro declaration.
    
    - Added the impl_mac_task_actor_flexible and impl_mac_task_actor_with_state_builder_flexible macros.

commit 95bc72d87db73390e8e9ce62cea70d85a3209080
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Sat Mar 14 18:08:49 2026 +1300

    - Updated the package version string to “0.2.0-alpha”.
    
    - Updated the act_rs dependency to point to a local repository.
    
    - Added a spawn_and_build method to the BlockingActor implementation.
    
    - Added a spawn_and_build method to the TaskActor implementation.
