import { test, expect } from "@playwright/test";

const backend = process.env.E2E_BACKEND ?? "http://127.0.0.1:8080";
const apiKey = process.env.E2E_API_KEY ?? "";

test.describe("Dashboard de contenedores", () => {
  test("se conecta al backend real o cae a mock", async ({ page, request }) => {
    let createdName: string | null = null;
    try {
      const health = await request.get(`${backend}/healthz`, {
        headers: apiKey ? { "x-api-key": apiKey } : {},
      });
      if (health.ok() && apiKey) {
        createdName = `e2e-${Date.now()}`;
        await request.post(`${backend}/api/containers`, {
          headers: {
            "x-api-key": apiKey,
            "content-type": "application/json",
          },
          data: { name: createdName },
        });
      }
    } catch {
      createdName = null;
    }

    if (!createdName) {
      await page.route("**/api/containers", async (route) => {
        await route.fulfill({
          status: 200,
          body: JSON.stringify([
            { id: "mock-1", name: "Demo Browser", status: "draft" },
          ]),
          headers: { "Content-Type": "application/json" },
        });
      });
    }

    await page.goto("/");
    await expect(page.getByText("Panel principal")).toBeVisible();

    if (createdName) {
      await expect(page.getByText(createdName)).toBeVisible();
    } else {
      await expect(page.getByText("Demo Browser")).toBeVisible();
    }
  });
});
