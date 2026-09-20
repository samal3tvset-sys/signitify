# ==============================
# SIGNITIFY V10 BUILD SYSTEM
# ==============================

APP_NAME = signitify

RUST_TARGET = target/debug/$(APP_NAME)

C_SRC = src/ffi/bridge.c
C_OUT = target/bridge.o

CC = gcc
CFLAGS = -Wall -Wextra -O2

LIBS = -limobiledevice-1.0 -lplist -lpthread

# ==============================
# DEFAULT TARGET
# ==============================

all: build-rust build-c link

# ==============================
# RUST BUILD
# ==============================

build-rust:
	cargo build

# ==============================
# C BRIDGE BUILD
# ==============================

build-c:
	@mkdir -p target
	$(CC) $(CFLAGS) -c $(C_SRC) -o $(C_OUT)

# ==============================
# LINK (optional standalone test)
# ==============================

link:
	@echo "C bridge compiled at $(C_OUT)"

# ==============================
# RUN APP
# ==============================

run:
	cargo run

# ==============================
# CLEAN
# ==============================

clean:
	cargo clean
	rm -rf target/*.o

# ==============================
# TEST C ONLY
# ==============================

test-c:
	$(CC) $(CFLAGS) $(C_SRC) -o target/test_bridge $(LIBS)
	@echo "Run: ./target/test_bridge"

# ==============================
# FULL REBUILD
# ==============================

rebuild: clean all