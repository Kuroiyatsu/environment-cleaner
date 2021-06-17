use powershell_script;

fn main() {
    let docker_ping = "docker ps";

    docker_ps(docker_ping);

}

fn docker_ps(command: &str) {
    match powershell_script::run(command, true){
        Ok(output) => {
            println!("{}", output);
        }
        Err(e) =>{
            println!("Error: {}", e);
        }
    }
}
