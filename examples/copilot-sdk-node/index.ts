import { CopilotClient } from "@github/copilot-sdk";

async function main() {
  const client = new CopilotClient();
  const session = await client.createSession({ model: "gemini-3-pro-preview", streaming: false });
  const response = await session.sendAndWait({ prompt: "Give a one-line summary of the TRAE CLI project README" });
  console.log("=== Copilot SDK example output ===\n");
  console.log(response?.data?.content ?? "(no response)");
  await client.stop();
  process.exit(0);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});