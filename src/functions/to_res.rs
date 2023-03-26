use crate::traits::*;
use std::error::Error;

pub fn to_res<R: ResTrait>(res: Result<R, Box<dyn Error>>) -> R
where
    <R as ResTrait>::Error: 'static,
{
    match res {
        Ok(res) => res,
        Err(e) => {
            let e = match e.downcast::<R::Error>() {
                Ok(downcasted) => *downcasted,
                Err(e) => R::Error::external(e)
            };

            R::error(e)
        }
        // Err(e) => GMResponses::Error {
        //     kind: match e.downcast::<GMError>() {
        //         Ok(downcasted) => *downcasted,
        //         Err(e) => GMError::External(e.to_string()),
        //     },
        // },
    }
}
