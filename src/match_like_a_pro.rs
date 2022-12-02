enum Activity
{
  Sleep(u8),
  Code,
  Ski(String),
}

// One can match tuples and thus match based on different kinds of state, nice 👍
pub fn run()
{
  let activity = Activity::Sleep(9);
  let weekend = true; // Can sleep until 10
  let powder = true; // No sleep, no friends

  let message = match (powder, weekend, activity)
  {
    (true, true, Activity::Sleep(_) | Activity::Code) => format!("Go Ski"),
    (_, true, Activity::Sleep(hours)) if hours > 10 => format!("Wake up after {} hours", hours),
    (_, false, Activity::Sleep(hours)) if hours > 8 => format!("Wake up after {} hours", hours),
    (_, _, Activity::Sleep(hours)) => format!("Sleep for {} hours", hours),
    (_, _, Activity::Ski(resort)) => format!("Ski in {}", resort),
    (_, _, Activity::Code) => format!("Code"),
  };

  println!("{}", message);
}
