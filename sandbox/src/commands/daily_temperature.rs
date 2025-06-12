/**
 * Given an array of integers temperatures represents the daily temperatures, return an array answer such that answer[i] is the number of days you have to wait after the ith day to get a warmer temperature. If there is no future day for which this is possible, keep answer[i] == 0 instead.
 */

pub enum DailyTemperatureMode {
    //
    Stack,
    Reverse,
}

impl DailyTemperatureMode {
    fn from_str(s: &str) -> Self {
        match s {
            "Stack" => DailyTemperatureMode::Stack,
            "Reverse" => DailyTemperatureMode::Reverse,
            _ => panic!("Invalid mode"),
        }
    }
}

pub fn get_daily_temperature(temperatures: &Vec<i32>, mode: &String) -> Result<Vec<usize>, String> {
    match DailyTemperatureMode::from_str(&mode) {
        DailyTemperatureMode::Stack => {
            let mut stack: Vec<usize> = Vec::new();
            let mut result: Vec<usize> = vec![0; temperatures.len()];
            // iterate through all temperatures
            for (i, &temp) in temperatures.iter().enumerate() {
                // if the stack is not empty and the current temperature is greater than the temperature at the top of the stack
                while stack.len() > 0 && &temp > &temperatures[*stack.last().unwrap()] {
                    // calculate the difference between the current index and the index at the top of the stack
                    let index = stack.pop().unwrap();
                    result[index] = i - index;
                }
                // push the current index onto the stack
                stack.push(i);
            }
            Ok(result)
        }
        DailyTemperatureMode::Reverse => {
            let mut result: Vec<usize> = vec![0; temperatures.len()];
            // iterate through all temperatures in reverse order
            for i in (0..temperatures.len()).rev() {
                // create a pointer to the following temperature
                let mut current_pointer = i + 1;
                // as long as the pointer is within the bounds of the array
                while current_pointer < temperatures.len() {
                    // if the pointer temperature is greater than the temperature at the evaluating index
                    if temperatures[current_pointer] > temperatures[i] {
                        // calculate the difference between the current index and the index
                        result[i] = current_pointer - i;
                        break;
                    }
                    // if the result at the pointer is 0, it means there is no warmer temperature
                    if result[current_pointer] == 0 {
                        break;
                    }
                    current_pointer += 1;
                }
            }
            Ok(result)
        }
    }
}
