# Firewall-Checker Changelog

## Version 2.0.0
- firewall-checker support multi-threading
- By default, firewall-checker start in multi-threading mode
- Added the possibility to choose the number of workers (number of processes)

## Version 1.1.0
- Fix some bugs
- IPv6 is now supported
- Add security when an user give an output filename with special chars
- Results in the terminal are displayed by the smallest to the largest port number (before results was sorted randomly)
