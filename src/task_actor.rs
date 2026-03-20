//use async_trait::async_trait;
//use futures::Future;

use tokio::task::{self, JoinHandle, spawn_blocking, JoinError};
//use futures::{executor::block_on, FutureExt};
use std::{any::Any, marker::PhantomData, panic::{AssertUnwindSafe, UnwindSafe}, sync::Arc};

use act_rs::{ActorStateAsync, ActorStateBuilderAsync, ActorStateUnwindSafeAsync};

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

    pub fn spawn_and_build<ST, STB>(state_builder: STB) -> JoinHandle<()>
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
              F: AsyncFnOnce(Box<dyn Any + Send>) + Send + 'static
    {
        
        tokio::spawn(async move
        {
    
            let future = TaskActor::run_catch_unwind(state);

            let result = future.catch_unwind().await; //future.catch_unwind().await;

            if let Err(err) = result
            {

                err_fn(err).await;

            }

        })

    }

    async fn run<ST>(mut state: ST)
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

    async fn run_catch_unwind<ST>(mut state: ST)
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
