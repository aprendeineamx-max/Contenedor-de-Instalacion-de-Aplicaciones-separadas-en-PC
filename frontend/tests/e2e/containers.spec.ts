import { test, expect } from "@playwright/test";

test.describe("Dashboard de contenedores", () => {
  test("muestra lista vacía y luego contenedores mockeados", async ({ page }) => {
    await page.route("**/api/containers", async (route) => {
      await route.fulfill({
        status: 200,
        body: JSON.stringify([
          { id: "demo-1", name: "Demo Browser", status: "draft" },
          { id: "demo-2", name: "Legacy App", status: "running" },
        ]),
        headers: { "Content-Type": "application/json" },
      });
    });

    await page.goto("/");

    await expect(page.getByText("Dashboard inicial")).toBeVisible();
    await expect(page.getByText("Demo Browser")).toBeVisible();
    await expect(page.getByText("Legacy App")).toBeVisible();
  });
});
