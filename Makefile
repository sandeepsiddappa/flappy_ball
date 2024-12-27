PROJECT_NAME := flappy_ball
CARGO := cargo

# Default target
all: clean build run

# Build the project
build:
	$(CARGO) build

# Run the project
run:
	$(CARGO) run

# Test the project
test:
	$(CARGO) test

# Clean the project
clean:
	$(CARGO) clean

.PHONY: all build run test clean

