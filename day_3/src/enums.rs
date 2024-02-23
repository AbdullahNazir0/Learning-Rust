#![allow(dead_code)]

#[derive(Debug)]
enum TaskStatus {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Debug)]
struct Task {
    title: String,
    description: String,
    status: TaskStatus,
}

fn create_task(titl: String, descript: String) -> Task {
    let task = Task {
        title: titl,
        description: descript,
        status: TaskStatus::NotStarted,
    };

    task
}

fn display_task(task: &Task) {
    println!("Task Title: {}", task.title);
    println!("Task Description: {}", task.description);
    println!("Task Status: {:?}", task.status);
}

fn update_task_status(task: &mut Task, task_status: TaskStatus) {
    task.status = task_status;
}

fn main() {
    let task1 = create_task("Task 1".to_owned(), "This is task 1".to_owned());
    let mut task2 = create_task("Task 2".to_owned(), "This is task 2".to_owned());

    display_task(&task1);
    display_task(&task2);

    let new_task_status = TaskStatus::InProgress;
    update_task_status(&mut task2, new_task_status);

    display_task(&task2);
}
