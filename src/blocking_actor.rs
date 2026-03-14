//use async_trait::async_trait;
//use futures::Future;

//use futures::{executor::block_on, FutureExt};
use tokio::task::JoinHandle;
use std::{marker::PhantomData, sync::Arc, panic::UnwindSafe};

use act_rs::{ActorState, ActorStateBuilder};

///
/// A blocking thread based actor.
/// 
pub struct BlockingActor
{
}

impl BlockingActor
{

    pub fn spawn<ST>(state: ST) -> JoinHandle<()>
        where ST: ActorState + Send + 'static
    {
        
        tokio::task::spawn_blocking(move ||
        {
    
            BlockingActor::run(state);

        })

    }

    pub fn spawn_and_build<ST, STB>(state_builder: STB) -> JoinHandle<()>
        where ST: ActorState + Send + 'static,
              STB: ActorStateBuilder<ST> + Send + 'static
    {
        
        tokio::task::spawn_blocking(move ||
        {

            if let Some(state) = state_builder.build()
            {

                BlockingActor::run(state);

            }      

        })

    }

    fn run<ST>(mut state: ST)
        where ST: ActorState + Send + 'static
    {

        let mut proceed = true; 
        
        if state.pre_run()
        {

            while proceed
            {
                
                proceed = state.run();
    
            }

        }

        state.post_run();

    }  
    
}
