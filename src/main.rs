use std::io;

fn main() {
    println!("Swim Celabrator🏊");
    loop {
        println!("Please input your distance");
        let mut distance = String::new();
        io::stdin()
            .read_line(&mut distance)
            .expect("Failed to read line");

        println!("Please input your stroke (breastroke/freestyle/backstroke/butterfly/IM)");
        let mut stroke = String::new();
        io::stdin()
            .read_line(&mut stroke)
            .expect("Failed to read line");
        let stroke: String = match stroke.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
        println!("Your event is {distance}{stroke}");

        let event = distance.clone() + &stroke;

        if stroke == "breastroke"{
            println!("Kick hard. explosive pull breastroke is all about the timing pull.kick.glide");
        }
        else if stroke == "backstroke"{
            println!("Kick short and snappy. powerful pull. head steady. max effort ");
        }
        else if stroke == "freestyle"{
            println!("Kick mad, fast, short and snappy. powerfull pull. head steady. don't breathe");

        }
        else if stroke == "butterfly"{
            println!("Kick from your chest. long breakout. rythmic pull. steady breathe pattern");
        }
        else if stroke == "IM"{
            println!("Rythmic fly. steady back. powerful breast. explosive free");
        }
        else{
            println!("Invalid stroke");
            break;
        }

        println!("Please input your PB");
        let mut pb = String::new();
        io::stdin()
            .read_line(&mut pb)
            .expect("Failed to read line");

        let pb: f32 = match pb.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("Your PB for {event} is {pb}");

        println!("Please input your time at the race");
        let mut time  = String::new();
        io::stdin()
            .read_line(&mut time)
            .expect("Failed to read line");

        let time: f32 = match time.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        
        if time == pb {
            println!("Congrats, try a teeny bit harder next time to smash your pb😊 .");
        }
        if time > pb {
            println!("Go back home and try again😣");
        }
        if time < pb {
            println!("You broke your PB, your time was {time}💯💯😀😀");
        }

        println!("Do you want to do it again(yes/no)");
        let mut repeat = String::new();
        io::stdin()
            .read_line(&mut repeat)
            .expect("Failed to read line");
        let repeat: String = match repeat.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        if repeat == "yes"{
            println!("🏊🏊Lets do it again");
            continue;
        }
        else{
            println!("👋Bye race well!");
            break;
        }
    }
}
