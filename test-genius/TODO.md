## Test Generator Script

Easy:

[x] Create a new colocated test file
[] Name the new test file after the component
[] Import testing library
[] Import tested component
[] Import hooks used by the component
[] Name a test suite after the target component

Medium:

[] Set mock implementations in beforeAll
[] Mock all hooks used by the test
[] Import userEvent if needed

Hard:

[] Create one test per positive conditional render
[] Create one test per negative conditional render
[] Create two tests per ternary render
[] Create one test per trigger
[] Create one test per side effect
