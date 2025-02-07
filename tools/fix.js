// call audit agent api:
// $ cd tools
// $ node fix.js

const server = "http://127.0.0.1";
const port = 8000;
const route = "fix";

const controller = new AbortController();
const timeoutSeconds = 300_000; // 5 minutes
const timeoutId = setTimeout(() => controller.abort(), timeoutSeconds);

const vulnerability1 = {
  name: "Reentrancy",
  severity: "high",
  description: "State changes after external call create reentrancy risk",
  location: "withdraw() function",
  impacted_code: "balances and call",
  recommendations: "Prevent reentrancy risk",
};

const vulnerability2 = {
  name: "Integer Overflow",
  severity: "high",
  description: "Unsigned integer arithmetic may overflow",
  location: "balances mapping",
  impacted_code: "balances and call",
  recommendations: "Prevent integer arithmetic overflow",
};

const request = {
  contract_code: `
    contract VulnerableContract {
        mapping(address => uint256) public balances;
        function withdraw() public {
            uint256 amount = balances[msg.sender];
            (bool success, ) = msg.sender.call{value: amount}("");
            balances[msg.sender] = 0;
        }
    }
    `,
  language: "Solidity",
  model: "gpt-4o-mini",
  vulnerabilities: [vulnerability1, vulnerability2],
};

const postFix = (request) => {
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

postFix(request);
