import { SecretVaultWrapper } from "nillion-sv-wrappers";
import { orgConfig } from "./nillionOrgConfig.js";

const SCHEMA_ID = "b1934ab0-b66b-4850-880f-9eacee621e2c";

function convert(vulnerabilities) {
  return vulnerabilities.map((v) => ({
    name: { $allot: v.name },
    severity: { $allot: v.severity },
    description: { $allot: v.description },
    location: { $allot: v.location },
    impacted_code: { $allot: v.impacted_code },
    recommendations: { $allot: v.recommendations },
  }));
}

async function main() {
  try {
    const input = process.argv[2];

    const v = JSON.parse(input);

    const data = [
      {
        vulnerabilities: convert(v),
      },
    ];

    const collection = new SecretVaultWrapper(
      orgConfig.nodes,
      orgConfig.orgCredentials,
      SCHEMA_ID
    );
    await collection.init();

    const dataWritten = await collection.writeToNodes(data);

    return dataWritten[0].result.data.created[0];
  } catch (error) {
    console.error("SecretVaultWrapper error:", error.message);
    process.exit(1);
  }
}

main().then((result) => {
  console.log(result); // Prints directly to stdout
});
