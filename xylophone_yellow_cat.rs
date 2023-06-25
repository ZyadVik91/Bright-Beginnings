//  This code implements a data structure that tracks children’s early development and records any changes in their development over time

use std::collections::HashMap;

struct Child {
   name: String,
   age: i32,
   milestones: HashMap<String, String>,
}

impl Child {
   fn new(name: &str, age: i32) -> Child {
       let mut milestones = HashMap::new();
       milestones.insert("Smiling".to_string(), "None".to_string());
       milestones.insert("Rolling Over".to_string(), "None".to_string());
       milestones.insert("Walking".to_string(), "None".to_string());
       milestones.insert("Talking".to_string(), "None".to_string());
       milestones.insert("Recognizing".to_string(), "None".to_string());

       Child {
           name: name.to_string(),
           age,
           milestones,
       }
   }

   //Method to record a milestone for a child
   fn record_milestone(&mut self, milestone: &str, date: &str) {
       self.milestones.insert(milestone.to_string(), date.to_string());
   }

   //Method to get a list of milestones achieved by a child
   fn get_milestones(&self) -> Vec<&str> {
       let mut result: Vec<&str> = vec![];

       for (k, v) in &self.milestones {
           if v != "None" {
               result.push(k.as_str());
           }
       }

       result
   }

}

struct ChildTracker {
    children: Vec<Child>,
}

impl ChildTracker {
    fn new() -> ChildTracker {
        ChildTracker {
            children: vec![],
        }
    }

    //Method to add a child to the tracker
    fn add_child(&mut self, name: &str, age: i32) {
        let c = Child::new(name, age);
        self.children.push(c);
    }

    //Method to remove a child from the tracker
    fn remove_child(&mut self, name: &str) {
        let mut index = None;
        for (i, c) in self.children.iter().enumerate() {
            if c.name == name {
                index = Some(i);
                break;
            }
        }

        if let Some(i) = index {
            self.children.remove(i);
        }
    }

    //Method to update a milestone for a child
    fn update_milestone(&mut self, name: &str, milestone: &str, date: &str) {
        for c in &mut self.children {
            if c.name == name {
                c.record_milestone(milestone, date);
                break;
            }
        }
    }

    //Method to get a list of achieved milestones for a child
    fn get_milestones(&self, name: &str) -> Vec<&str> {
        let mut result: Vec<&str> = vec![];

        for c in &self.children {
            if c.name == name {
                result = c.get_milestones();
                break;
            }
        }

        result
    }
}

fn main(){
    let mut tracker = ChildTracker::new();

    //Adding a child to the tracker
    tracker.add_child("John", 2);

    //Recording milestone achieved by the child
    tracker.update_milestone("John", "Smiling", "April 15, 2018");
    tracker.update_milestone("John", "Rolling Over", "May 04, 2018");

    let milestones = tracker.get_milestones("John");

    println!("{:?}", milestones); // Prints [Smiling, Rolling Over]
}