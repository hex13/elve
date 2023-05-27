import { test, expect } from '@playwright/test';
const APP_URL = 'localhost:8000';
test('rectangle', async ({ page }) => {
    await page.goto(APP_URL);
    const rectangles = await page.getByRole('button', {name: 'rectangles'});
    await rectangles.click();

    const canvas = await page.locator('#game');
    const box = await canvas.boundingBox();
    await page.mouse.move(box.x, box.y);
    await page.mouse.down();
    await page.mouse.move(box.x + 200, box.y + 200);
    await page.mouse.up();
    await expect(canvas).toHaveScreenshot('rectangle.png');
});

test('drawing', async ({ page }) => {
    await page.goto(APP_URL);
    const drawing = await page.getByRole('button', {name: 'drawing'});
    await drawing.click();

    const input = await page.locator('input[type=color]');

    await input.fill('#ff0000');

    const canvas = await page.locator('#game');
    const box = await canvas.boundingBox();
    await page.mouse.move(box.x + 10, box.y + 10);
    await page.mouse.down();
    await page.mouse.move(box.x + 200, box.y + 200);
    await page.mouse.up();
    await expect(canvas).toHaveScreenshot('drawing.png');
});
