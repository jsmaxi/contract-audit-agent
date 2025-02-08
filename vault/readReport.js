import { SecretVaultWrapper } from "nillion-sv-wrappers";
import { orgConfig } from "./nillionOrgConfig.js";

const SCHEMA_ID = "b1934ab0-b66b-4850-880f-9eacee621e2c";

async function main() {
  try {
    const id = process.argv[2];

    const collection = new SecretVaultWrapper(
      orgConfig.nodes,
      orgConfig.orgCredentials,
      SCHEMA_ID
    );
    await collection.init();

    const decryptedCollectionData = await collection.readFromNodes({
      _id: id,
    });

    return decryptedCollectionData?.[0]?.vulnerabilities;
  } catch (error) {
    console.error("SecretVaultWrapper error:", error.message);
    process.exit(1);
  }
}

main().then((result) => {
  console.log(JSON.stringify(result)); // Prints directly to stdout
});
