import("stdfaust.lib");

freq = hslider("freq", 440, 25, 11000, 0.1);

process = os.osc(freq);
