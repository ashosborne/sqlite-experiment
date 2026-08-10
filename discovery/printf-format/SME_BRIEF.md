# SME brief — printf-format (Phase A)
**Found:** 3 candidates over one shared formatter engine. %q/%Q/%w SQL-quoting is injection-safety-relevant — flag for security-minded characterization.
**Recommended binds:** accept 001 (SQL-visible) + 002; 003 if downstream uses the str API.
STOPPED for human bind. No Phase B performed.
