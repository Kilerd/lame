#include <stdio.h>
#include "lame/include/lame.h"

int main() {
    lame_global_flags *gfp;

    printf("Testing LAME AVX2 support...\n\n");

    gfp = lame_init();
    if (gfp == NULL) {
        fprintf(stderr, "Failed to initialize LAME\n");
        return 1;
    }

    // Set basic parameters
    lame_set_in_samplerate(gfp, 44100);
    lame_set_num_channels(gfp, 1);
    lame_set_brate(gfp, 192);
    lame_set_quality(gfp, 4);

    // This will trigger CPU feature detection and print CPU info
    if (lame_init_params(gfp) < 0) {
        fprintf(stderr, "Failed to initialize parameters\n");
        lame_close(gfp);
        return 1;
    }

    printf("\nLAME initialized successfully!\n");
    printf("Check the output above for CPU features line.\n");

    lame_close(gfp);
    return 0;
}
