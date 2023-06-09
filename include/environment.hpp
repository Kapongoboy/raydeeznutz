#pragma once
#include <iostream>
#include <raylib.h>
#define NUM_BUILDINGS 100

struct Buildings {
    const int h;
    const int w;
    int spacing = 0;
    Rectangle buildings[NUM_BUILDINGS] = {0};
    Color building_colors[NUM_BUILDINGS] = {0};

    Buildings(const int h, const int w ): h(h), w(w) {
        for (int i = 0; i < NUM_BUILDINGS; i++) {
                buildings[i].width = (float)GetRandomValue(50, 200);
                buildings[i].height = (float)GetRandomValue(100, 800);
                buildings[i].y = h - 130.0f - buildings[i].height;
                buildings[i].x = -6000.0f + spacing;

                spacing += (int)buildings[i].width;

                building_colors[i] = (Color){
                    static_cast<unsigned char>(GetRandomValue(200, 240)),
                    static_cast<unsigned char>(GetRandomValue(200, 240)),
                    static_cast<unsigned char>(GetRandomValue(200, 240)),
                    255 
                };

            }
    }

    ~Buildings() {
        std::cout << "buildings memory freed" << "\n";
    }

    void draw_buildings() {
        DrawRectangle(-6000, 320, 13000, 8000, DARKGRAY); // grey base

        for (int i = 0; i < NUM_BUILDINGS; i++) {
            DrawRectangleRec(
                    buildings[i],
                    building_colors[i]
                    );
        }
    }
};
