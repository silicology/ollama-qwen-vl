use base64::{Engine as _, engine::general_purpose};
use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::images::Image;
use std::{fs, io::Write};

#[tokio::main]
async fn main() {
    // Create the Ollama client (pointing to local server)
    let ollama = Ollama::new("http://localhost".to_string(), 11434);

    // Read and encode the image to base64
    let image_bytes = fs::read("test.png").expect("Failed to read image file");
    let image_b64 = general_purpose::STANDARD.encode(image_bytes);
    let image = Image::from_base64(image_b64);

    // Prepare the request
    let mut req = GenerationRequest::new(
        "qwen2.5vl:latest".to_string(), // Model: qwen2.5vl:latest (https://ollama.com/library/qwen2.5vl), gemma3:27b
        "Do the ocr, its in hindi, extract name, fathers name, age and gender of all the boxes"
            .to_string(), // Prompt
    );
    req.images = vec![image]; // Attach the image
    req.think = Some(false);
    // Send the request
    // Send the request
    match ollama.generate(req).await {
        Ok(res) => {
            println!("{}", res.response);

            // Write the response to a text file
            let mut file =
                fs::File::create("qwen-hindi-output.txt").expect("Failed to create output file");
            file.write_all(res.response.as_bytes())
                .expect("Failed to write to output file");

            println!("Response written to qwen_output.txt");
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
