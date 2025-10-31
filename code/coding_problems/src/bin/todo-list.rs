use coding_problems::{input_with_exit, to_number};

enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

struct Task {
    description: String,
    status: TaskStatus,
}

impl Task {
    fn new(desc: &str) -> Task {
        Task {
            description: desc.to_string(),
            status: TaskStatus::Pending,
        }
    }

    fn start_task(&mut self) {
        match self.status {
            TaskStatus::Pending => self.status = TaskStatus::InProgress,
            _ => println!("Can only start a task which has not been started yet"),
        }
    }

    fn complete_task(&mut self) {
        match self.status {
            TaskStatus::InProgress => self.status = TaskStatus::Completed,
            _ => println!("Can only complete a task which is in progress!"),
        }
    }

    fn status(&self) -> &'static str {
        match self.status {
            TaskStatus::Pending => "Pending",
            TaskStatus::InProgress => "In progress",
            TaskStatus::Completed => "Completed",
        }
    }
}

fn get_task_id(list_len: usize) -> Option<usize> {
    let input_str = input_with_exit("Enter task serial number: ", false);
    let Some(input) = to_number(&input_str) else {
        return None;
    };

    let input = input as usize;

    if input > list_len || input <= 0 {
        None
    } else {
        Some(input - 1)
    }
}

fn wait_for_return() {
    let _ = input_with_exit("\nPress enter/return to return to main menu", false);
}

const MENU: &str = "┌───────────────────────┐\n\
                    │       TODO List       │\n\
                    │───────────────────────│\n\
                    │ 1. Add new task       │\n\
                    │ 2. See tasks status   │\n\
                    │ 3. Start a task       │\n\
                    │ 4. Complete a task    │\n\
                    └───────────────────────┘\n";

fn main() {
    println!("Enter choice or e to exit");
    let mut tasks_list: Vec<Task> = Vec::new();

    loop {
        clearscreen::clear().unwrap();
        println!("{}", MENU);

        let choice = input_with_exit("Enter choice or 'e' to exit: ", true);

        match choice.trim() {
            "1" => {
                let desc = input_with_exit(
                    "Enter task description:\n\
                    -> ",
                    false,
                );
                let new_task = Task::new(desc.trim());
                tasks_list.push(new_task);
            }

            "2" => {
                for (i, task) in tasks_list.iter().enumerate() {
                    println!(
                        "{serial_number}. {description}: {status}",
                        serial_number = i + 1,
                        description = task.description,
                        status = task.status()
                    );
                }
                wait_for_return();
            }

            "3" => {
                let Some(task_id) = get_task_id(tasks_list.len()) else {
                    println!("Enter a valid task id!");
                    wait_for_return();
                    continue;
                };
                tasks_list[task_id].start_task();
            }

            "4" => {
                let Some(task_id) = get_task_id(tasks_list.len()) else {
                    println!("Enter a valid task id!");
                    wait_for_return();
                    continue;
                };
                tasks_list[task_id].complete_task();
            }

            _ => {
                println!("Please enter a valid choice!");
                wait_for_return();
                continue;
            }
        }
    }
}
