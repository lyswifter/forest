// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::{check_empty_params, SYSTEM_ACTOR_ADDR};

use ipld_blockstore::BlockStore;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use runtime::{ActorCode, Runtime};
use vm::{ActorError, ExitCode, MethodNum, Serialized, METHOD_CONSTRUCTOR};

/// Init actor methods available
#[derive(FromPrimitive)]
#[repr(u64)]
pub enum Method {
    Constructor = METHOD_CONSTRUCTOR,
}

impl Method {
    /// Converts a method number into a Method enum
    fn from_method_num(m: MethodNum) -> Option<Method> {
        FromPrimitive::from_u64(m)
    }
}

/// Init actor
pub struct Actor;
impl Actor {
    /// Init actor constructor
    pub fn constructor<BS, RT>(rt: &RT) -> Result<(), ActorError>
    where
        BS: BlockStore,
        RT: Runtime<BS>,
    {
        rt.validate_immediate_caller_is(std::iter::once(&*SYSTEM_ACTOR_ADDR))?;

        Ok(())
    }
}

impl ActorCode for Actor {
    fn invoke_method<BS, RT>(
        &self,
        rt: &mut RT,
        method: MethodNum,
        params: &Serialized,
    ) -> Result<Serialized, ActorError>
    where
        BS: BlockStore,
        RT: Runtime<BS>,
    {
        match Method::from_method_num(method) {
            Some(Method::Constructor) => {
                check_empty_params(params)?;
                Self::constructor(rt)?;
                Ok(Serialized::default())
            }
            _ => {
                // Method number does not match available, abort in runtime
                Err(rt.abort(ExitCode::SysErrInvalidMethod, "Invalid method".to_owned()))
            }
        }
    }
}