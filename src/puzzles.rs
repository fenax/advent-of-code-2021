use std::fmt::*;



pub trait Data<T>{
    fn new(input:&str) ->Self;
    fn get_data(&self) -> &T;
 //   fn do_with_data (&self,f: &dyn Fn(&Self)){
 //       f(&self);
 //   }
}

pub trait Puzzle<'r,T>: Display
{
    fn new(input:&'r T) -> Self;
    fn resolve(&mut self);
}
