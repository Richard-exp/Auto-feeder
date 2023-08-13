use reqwest;

pub fn make_request() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://httpbin.org/ip")?.text()?;
    println!("{}", resp);
    Ok(())
}
