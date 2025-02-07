// call audit agent api:
// $ cd tools
// $ node chat.js

const server = "http://127.0.0.1";
const port = 8000;
const route = "chat";

const controller = new AbortController();
const timeoutSeconds = 300_000; // 5 minutes
const timeoutId = setTimeout(() => controller.abort(), timeoutSeconds);

const request = {
  text: "How can I start developing smart contracts",
  model: "gpt-4o-mini",
};

const postChat = (request) => {
  const url = `${server}:${port}/${route}`;

  const config = {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(request),
    signal: controller.signal,
  };
  console.log("sending...");

  fetch(url, config)
    .then(async (response) => {
      console.log(`Response Status: ${response.status}`);
      console.log(`Response Status Text: ${response.statusText}`);
      if (response.status !== 200) {
        return response.text().then((text) => {
          console.log(`Response: ${text}`);
        });
      } else {
        const data = await response.json();
        console.log("Response:", data);
        return data;
      }
    })
    .catch((error) => console.error("Error:", error));
};

postChat(request);
