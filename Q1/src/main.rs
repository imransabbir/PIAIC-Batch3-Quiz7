fn main() {
    crate::school::class::staff(); 
}

  mod school {
   pub mod class {
       pub fn staff (){
            println!("We guarantee the best staff to serve your child");
        }
    }
}
