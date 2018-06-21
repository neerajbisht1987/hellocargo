#[derive(Clone)]
pub struct information
{
    pub name:String,
    pub age:i32
}

#[derive(Clone)]
pub struct ListNode
{
    nodeInfo : Box<information>,
    link : Option<Box<ListNode>>
}

pub struct LinkedList
{
     pub head : Option<ListNode> 
}

impl LinkedList
{   
    // pub fn new(&self)
    // {
    //    self.head=None;
    // } 
    pub fn add (&mut self,info:information) 
    {
        match self.head 
        {
            None =>
            {
                self.head= Some(ListNode{nodeInfo : Box::new(info) , link:None});

            },
            Some(ref mut x) =>
            {
                let a = ListNode{ nodeInfo:Box::new(info), link:Some(Box::new(x.clone())) };
                *x = a;   
            }
        }        

    }
    pub fn traverse(&self)
    {

    }
}