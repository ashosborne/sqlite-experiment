# engine-lookaside35-002 — pool behaviour + fallback

Confidence: observed-in-code (composed slice, run 45).
prepare traffic grows HIT (current always 0) and drains USED to 0; 64-byte slots force MISS_SIZE; 512x2 pool with six live statements forces MISS_FULL while USED positive; freed slot HITs again; resetFlag for USED (hi->cur) and HIT (clear).
Frozen scope = run-45 pinned cases (pack v35).
