struct Event{
    body:String, //event text
    tims:String, //date and time of the event
    status:i8, //0 -error 1 -warning 2 -info
}

struct Controller{
    name:String, //controller name
    events:Vec<Event>, //event collection (logs)
    filename:String, //log file name or path
}

fn main(){
    println!("empty body...exiting...");
}
