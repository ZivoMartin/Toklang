#[allow(dead_code)]
pub struct Stack<T>{
    vec: Vec::<T>
}

#[allow(dead_code)]
impl<T> Stack<T>{

    pub fn new() -> Stack<T>{ 
        Stack{
            vec: Vec::new()
        }
    }

    pub fn init(first: T) -> Stack<T> {
        let mut res = Stack::new();
        res.push(first);
        res
    }

    // pub fn clone(&self) -> Stack<T> {
    //     Stack {
    //         vec: self.vec.clone()
    //     }
    // }

    pub fn clear(&mut self) {
        self.vec.clear()
    }
    
    pub fn push(&mut self, x: T){
        self.vec.push(x);
    }

    pub fn pop(&mut self) -> Option<T>{
        self.vec.pop()
    }

    pub fn val(&self) -> Option<&T>{
        self.vec.last()
    }

    pub fn val_mut(&mut self) -> Option<&mut T>{
        self.vec.last_mut()
    }

    pub fn is_empty(&self) -> bool{
        self.vec.len() == 0
    }

    pub fn size(&self) -> usize{
        self.vec.len()
    }

    pub fn change_top(&mut self, new: T) {
        *self.val_mut().unwrap() = new;
    }

} 

impl<T> Clone for Stack<T> where T: Clone {

    fn clone(&self) -> Stack<T> {
        Stack {
            vec: self.vec.clone()
        }
    }
    
}
