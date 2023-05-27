import { test, expect } from '@playwright/test';
const APP_URL = 'localhost:8000';

test('fireworks', async ({ page }) => {
    await page.goto(APP_URL);
    await page.getByRole('button', {name: 'fireworks'}).click();

    const canvas = await page.locator('#game');
    const box = await canvas.boundingBox();
    await page.mouse.move(box.x + 10, box.y + 10);
    await page.mouse.down();
    await page.mouse.move(box.x + 200, box.y + 200);
    await page.mouse.up();

    await page.mouse.move(box.x + 400, box.y + 400);
    await page.mouse.down();
    await page.mouse.move(box.x + 400, box.y + 350);
    await page.mouse.up();

    await page.waitForTimeout(300);

    // fireworks are random, so no assertion, only screenshot for documentation
    await canvas.screenshot({path: 'fireworks.png'});
});


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
