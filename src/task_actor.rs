//use async_trait::async_trait;
//use futures::Future;

use tokio::task::{self, JoinHandle, spawn_blocking, JoinError};
//use futures::{executor::block_on, FutureExt};
use std::{any::Any, marker::PhantomData, panic::{AssertUnwindSafe, UnwindSafe}, sync::Arc};

use act_rs::{ActorStateAsync, ActorStateBuilderAsync, ActorStateBuilderUnwindSafeAsync, ActorStateUnwindSafeAsync};

use futures::FutureExt;

///
/// A task based actor.
/// 
pub struct TaskActor
{
}

impl TaskActor
{

    pub fn spawn<ST>(state: ST) -> JoinHandle<()>
        where ST: ActorStateAsync + Send + 'static
    {
        
        tokio::spawn(async move
        {
    
            TaskActor::run(state).await;

        })

    }

    pub fn spawn_and_build_state<ST, STB>(state_builder: STB) -> JoinHandle<()>
        where ST: ActorStateAsync + Send + 'static,
              STB: ActorStateBuilderAsync<ST> + Send + 'static
    {
        
        tokio::spawn(async move
        {

            if let Some(state) = state_builder.build_async().await
            {

                TaskActor::run(state).await;

            }      

        })

    }

    pub fn spawn_catch_unwind<ST, F>(state: ST, err_fn: F) -> JoinHandle<()>
        where ST: ActorStateUnwindSafeAsync + Send + 'static, //+ UnwindSafe //ActorStateUnwindSafeAsync
              F: FnOnce(Box<dyn Any + Send>) + Send + 'static
    {
        
        tokio::spawn(async move
        {
    
            let future = TaskActor::run_catch_unwind(state);

            let result = future.catch_unwind().await; //future.catch_unwind().await;

            if let Err(err) = result
            {

                //Can't make AsyncFnOnce Send.

                err_fn(err);

            }

        })

    }

    pub fn spawn_build_state_and_catch_unwind<ST, STB, F>(state_builder: STB, err_fn: F) -> JoinHandle<()>
        where ST: ActorStateUnwindSafeAsync + Send + 'static,
              STB: ActorStateBuilderUnwindSafeAsync<ST> + Send + UnwindSafe + 'static,
              F: FnOnce(bool, Box<dyn Any + Send>) + Send + 'static
    {
        
        tokio::spawn(async move
        {

            match AssertUnwindSafe(state_builder.build_async()).catch_unwind().await
            {

                Ok(opt_state) =>
                {

                    if let Some(state) = opt_state
                    {

                        if let Err(err) = TaskActor::run_catch_unwind(state).catch_unwind().await
                        {

                            //bool: builder_error

                            err_fn(false, err);

                        }

                    }

                }
                Err(err) =>
                {

                    //bool: builder_error

                    err_fn(true, err);

                }
                
            }

        })

    }

    pub async fn run<ST>(mut state: ST)
        where ST: ActorStateAsync + Send + 'static
    {

        let mut proceed = true; 
        
        if state.pre_run_async().await
        {

            while proceed
            {
                
                proceed = state.run_async().await;
    
            }

        }
        
        state.post_run_async().await;

    }

    pub async fn run_catch_unwind<ST>(mut state: ST)
        where ST: ActorStateAsync + Send + UnwindSafe + 'static //ActorStateUnwindSafeAsync
    {

        let mut proceed = true; 
        
        if AssertUnwindSafe(state.pre_run_async()).await
        {

            while proceed
            {
                
                proceed = AssertUnwindSafe(state.run_async()).await;
    
            }

        }
        
        AssertUnwindSafe(state.post_run_async()).await;

    }

}
