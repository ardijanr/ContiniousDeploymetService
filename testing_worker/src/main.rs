use std::thread;

fn main() {
    println!("TESTING WORKER");
    println!("TESTING WORKING FOR 5 MINS");
    // thread::sleep_ms(5000);
    thread::sleep_ms(1000*60*2);

    println!("WORKER FINISHED PRINTING TEST DATA");

    println!("Nov 08 08:27:02 webserver web_interface[855]: Build succeeded.");
    println!("Nov 08 08:27:02 webserver web_interface[855]: Done. To undo this action, use 'ef migrations >");
    println!("Nov 08 08:27:02 webserver web_interface[855]: Removing intermediate container ba41da3fe57f'");
    println!("Nov 08 08:27:02 webserver web_interface[855]:  ---> 84b77a15c20f'");
    println!("Nov 08 08:27:02 webserver web_interface[855]: Step 16/18 : RUN mkdir ./Infrastructure/Data'");
    println!("Nov 08 08:27:02 webserver web_interface[855]:  ---> Running in 0394d396a56f'");
    println!("Nov 08 08:27:02 webserver web_interface[855]: mkdir: cannot create directory './Infrastructu>'");
    println!("Nov 08 08:27:02 webserver web_interface[855]:'");
    println!("Nov 08 08:27:02 webserver web_interface[855]: Successfully deployed new version'");
    println!("Nov 08 08:27:02 webserver web_interface[855]: Exit code: 0'");

}