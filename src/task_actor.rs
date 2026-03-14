//use async_trait::async_trait;
//use futures::Future;

use tokio::task::{self, JoinHandle, spawn_blocking, JoinError};
//use futures::{executor::block_on, FutureExt};
use std::{marker::PhantomData, sync::Arc, panic::UnwindSafe};

use act_rs::{ActorStateAsync, ActorStateBuilderAsync};

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

}
