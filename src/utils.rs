use crate::digits::*;

const SECONDS: usize = 1;
const MINUTES: usize = 2;
const HOURS: usize = 3;

// Format of a timer is HH:MM:SS or MM:SS or SS
pub fn to_seconds(timer: String) -> Result<usize, String> {
    let values: Vec<usize> = timer
        .split(":")
        .map(|value| value.parse::<usize>().except("Failed to parse string as number"))
        .collect();

    // Depending on the length of the vector, we convert a timer to seconds
    match values.len() {
        SECONDS => Ok(values[0]),
        MINUTES => Ok(values[0] * 60 + values[1]),
        HOURS => Ok(values[0] * 3600 + values[1] * 60 + values[2]),
        _ => Err(format!(
            "Expected format of 'HH:MM:SS', got {}",
            timer
        )),
    }
}

// Convert seconds to hours
pub fn to_hours(seconds: usize) -> Vec<Box<dyn Digit>> {
    let hours = seconds / 3600;
    let minutes = (seconds - hours * 3600) / 60;
    let remainder_seconds = seconds - hours * 3600 - minutes * 60;

    // Formatting the timer correctly as HH:MM:SS
    // We also pad single digits with a 0 to provide symmetry
    let a = String::from(format!(
        "{:0>2}:{:0>2}:{:0>2}",
        hours, minutes, remainder_seconds
    ));

    let mut objects: Vec<Box<dyn Digit>> = Vec::new();
    for c in a.chars() {
        match c {
            '0' => objects.push(Box::new(Zero)),
            '1' => objects.push(Box::new(One)),
            '2' => objects.push(Box::new(Two)),
            '3' => objects.push(Box::new(Three)),
            '4' => objects.push(Box::new(Four)),
            '5' => objects.push(Box::new(Five)),
            '6' => objects.push(Box::new(Six)),
            '7' => objects.push(Box::new(Seven)),
            '8' => objects.push(Box::new(Eight)),
            '9' => objects.push(Box::new(Nine)),
            ':' => {}
            _ => eprintln!("Something went wrong"),
        }
    }

    objects
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_to_seconds_normal() {
        let seconds = to_seconds("3:00:00".to_string()).unwrap();
        assert_eq!(seconds, 10_800);
    }

    #[test]
    #[should_panic]
    fn test_alphabet() {
        to_seconds("AA".to_string()).unwrap();
    }

}

