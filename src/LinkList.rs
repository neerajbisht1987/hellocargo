use std::fmt;

#[derive(Clone,Debug)]
pub struct Information
{
    pub name:String,
    pub age:i32
}

// Implement `Display` for `MinMax`.
impl fmt::Display for Information {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
       // println!("({}, {})", self.name.as_str(), self.age);
        write!( f,"({}, {})", self.name.as_str(), self.age)
    }
}

#[derive(Clone)]
pub struct ListNode
{
    nodeInfo : Box<Information>,
    link : Option<Box<ListNode>>
}

pub struct LinkedList
{
     pub head : Option<Box<ListNode>> 
}

impl LinkedList
{   
    // pub fn new(&self)
    // {
    //    self.head=None;
    // } 
    pub fn add (&mut self,info:Information) 
    {
        match self.head 
        {
            None =>
            {
                self.head= Some(Box::new(ListNode{nodeInfo : Box::new(info) , link:None}));

            },
            Some(ref mut x) =>
            {
                let a = ListNode{ nodeInfo:Box::new(info), link:Some(Box::new(*x.clone()))};
                *x = Box::new(a);   
            }
        }        

    }
    pub fn traverse(&mut self)
    {
        let mut temp = &self.head;
        while temp.is_some()
        {    
           println!("{:?}\n",temp.as_ref().unwrap().nodeInfo );
            temp=&temp.as_ref().unwrap().link;
        }
    }
}