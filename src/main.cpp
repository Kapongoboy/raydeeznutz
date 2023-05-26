#include "raylib.h"

void check_user_key(Vector2 &position) {
        if (IsKeyDown(KEY_A)) {
            position.x -= 2.0f;
        }
        if (IsKeyDown(KEY_D)) {
            position.x += 2.0f;
        }
        if (IsKeyDown(KEY_S)) {
            position.y += 2.0f;
        }
        if (IsKeyDown(KEY_W)) {
            position.y -= 2.0f;
        }
}

void check_user_mouse(Vector2 &position, Color &ballColor) {
    if (IsMouseButtonPressed(MOUSE_BUTTON_LEFT)) {
        position = GetMousePosition();
        ballColor = MAROON;
    }
    else if (IsMouseButtonPressed(MOUSE_BUTTON_MIDDLE)) ballColor = LIME;
    else if (IsMouseButtonPressed(MOUSE_BUTTON_RIGHT)) ballColor = DARKBLUE;
    else if (IsMouseButtonPressed(MOUSE_BUTTON_SIDE)) ballColor = PURPLE;
    else if (IsMouseButtonPressed(MOUSE_BUTTON_EXTRA)) ballColor = YELLOW;
    else if (IsMouseButtonPressed(MOUSE_BUTTON_FORWARD)) ballColor = ORANGE;
    else if (IsMouseButtonPressed(MOUSE_BUTTON_BACK)) ballColor = BEIGE;
}

int main(void) {
    const int screenWidth = 800;
    const int screenHeight = 450;

    InitWindow(screenWidth, screenHeight, "Ordained v0.1.0");

    Vector2 ballPosition = {
        (float)screenWidth/2,
        (float)screenHeight/2
    };

    Color ballColor = DARKBLUE;

    SetTargetFPS(60);

    while (!WindowShouldClose()) {
        check_user_key(ballPosition);
        check_user_mouse(ballPosition, ballColor);

        BeginDrawing();
        ClearBackground(RAYWHITE);
        DrawText("Ordained the game for real this time", 10, 10, 20, DARKPURPLE);
        DrawCircleV(ballPosition, 50, ballColor);
        EndDrawing();
    }

    CloseWindow();

    return 0;
}
