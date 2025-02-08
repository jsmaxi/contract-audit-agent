import { SecretVaultWrapper } from "nillion-sv-wrappers";
import { orgConfig } from "./nillionOrgConfig.js";

const SCHEMA_ID = "b1934ab0-b66b-4850-880f-9eacee621e2c";

const data = [
  {
    vulnerabilities: [
      {
        name: { $allot: "Reentrancy Attack" },
        severity: { $allot: "high" },
        description: { $allot: "Some description" },
        // location: { $allot: "withdraw" },
        // impacted_code: { $allot: "test code" },
        // recommendations: { $allot: "prevent reentrancy attack" },
      },
      {
        name: { $allot: "Logic Error" },
        severity: { $allot: "low" },
        description: { $allot: "check corner cases" },
        location: { $allot: "init" },
        impacted_code: { $allot: "test code" },
        recommendations: { $allot: "add additional if check" },
      },
    ],
  },
];

async function main() {
  try {
    // Create a secret vault wrapper and initialize the SecretVault collection to use
    const collection = new SecretVaultWrapper(
      orgConfig.nodes,
      orgConfig.orgCredentials,
      SCHEMA_ID
    );
    await collection.init();

    // Write collection data to nodes encrypting the specified fields ahead of time
    const dataWritten = await collection.writeToNodes(data);
    console.log(
      "👀 Data written to nodes:",
      JSON.stringify(dataWritten, null, 2)
    );

    // Get the ids of the SecretVault records created
    const newIds = [
      ...new Set(dataWritten.map((item) => item.result.data.created).flat()),
    ];
    console.log("uploaded record ids:", newIds);

    // Read all collection data from the nodes, decrypting the specified fields
    const decryptedCollectionData = await collection.readFromNodes({
      //_id: "92114342-55bb-41a5-9ea3-38ce1e8c4a7d",
    });

    // Log first 5 records
    console.log(
      "Most recent records",
      decryptedCollectionData?.slice(0, data.length)
    );
  } catch (error) {
    console.error("❌ SecretVaultWrapper error:", error.message);
    process.exit(1);
  }
}

main();
