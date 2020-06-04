const URL: &str = "http://localhost";

pub async fn run() {
  // case 1(parse response by steps)
  match reqwest::get(URL).await {
    Ok(res) => {
      if res.status() == reqwest::StatusCode::OK {
        println!("response status is ok");

        match res.text().await {
          Ok(text) => println!("response text is\n\n{}", text),
          Err(e) => println!("No response text but error is\n\n{}", e),
        }
      } else {
        println!("response status is not ok");
      }
    }
    Err(e) => println!("error is {}", e),
  }

  // case 2(use shortcuts to parse response)
  let text = reqwest::get(URL)
    .await
    .expect("cannot get the url")
    .text()
    .await
    .expect("No response text");
  println!("response text is\n\n{}", text);
}
