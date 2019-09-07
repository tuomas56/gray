use std::collections::{VecDeque, HashMap};
use std::rc::Rc;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct TaskID(usize);
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct GroupID(usize);

struct Group<R> {
    parent: TaskID,
    size: usize,
    results: Vec<R>
}

struct Executor<'a, T, R> {
    next_id: usize,
    groups: HashMap<GroupID, Group<R>>,
    tasks: HashMap<TaskID, (Option<GroupID>, T)>,
    run: VecDeque<(TaskID, Option<GroupID>)>,
    func: Rc<dyn Fn(&mut Self, TaskID, &T, Option<Vec<R>>) + 'a>
}

impl<'a, T: Clone, R> Executor<'a, T, R> {
    fn new<F: Fn(&mut Self, TaskID, &T, Option<Vec<R>>) + 'a>(func: F) -> Executor<'a, T, R> {
        Executor {
            next_id: 0,
            groups: HashMap::new(),
            tasks: HashMap::new(),
            run: VecDeque::new(),
            func: Rc::new(func)
        }
    }

    fn group(&mut self, parent: TaskID) -> GroupID {
        let id = GroupID(self.next_id);
        self.next_id += 1;
        self.groups.insert(id, Group { parent, size: 0, results: Vec::new() });
        id
    }

    fn spawn(&mut self, group: Option<GroupID>, task: T) -> TaskID {
        let id = TaskID(self.next_id);
        self.next_id += 1;
        if let Some(group_id) = group {
            self.groups.get_mut(&group_id).unwrap().size += 1;
        }
        self.tasks.insert(id, (group, task));
        self.run.push_back((id, None));
        id
    }

    fn done(&mut self, id: TaskID, result: R) {
        if let Some(group_id) = self.tasks[&id].0 {
            let (finished, parent) = {
                let group = self.groups.get_mut(&group_id).unwrap();
                group.results.push(result);
                (group.results.len() == group.size, group.parent)
            };

            if finished {
                self.run.push_back((parent, Some(group_id)));
            }
        }

        self.tasks.remove(&id);
    }

    fn modify(&mut self, id: TaskID) -> &mut T {
        &mut self.tasks.get_mut(&id).unwrap().1
    }

    fn go(&mut self) {
        while let Some((id, res)) = self.run.pop_front() {
            let res = res.map(|group_id| self.groups.remove(&group_id).unwrap().results);
            let task = self.tasks[&id].clone().1;
            self.func.clone()(self, id, &task, res);
        }
    }
}