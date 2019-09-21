#pragma STDC FENV_ACCESS ON
#include <fenv.h>

void round_upward() {
    fesetround(FE_UPWARD);
}

void round_restore() {
    fesetround(FE_TONEAREST);
}

int get_state() {
    return fegetround();
}