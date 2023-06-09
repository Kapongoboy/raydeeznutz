#pragma once
#include "raylib.h"

#define MAX_FRAME_SPEED 15
#define MIN_FRAME_SPEED 1

class Character {
public:
    Texture2D texture;
    Vector2 position;
    Rectangle frameRec;
    int currentFrame;
    int framesCounter;
    int framesSpeed;
public:
    Character();
    ~Character();

};
