use celery::TaskResult;

#[celery::task]
fn add(x: i32, y: i32) -> TaskResult<i32> {
    Ok(x + y)
}

let my_app = celery::app!(
    broker = AMQP { std::env::var("AMQP_ADDR").unwrap() },
    tasks = [add],
    task_routes = [
        "*" => "celery",
    ],
);
